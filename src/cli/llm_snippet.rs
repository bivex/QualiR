use crate::domain::smell::SourceLocation;

pub(crate) fn source_snippet(location: &SourceLocation) -> Option<String> {
    let source = std::fs::read_to_string(&location.file).ok()?;
    let start = location.line_start.max(1);
    let end = location.line_end.max(start);
    let mut snippet = String::new();

    for (index, line) in source.lines().enumerate() {
        let line_number = index + 1;
        if line_number < start {
            continue;
        }
        if line_number > end {
            break;
        }
        if !snippet.is_empty() {
            snippet.push('\n');
        }
        snippet.push_str(line);
    }

    (!snippet.is_empty()).then_some(snippet)
}

pub(crate) fn source_snippet_with_context(
    location: &SourceLocation,
    context_lines: usize,
) -> Option<String> {
    use std::fmt::Write as _;

    const MAX_CONTEXT_SNIPPET_LINES: usize = 32;

    let source = std::fs::read_to_string(&location.file).ok()?;
    let lines: Vec<_> = source.lines().collect();
    if lines.is_empty() || location.line_start == 0 || location.line_start > lines.len() {
        return None;
    }

    let finding_start = location.line_start;
    let finding_end = location.line_end.max(finding_start).min(lines.len());
    let snippet_start = finding_start.saturating_sub(context_lines).max(1);
    let snippet_end = finding_end.saturating_add(context_lines).min(lines.len());
    let line_number_width = snippet_end.to_string().len();
    let mut snippet = String::new();
    let total_snippet_lines = snippet_end - snippet_start + 1;

    if total_snippet_lines <= MAX_CONTEXT_SNIPPET_LINES {
        for line_number in snippet_start..=snippet_end {
            write_context_line(
                &mut snippet,
                &lines,
                line_number,
                line_number_width,
                finding_start,
                finding_end,
            )?;
        }
    } else {
        let leading_lines = MAX_CONTEXT_SNIPPET_LINES / 2;
        let trailing_lines = MAX_CONTEXT_SNIPPET_LINES - leading_lines;
        let leading_end = snippet_start + leading_lines - 1;
        let trailing_start = snippet_end - trailing_lines + 1;

        for line_number in snippet_start..=leading_end {
            write_context_line(
                &mut snippet,
                &lines,
                line_number,
                line_number_width,
                finding_start,
                finding_end,
            )?;
        }
        writeln!(snippet, "  {:>line_number_width$} | ...", "...").ok()?;
        for line_number in trailing_start..=snippet_end {
            write_context_line(
                &mut snippet,
                &lines,
                line_number,
                line_number_width,
                finding_start,
                finding_end,
            )?;
        }
    }

    Some(snippet)
}

fn write_context_line(
    snippet: &mut String,
    lines: &[&str],
    line_number: usize,
    line_number_width: usize,
    finding_start: usize,
    finding_end: usize,
) -> Option<()> {
    use std::fmt::Write as _;

    let marker = if (finding_start..=finding_end).contains(&line_number) {
        ">"
    } else {
        " "
    };
    let source_line = lines[line_number - 1];
    writeln!(
        snippet,
        "{marker} {line_number:>line_number_width$} | {source_line}"
    )
    .ok()
}

pub(crate) fn print_fenced_code(language: &str, code: &str) {
    let fence = if code.contains("```") { "````" } else { "```" };
    println!("{fence}{language}");
    print!("{code}");
    if !code.ends_with('\n') {
        println!();
    }
    println!("{fence}");
}

#[cfg(test)]
mod tests {
    use crate::domain::smell::SourceLocation;

    use super::*;

    #[test]
    fn source_snippet_extracts_exact_line_range() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("sample.rs");
        std::fs::write(
            &path,
            "fn first() {}\nfn target() {\n    work();\n}\nfn last() {}\n",
        )
        .expect("write sample source");

        let location = SourceLocation::new(path, 2, 4, None);

        assert_eq!(
            source_snippet(&location).as_deref(),
            Some("fn target() {\n    work();\n}")
        );
    }

    #[test]
    fn source_snippet_with_context_marks_finding_lines() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("sample.rs");
        std::fs::write(
            &path,
            "fn first() {}\nfn target() {\n    work();\n}\nfn last() {}\n",
        )
        .expect("write sample source");

        let location = SourceLocation::new(path, 2, 3, None);

        assert_eq!(
            source_snippet_with_context(&location, 1).as_deref(),
            Some("  1 | fn first() {}\n> 2 | fn target() {\n> 3 |     work();\n  4 | }\n")
        );
    }

    #[test]
    fn source_snippet_with_context_elides_large_ranges() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("large.rs");
        let source = (1..=40)
            .map(|line| format!("line {line}\n"))
            .collect::<String>();
        std::fs::write(&path, source).expect("write large source");

        let location = SourceLocation::new(path, 1, 40, None);
        let snippet = source_snippet_with_context(&location, 0).expect("snippet");

        assert!(snippet.contains(">  1 | line 1"));
        assert!(snippet.contains("  ... | ..."));
        assert!(snippet.contains("> 40 | line 40"));
        assert!(!snippet.contains("> 17 | line 17"));
    }
}
