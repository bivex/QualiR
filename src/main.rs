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

    let config = get_config(&args)?;
    let engine = setup_engine(config);
    let report = engine.analyze(&args.path);

    if args.quiet {
        print_summary(&report);
    } else {
        cli::output::print_report(&report);
    }

    if report.count_by_severity(Severity::Critical) > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn get_config(args: &Args) -> anyhow::Result<Config> {
    let path = args.path.canonicalize().unwrap_or_else(|_| args.path.clone());
    let mut config = if let Some(config_path) = &args.config {
        Config::load_from_file(config_path)?
    } else {
        Config::load_or_default(&path)
    };

    config.min_severity = match args.min_severity.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "warning" | "warn" => Severity::Warning,
        "info" => Severity::Info,
        other => {
            anyhow::bail!("Unknown severity level: {other}. Use: info, warning, critical");
        }
    };

    Ok(config)
}

fn setup_engine(config: Config) -> Engine {
    let mut engine = Engine::new(config);
    engine.register_defaults();
    engine
}

fn print_summary(report: &analysis::engine::AnalysisReport) {
    println!(
        "Files: {} | Smells: {} (Critical: {}, Warning: {}, Info: {})",
        report.total_files,
        report.total_smells(),
        report.count_by_severity(Severity::Critical),
        report.count_by_severity(Severity::Warning),
        report.count_by_severity(Severity::Info),
    );
}
