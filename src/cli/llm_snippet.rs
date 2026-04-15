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
}
