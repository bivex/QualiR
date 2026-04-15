mod analysis;
mod cli;
mod detectors;
mod domain;
mod infrastructure;

use analysis::engine::Engine;
use cli::args::{Args, Command, OutputFormat};
use domain::config::Config;
use domain::smell::{Severity, SmellCategory};

fn main() -> anyhow::Result<()> {
    let args = Args::parse_args();

    if let Some(command) = &args.command {
        return run_command(command);
    }

    if args.list_detectors {
        cli::detector_list::print_detector_list();
        return Ok(());
    }

    let config = get_config(&args)?;
    let engine = setup_engine(config);
    let mut report = engine.analyze(&args.path);

    if let Some(category) = &args.category {
        let category = category
            .parse::<SmellCategory>()
            .map_err(anyhow::Error::msg)?;
        report.smells.retain(|smell| smell.category == category);
    }

    if args.output_options.quiet {
        print_summary(&report);
    } else if args.output_options.format == Some(OutputFormat::Json) {
        cli::json_output::emit_json_report(&report, args.output_options.output_path.as_deref())?;
    } else if args.output_options.llm {
        cli::output::print_llm_report(&report);
    } else if args.output_options.table {
        cli::output::print_report(&report);
    } else {
        cli::output::print_compact_report(&report);
    }

    if report.count_by_severity(Severity::Critical) > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn run_command(command: &Command) -> anyhow::Result<()> {
    match command {
        Command::InitConfig { output, force } => {
            Config::write_default_file(output, *force)?;
            println!("Created {}", output.display());
            Ok(())
        }
    }
}

fn get_config(args: &Args) -> anyhow::Result<Config> {
    let path = args
        .path
        .canonicalize()
        .unwrap_or_else(|_| args.path.clone());
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
