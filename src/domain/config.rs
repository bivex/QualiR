#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ArchThresholds {
    pub god_module_loc: usize,
    pub god_module_items: usize,
    pub public_api_ratio: f64,
    pub feature_concentration: usize,
    pub hidden_global_state: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct DesignThresholds {
    pub large_trait_methods: usize,
    pub excessive_generics: usize,
    pub deep_trait_bounds: usize,
    pub wide_hierarchy: usize,
    pub fat_impl_methods: usize,
    pub god_struct_fields: usize,
    pub primitive_obsession_fields: usize,
    pub data_clumps_args: usize,
    pub data_clumps_occurrences: usize,
    pub stringly_typed_fields: usize,
    pub large_error_enum_variants: usize,
}

/// Thresholds for control-flow complexity smells.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub(crate) struct ControlFlowThresholds {
    pub long_function_loc: usize,
    pub long_closure_loc: usize,
    pub deep_closure_nesting: usize,
    pub cyclomatic_complexity: usize,
    pub too_many_arguments: usize,
    pub deep_match_nesting: usize,
    pub deep_if_else: usize,
    pub excessive_unwrap: usize,
    pub large_enum_variants: usize,
    pub long_method_chain: usize,
    pub lifetime_explosion: usize,
}

/// Thresholds for type-safety and unsafe-usage smells.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub(crate) struct TypeSafetyThresholds {
    pub unsafe_block_overuse: usize,
    pub deeply_nested_type: usize,
    pub interior_mutability_abuse: usize,
}

/// Combined implementation thresholds composed of focused sub-groups.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ImplThresholds {
    #[serde(default, flatten)] pub(crate) control_flow: ControlFlowThresholds,
    #[serde(default, flatten)] pub(crate) type_safety: TypeSafetyThresholds,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ConcurrencyThresholds {
    pub large_future_loc: usize,
    pub arc_mutex_overuse: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UnsafeThresholds {
    pub unsafe_without_comment: bool,
}

/// Thresholds that control when a smell is reported.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Thresholds {
    #[serde(default)] pub arch: ArchThresholds,
    #[serde(default)] pub design: DesignThresholds,
    #[serde(default)] pub r#impl: ImplThresholds,
    #[serde(default)] pub concurrency: ConcurrencyThresholds,
    #[serde(default)] pub r#unsafe: UnsafeThresholds,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_thresholds_are_sane() {
        let config = Config::default();
        assert_eq!(config.thresholds.r#impl.control_flow.long_function_loc, 50);
        assert_eq!(config.thresholds.r#impl.control_flow.too_many_arguments, 6);
        assert_eq!(config.thresholds.r#impl.control_flow.excessive_unwrap, 3);
        assert_eq!(config.thresholds.r#impl.control_flow.cyclomatic_complexity, 15);
        assert_eq!(config.thresholds.r#impl.control_flow.deep_match_nesting, 3);
        assert_eq!(config.thresholds.r#impl.control_flow.deep_if_else, 4);
        assert_eq!(config.thresholds.r#impl.control_flow.large_enum_variants, 20);
        assert_eq!(config.thresholds.r#impl.control_flow.lifetime_explosion, 4);
        assert_eq!(config.thresholds.r#impl.type_safety.deeply_nested_type, 3);
        assert_eq!(config.thresholds.r#impl.type_safety.interior_mutability_abuse, 5);

        // Arch
        assert_eq!(config.thresholds.arch.hidden_global_state, 3);

        // Design
        assert_eq!(config.thresholds.design.fat_impl_methods, 20);
        assert_eq!(config.thresholds.design.primitive_obsession_fields, 4);
        assert_eq!(config.thresholds.design.data_clumps_args, 3);
        assert_eq!(config.thresholds.design.data_clumps_occurrences, 3);
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            arch: ArchThresholds {
                god_module_loc: 1000,
                god_module_items: 20,
                public_api_ratio: 0.7,
                feature_concentration: 15,
                hidden_global_state: 3,
            },
            design: DesignThresholds {
                large_trait_methods: 15,
                excessive_generics: 5,
                deep_trait_bounds: 4,
                wide_hierarchy: 10,
                fat_impl_methods: 20,
                god_struct_fields: 20,
                primitive_obsession_fields: 4,
                data_clumps_args: 3,
                data_clumps_occurrences: 3,
                stringly_typed_fields: 3,
                large_error_enum_variants: 12,
            },
            r#impl: ImplThresholds {
                control_flow: ControlFlowThresholds {
                    long_function_loc: 50,
                    long_closure_loc: 25,
                    deep_closure_nesting: 3,
                    cyclomatic_complexity: 15,
                    too_many_arguments: 6,
                    deep_match_nesting: 3,
                    deep_if_else: 4,
                    excessive_unwrap: 3,
                    large_enum_variants: 20,
                    long_method_chain: 4,
                    lifetime_explosion: 4,
                },
                type_safety: TypeSafetyThresholds {
                    unsafe_block_overuse: 5,
                    deeply_nested_type: 3,
                    interior_mutability_abuse: 5,
                },
            },
            concurrency: ConcurrencyThresholds {
                large_future_loc: 100,
                arc_mutex_overuse: 3,
            },
            r#unsafe: UnsafeThresholds {
                unsafe_without_comment: true,
            },
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
