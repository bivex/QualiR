use std::path::PathBuf;

use qualirs::analysis::detector::Detector;
use qualirs::domain::source::SourceFile;

/// Parse a Rust source string into a SourceFile for testing.
pub fn parse(code: &str) -> SourceFile {
    SourceFile::from_source(PathBuf::from("test.rs"), code.to_string())
        .expect("test source should parse")
}

/// Run a detector on code and return smells.
pub fn detect<D: Detector>(detector: &D, code: &str) -> Vec<qualirs::domain::smell::Smell> {
    let file = parse(code);
    detector.detect(&file)
}

/// Assert that a detector finds exactly N smells with a given name.
pub fn assert_smell_count<D: Detector>(
    detector: &D,
    code: &str,
    smell_name: &str,
    expected: usize,
) {
    let smells = detect(detector, code);
    let count = smells.iter().filter(|s| s.name == smell_name).count();
    assert_eq!(
        count,
        expected,
        "Expected {expected} '{smell_name}' smell(s), found {count}. Smells: {:?}",
        smells.iter().map(|s| &s.name).collect::<Vec<_>>()
    );
}

/// Assert a detector finds zero smells.
pub fn assert_clean<D: Detector>(detector: &D, code: &str) {
    let smells = detect(detector, code);
    assert!(
        smells.is_empty(),
        "Expected no smells, but found: {:?}",
        smells
            .iter()
            .map(|s| format!("{}: {}", s.name, s.message))
            .collect::<Vec<_>>()
    );
}
