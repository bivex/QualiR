use std::path::PathBuf;

use clap::Parser;

/// QualiRS — structural and architectural code smell detector for Rust.
#[derive(Parser, Debug)]
#[command(name = "qualirs", version, about)]
pub struct Args {
    /// Path to the Rust project or file to analyze
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Configuration file path (default: qualirs.toml in project root)
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Minimum severity to report: info, warning, critical
    #[arg(short = 'm', long, default_value = "info")]
    pub min_severity: String,

    /// Show only smells of a specific category
    #[arg(short = 't', long)]
    pub category: Option<String>,

    /// Quiet mode: only show summary counts
    #[arg(short, long)]
    pub quiet: bool,

    /// List available detectors and exit
    #[arg(long)]
    pub list_detectors: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Parser::parse()
    }
}
