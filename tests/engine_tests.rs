use std::path::PathBuf;

use qualirs::analysis::engine::Engine;
use qualirs::domain::config::Config;
use qualirs::domain::smell::Severity;
use qualirs::domain::source::SourceFile;

/// Create a temp dir with sample Rust files and analyze them.
#[test]
fn engine_detects_smells_in_sample_project() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let dir_path = dir.path();

    // Write a file with several known smells
    let smelly_code = r#"
// A file with intentional smells

pub fn overly_long_and_complex(x: i32, y: i32, z: i32, w: i32, v: i32, u: i32, extra: i32) {
    // Too many arguments (7 > 6)
    let a = Some(1).unwrap();
    let b = Some(2).unwrap();
    let c = Some(3).unwrap();
    let d = Some(4).unwrap();
    // Excessive unwrap (4 > 3)

    if x > 0 {
        if y > 0 {
            if z > 0 {
                if w > 0 {
                    if v > 0 {
                        // Deep if/else nesting > 4
                    }
                }
            }
        }
    }

    // Magic number
    let magic = 1337;

    // Unsafe without comment
    unsafe { let _ = 1; }
    unsafe { let _ = 2; }
    unsafe { let _ = 3; }
    unsafe { let _ = 4; }
    unsafe { let _ = 5; }
    unsafe { let _ = 6; }
}
"#;
    std::fs::write(dir_path.join("smelly.rs"), smelly_code).expect("write smelly.rs");

    // Write a clean file
    let clean_code = r#"
fn clean_function(x: i32) -> i32 {
    x + 1
}
"#;
    std::fs::write(dir_path.join("clean.rs"), clean_code).expect("write clean.rs");

    let config = Config::default();
    let mut engine = Engine::new(config);
    engine.register_defaults();

    let report = engine.analyze(dir_path);

    // Should find files
    assert!(report.total_files >= 2, "Should find at least 2 files");

    // Should detect smells
    assert!(
        report.total_smells() > 0,
        "Should detect at least some smells"
    );

    // Check specific detectors fired
    let smell_names: Vec<&str> = report.smells.iter().map(|s| s.name.as_str()).collect();
    assert!(
        smell_names.contains(&"Too Many Arguments"),
        "Should detect too many arguments"
    );
    assert!(
        smell_names.contains(&"Excessive Unwrap"),
        "Should detect excessive unwrap"
    );
    assert!(
        smell_names.contains(&"Deep If/Else Nesting"),
        "Should detect deep if/else"
    );
    assert!(
        smell_names.contains(&"Magic Numbers"),
        "Should detect magic numbers"
    );
    assert!(
        smell_names.contains(&"Unsafe Block Overuse"),
        "Should detect unsafe overuse"
    );
}

#[test]
fn min_severity_filters_correctly() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let code = r#"
fn risky() {
    let _ = Some(1).unwrap();
    let _ = Some(2).unwrap();
    let _ = Some(3).unwrap();
    let _ = Some(4).unwrap();
}
"#;
    std::fs::write(dir.path().join("prod.rs"), code).expect("write prod.rs");

    // With min_severity = Warning, should still find excessive unwrap (Warning)
    let config = Config {
        min_severity: Severity::Warning,
        ..Config::default()
    };
    let mut engine = Engine::new(config);
    engine.register_defaults();
    let report = engine.analyze(dir.path());
    assert!(report.total_smells() > 0, "Should find warnings");

    // With min_severity = Critical, should find nothing (no critical smells here)
    let config2 = Config {
        min_severity: Severity::Critical,
        ..Config::default()
    };
    let mut engine2 = Engine::new(config2);
    engine2.register_defaults();
    let report2 = engine2.analyze(dir.path());
    assert_eq!(
        report2.total_smells(),
        0,
        "Should find nothing at Critical severity"
    );
}

#[test]
fn policy_skip_tests_controls_test_file_analysis() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let tests_dir = dir.path().join("tests");
    std::fs::create_dir(&tests_dir).expect("create tests dir");
    let code = r#"
fn risky_test_helper() {
    let _ = Some(1).unwrap();
    let _ = Some(2).unwrap();
    let _ = Some(3).unwrap();
    let _ = Some(4).unwrap();
}
"#;
    std::fs::write(tests_dir.join("risky.rs"), code).expect("write risky test file");

    let mut engine = Engine::new(Config::default());
    engine.register_defaults();
    let skipped_report = engine.analyze(dir.path());
    assert_eq!(
        skipped_report.total_smells(),
        0,
        "default policy should skip test files"
    );

    let config = Config {
        policy: qualirs::domain::config::PolicyConfig {
            skip_tests: false,
            ..Default::default()
        },
        ..Config::default()
    };
    let mut engine = Engine::new(config);
    engine.register_defaults();
    let analyzed_report = engine.analyze(dir.path());
    assert!(
        analyzed_report
            .smells
            .iter()
            .any(|smell| smell.name == "Excessive Unwrap"),
        "disabling skip_tests should analyze test files"
    );
}

#[test]
fn parse_errors_are_collected() {
    let dir = tempfile::tempdir().expect("create temp dir");
    std::fs::write(dir.path().join("bad.rs"), "fn incomplete {").expect("write bad.rs");

    let config = Config::default();
    let mut engine = Engine::new(config);
    engine.register_defaults();

    let report = engine.analyze(dir.path());
    assert_eq!(
        report.parse_errors.len(),
        1,
        "Should report one parse error"
    );
}

#[test]
fn source_file_from_source_works() {
    let code = "fn main() { println!(\"hello\"); }";
    let sf = SourceFile::from_source(PathBuf::from("test.rs"), code.to_string());
    assert!(sf.is_ok());
    let sf = sf.unwrap();
    assert_eq!(sf.line_count, 1);
    assert_eq!(sf.ast.items.len(), 1);
}

#[test]
fn source_file_rejects_invalid_rust() {
    let code = "this is not valid rust {{{{";
    let sf = SourceFile::from_source(PathBuf::from("bad.rs"), code.to_string());
    assert!(sf.is_err());
}
