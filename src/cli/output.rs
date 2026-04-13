use colored::*;
use comfy_table::{presets::UTF8_FULL, Cell, Color as TableColor, Table};

use crate::analysis::engine::AnalysisReport;
use crate::domain::smell::{Severity, SmellCategory};

/// Print the full analysis report to stdout.
pub fn print_report(report: &AnalysisReport) {
    print_header();
    print_summary(report);

    if !report.smells.is_empty() {
        println!();
        print_smell_table(report);
    }

    if !report.parse_errors.is_empty() {
        println!();
        print_parse_errors(report);
    }

    print_footer(report);
}

fn print_header() {
    println!();
    println!(
        "{} {}",
        "QualiRS".bright_cyan().bold(),
        "— Rust Code Smell Detector".dimmed()
    );
    println!("{}", "━".repeat(60).dimmed());
}

fn print_summary(report: &AnalysisReport) {
    let total = report.total_smells();
    let critical = report.count_by_severity(Severity::Critical);
    let warnings = report.count_by_severity(Severity::Warning);
    let info = report.count_by_severity(Severity::Info);

    println!();
    println!(
        "  {} {} files analyzed, {} smell(s) detected",
        "→".bright_cyan(),
        report.total_files.to_string().bold(),
        total.to_string().bold()
    );

    if total > 0 {
        println!(
            "    {} critical  {} warning  {} info",
            critical.to_string().red().bold(),
            warnings.to_string().yellow().bold(),
            info.to_string().blue().bold()
        );
    }
}

fn print_smell_table(report: &AnalysisReport) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        Cell::new("Severity"),
        Cell::new("Category"),
        Cell::new("Smell"),
        Cell::new("Location"),
        Cell::new("Message"),
    ]);

    let mut smells = report.smells.clone();
    smells.sort_by(|a, b| b.severity.cmp(&a.severity));

    for smell in &smells {
        let sev_cell = severity_cell(&smell.severity);
        let cat_cell = category_cell(&smell.category);

        table.add_row(vec![
            sev_cell,
            cat_cell,
            Cell::new(&smell.name),
            Cell::new(smell.location.to_string()),
            Cell::new(format!("{}\n{}", smell.message.bold(), smell.suggestion.dimmed())),
        ]);
    }

    println!("{table}");
}

fn severity_cell(severity: &Severity) -> Cell {
    match severity {
        Severity::Critical => Cell::new("CRIT").fg(TableColor::Red),
        Severity::Warning => Cell::new("WARN").fg(TableColor::Yellow),
        Severity::Info => Cell::new("INFO").fg(TableColor::Blue),
    }
}

fn category_cell(category: &SmellCategory) -> Cell {
    let color = match category {
        SmellCategory::Architecture => TableColor::Magenta,
        SmellCategory::Design => TableColor::Cyan,
        SmellCategory::Implementation => TableColor::Green,
        SmellCategory::Concurrency => TableColor::Yellow,
        SmellCategory::Unsafe => TableColor::Red,
    };
    Cell::new(category.to_string()).fg(color)
}

fn print_parse_errors(report: &AnalysisReport) {
    println!(
        "{}",
        "Parse errors (files could not be analyzed):".yellow().bold()
    );
    for error in &report.parse_errors {
        println!("  {} {error}", "✗".red());
    }
}

fn print_footer(report: &AnalysisReport) {
    println!("{}", "━".repeat(60).dimmed());
    if report.total_smells() == 0 {
        println!("{}", "  No smells detected. Clean code!".green().bold());
    } else {
        let total = report.total_smells();
        let critical = report.count_by_severity(Severity::Critical);
        if critical > 0 {
            println!(
                "  Found {total} smell(s), {critical} critical — consider refactoring.",
            );
        } else {
            println!("  Found {total} smell(s). Review warnings above.");
        }
    }
    println!();
}

/// Print the list of available detectors.
pub fn print_detector_list() {
    println!();
    println!("{}", "Available detectors:".bright_cyan().bold());
    println!("{}", "━".repeat(40).dimmed());

    let detectors = [
        ("Architecture", vec!["God Module", "Public API Explosion", "Feature Concentration", "Cyclic Crate Dependency", "Layer Violation", "Unstable Dependency"]),
        ("Design", vec!["Large Trait", "Excessive Generics", "Anemic Struct", "Wide Hierarchy", "Trait Impl Leakage", "Feature Envy", "Broken Constructor", "Rebellious Impl", "Deref Abuse", "Manual Drop"]),
        (
            "Implementation",
            vec![
                "Long Function",
                "Too Many Arguments",
                "Excessive Unwrap",
                "Deep Match Nesting",
                "Excessive Clone",
                "Magic Numbers",
                "Large Enum",
                "High Cyclomatic Complexity",
                "Deep If/Else Nesting",
                "Long Method Chain",
                "Unused Result Ignored",
                "Panic in Library",
                "Unsafe Block Overuse",
                "Lifetime Explosion",
                "Copy + Drop Conflict",
            ],
        ),
        ("Concurrency", vec!["Blocking in Async", "Large Future", "Arc Mutex Overuse", "Deadlock Risk", "Spawn Without Join", "Missing Send Bound"]),
        ("Unsafe", vec!["Unsafe Without Comment", "Transmute Usage", "Raw Pointer Arithmetic", "Multi Mut Ref Unsafe", "FFI Without Wrapper"]),
    ];

    for (category, names) in &detectors {
        println!();
        println!("  {} {}", "▸".bright_magenta(), category.bold());
        for name in names {
            println!("    • {name}");
        }
    }
    println!();
}
