use crate::domain::smell::SourceLocation;

const MAX_CONTEXT_SNIPPET_LINES: usize = 32;

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
    let source = std::fs::read_to_string(&location.file).ok()?;
    let lines: Vec<_> = source.lines().collect();
    let range = ContextSnippetRange::new(location, lines.len(), context_lines)?;
    let mut snippet = String::new();

    if range.total_lines() <= MAX_CONTEXT_SNIPPET_LINES {
        write_context_lines(
            &mut snippet,
            &lines,
            range.snippet_start..=range.snippet_end,
            range,
        )?;
    } else {
        write_elided_context_snippet(&mut snippet, &lines, range)?;
    }

    Some(snippet)
}

#[derive(Clone, Copy)]
struct ContextSnippetRange {
    finding_start: usize,
    finding_end: usize,
    snippet_start: usize,
    snippet_end: usize,
}

impl ContextSnippetRange {
    fn new(location: &SourceLocation, line_count: usize, context_lines: usize) -> Option<Self> {
        if line_count == 0 || location.line_start == 0 || location.line_start > line_count {
            return None;
        }

        let finding_start = location.line_start;
        let finding_end = location.line_end.max(finding_start).min(line_count);
        let snippet_start = finding_start.saturating_sub(context_lines).max(1);
        let snippet_end = finding_end.saturating_add(context_lines).min(line_count);

        Some(Self {
            finding_start,
            finding_end,
            snippet_start,
            snippet_end,
        })
    }

    fn total_lines(self) -> usize {
        self.snippet_end - self.snippet_start + 1
    }

    fn line_number_width(self) -> usize {
        self.snippet_end.to_string().len()
    }
}

fn write_elided_context_snippet(
    snippet: &mut String,
    lines: &[&str],
    range: ContextSnippetRange,
) -> Option<()> {
    use std::fmt::Write as _;

    let leading_lines = MAX_CONTEXT_SNIPPET_LINES / 2;
    let trailing_lines = MAX_CONTEXT_SNIPPET_LINES - leading_lines;
    let leading_end = range.snippet_start + leading_lines - 1;
    let trailing_start = range.snippet_end - trailing_lines + 1;
    let line_number_width = range.line_number_width();

    write_context_lines(snippet, lines, range.snippet_start..=leading_end, range)?;
    writeln!(snippet, "  {:>line_number_width$} | ...", "...").ok()?;
    write_context_lines(snippet, lines, trailing_start..=range.snippet_end, range)
}

fn write_context_lines(
    snippet: &mut String,
    lines: &[&str],
    line_numbers: std::ops::RangeInclusive<usize>,
    range: ContextSnippetRange,
) -> Option<()> {
    let line_number_width = range.line_number_width();

    for line_number in line_numbers {
        write_context_line(
            snippet,
            lines,
            line_number,
            line_number_width,
            range.finding_start,
            range.finding_end,
        )?;
    }

    Some(())
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
