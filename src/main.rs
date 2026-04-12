mod analysis;
mod cli;
mod detectors;
mod domain;
mod infrastructure;

use analysis::engine::Engine;
use cli::args::Args;
use domain::config::Config;
use domain::smell::Severity;

fn main() -> anyhow::Result<()> {
    let args = Args::parse_args();

    if args.list_detectors {
        cli::output::print_detector_list();
        return Ok(());
    }

    let path = args.path.canonicalize().unwrap_or_else(|_| args.path.clone());

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    // Load config
    let config = if let Some(config_path) = &args.config {
        Config::load_from_file(config_path)?
    } else {
        Config::load_or_default(&path)
    };

    // Override min severity from CLI
    let min_severity = match args.min_severity.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "warning" | "warn" => Severity::Warning,
        "info" => Severity::Info,
        other => {
            anyhow::bail!("Unknown severity level: {other}. Use: info, warning, critical");
        }
    };

    let mut config = config;
    config.min_severity = min_severity;

    // Build and run engine
    let mut engine = Engine::new(config);
    engine.register_defaults();

    let report = engine.analyze(&path);

    if args.quiet {
        println!(
            "Files: {} | Smells: {} (Critical: {}, Warning: {}, Info: {})",
            report.total_files,
            report.total_smells(),
            report.count_by_severity(Severity::Critical),
            report.count_by_severity(Severity::Warning),
            report.count_by_severity(Severity::Info),
        );
    } else {
        cli::output::print_report(&report);
    }

    // Exit with error code if critical smells found
    if report.count_by_severity(Severity::Critical) > 0 {
        std::process::exit(1);
    }

    Ok(())
}
