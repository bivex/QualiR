use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// QualiRS — structural and architectural code smell detector for Rust.
#[derive(Parser, Debug)]
#[command(name = "qualirs", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

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

    /// Compact mode: show findings as a categorized list (default)
    #[arg(long, conflicts_with_all = ["quiet", "table"])]
    pub compact: bool,

    /// Table mode: show findings in the legacy table layout
    #[arg(long, conflicts_with_all = ["quiet", "compact", "llm"])]
    pub table: bool,

    /// LLM mode: show compact Markdown with fenced finding blocks for coding assistants
    #[arg(long, conflicts_with_all = ["quiet", "compact"])]
    pub llm: bool,

    /// List available detectors and exit
    #[arg(long)]
    pub list_detectors: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate a default qualirs.toml configuration file
    InitConfig {
        /// Config file to create
        #[arg(short, long, default_value = "qualirs.toml")]
        output: PathBuf,

        /// Overwrite an existing config file
        #[arg(short, long)]
        force: bool,
    },
}

impl Args {
    pub fn parse_args() -> Self {
        Parser::parse()
    }
}
