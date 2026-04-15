mod analysis;
mod cli;
mod detectors;
mod domain;
mod infrastructure;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use analysis::engine::Engine;
use cli::args::{Args, Command, OutputFormat};
use domain::config::Config;
use domain::smell::{Severity, SmellCategory};
use infrastructure::source::{
    GitReference, SourceRequest, prepare_source, prepare_source_in, prepare_source_with_options,
};

fn main() -> anyhow::Result<ExitCode> {
    run()
}

fn run() -> anyhow::Result<ExitCode> {
    let args = Args::parse_args();

    if let Some(command) = &args.command {
        run_command(command)?;
        return Ok(ExitCode::SUCCESS);
    }

    if args.list_detectors {
        cli::detector_list::print_detector_list();
        return Ok(ExitCode::SUCCESS);
    }

    let source = prepare_analysis_source(&args)?;
    if let Some(path) = source.preserved_path() {
        eprintln!("Preserving temporary analysis source at {}", path.display());
    }
    let config = get_config(&args, source.path())?;
    let engine = setup_engine(config);
    let mut report = engine.analyze(source.path());

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
        return Ok(ExitCode::FAILURE);
    }

    Ok(ExitCode::SUCCESS)
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

fn prepare_analysis_source(args: &Args) -> anyhow::Result<infrastructure::source::PreparedSource> {
    if args.git.is_none() && (args.branch.is_some() || args.tag.is_some()) {
        anyhow::bail!("--branch and --tag can only be used with --git");
    }

    let request = if let Some(url) = args.git.as_deref() {
        SourceRequest::Git {
            url,
            reference: git_reference(args),
        }
    } else if let Some(name) = args.crate_name.as_deref() {
        SourceRequest::Crate {
            name,
            version: args.crate_version.as_deref(),
        }
    } else {
        let path = args.path.as_deref().unwrap_or_else(|| Path::new("."));
        SourceRequest::Local(path)
    };

    if args.keep_temp {
        prepare_source_with_options(request, args.temp_dir.as_deref(), true)
    } else if let Some(temp_dir) = args.temp_dir.as_deref() {
        prepare_source_in(request, Some(temp_dir))
    } else {
        prepare_source(request)
    }
}

fn git_reference(args: &Args) -> Option<GitReference<'_>> {
    args.branch
        .as_deref()
        .map(GitReference::Branch)
        .or_else(|| args.tag.as_deref().map(GitReference::Tag))
}

fn get_config(args: &Args, analysis_path: &Path) -> anyhow::Result<Config> {
    let path = analysis_path
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(analysis_path));
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
