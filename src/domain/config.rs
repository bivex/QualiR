/// Thresholds that control when a smell is reported.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Thresholds {
    // Architecture
    pub god_module_loc: usize,
    pub god_module_items: usize,
    pub public_api_ratio: f64,

    // Design
    pub large_trait_methods: usize,
    pub excessive_generics: usize,
    pub deep_trait_bounds: usize,

    // Implementation
    pub long_function_loc: usize,
    pub cyclomatic_complexity: usize,
    pub too_many_arguments: usize,
    pub deep_match_nesting: usize,
    pub excessive_unwrap: usize,
    pub large_enum_variants: usize,

    // Concurrency
    pub large_future_loc: usize,

    // Unsafe
    pub unsafe_without_comment: bool,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            god_module_loc: 1000,
            god_module_items: 20,
            public_api_ratio: 0.7,

            large_trait_methods: 15,
            excessive_generics: 5,
            deep_trait_bounds: 4,

            long_function_loc: 50,
            cyclomatic_complexity: 15,
            too_many_arguments: 6,
            deep_match_nesting: 3,
            excessive_unwrap: 3,
            large_enum_variants: 20,

            large_future_loc: 100,

            unsafe_without_comment: true,
        }
    }
}

/// Root configuration loaded from qualirs.toml or defaults.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub thresholds: Thresholds,
    pub exclude_paths: Vec<String>,
    pub min_severity: crate::domain::smell::Severity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thresholds: Thresholds::default(),
            exclude_paths: vec![
                "target".into(),
                ".git".into(),
                "node_modules".into(),
            ],
            min_severity: crate::domain::smell::Severity::Info,
        }
    }
}

impl Config {
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_or_default(dir: &std::path::Path) -> Self {
        let config_path = dir.join("qualirs.toml");
        if config_path.exists() {
            Self::load_from_file(&config_path).unwrap_or_else(|e| {
                eprintln!("Warning: failed to load {}: {e}", config_path.display());
                Self::default()
            })
        } else {
            Self::default()
        }
    }
}
