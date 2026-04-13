use std::path::Path;

use rayon::prelude::*;

use crate::analysis::detector::Detector;
use crate::domain::config::Config;
use crate::domain::smell::{Severity, Smell};
use crate::domain::source::SourceFile;
use crate::infrastructure::walker::RustFileWalker;

/// The central analysis engine. Owns a set of detectors and runs them
/// across a codebase.
pub struct Engine {
    detectors: Vec<Box<dyn Detector>>,
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        Self {
            detectors: Vec::new(),
            config,
        }
    }

    /// Register a detector. Call before `analyze`.
    pub fn register(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }

    /// Register all built-in detectors.
    pub fn register_defaults(&mut self) {
        use crate::detectors;

        // Architecture
        self.register(Box::new(detectors::architecture::god_module::GodModuleDetector));
        self.register(Box::new(detectors::architecture::public_api_explosion::PublicApiExplosionDetector));
        self.register(Box::new(detectors::architecture::feature_concentration::FeatureConcentrationDetector));
        self.register(Box::new(detectors::architecture::cyclic_crate_dependency::CyclicDependencyDetector));
        self.register(Box::new(detectors::architecture::layer_violation::LayerViolationDetector));
        self.register(Box::new(detectors::architecture::unstable_dependency::UnstableDependencyDetector));
        self.register(Box::new(detectors::architecture::leaky_error::LeakyErrorAbstractionDetector));
        self.register(Box::new(detectors::architecture::hidden_global_state::HiddenGlobalStateDetector));

        // Design
        self.register(Box::new(detectors::design::large_trait::LargeTraitDetector));
        self.register(Box::new(detectors::design::excessive_generics::ExcessiveGenericsDetector));
        self.register(Box::new(detectors::design::anemic_struct::AnemicStructDetector));
        self.register(Box::new(detectors::design::wide_hierarchy::WideHierarchyDetector));
        self.register(Box::new(detectors::design::trait_impl_leakage::TraitImplLeakageDetector));
        self.register(Box::new(detectors::design::feature_envy::FeatureEnvyDetector));
        self.register(Box::new(detectors::design::broken_constructor::BrokenConstructorDetector));
        self.register(Box::new(detectors::design::rebellious_impl::RebelliousImplDetector));
        self.register(Box::new(detectors::design::deref_abuse::DerefAbuseDetector));
        self.register(Box::new(detectors::design::manual_drop::ManualDropDetector));
        self.register(Box::new(detectors::design::fat_impl::FatImplDetector));
        self.register(Box::new(detectors::design::primitive_obsession::PrimitiveObsessionDetector));
        self.register(Box::new(detectors::design::data_clumps::DataClumpsDetector));
        self.register(Box::new(detectors::design::multiple_impl_blocks::MultipleImplBlocksDetector));

        // Implementation
        self.register(Box::new(detectors::implementation::long_function::LongFunctionDetector));
        self.register(Box::new(detectors::implementation::too_many_arguments::TooManyArgumentsDetector));
        self.register(Box::new(detectors::implementation::excessive_unwrap::ExcessiveUnwrapDetector));
        self.register(Box::new(detectors::implementation::deep_match::DeepMatchDetector));
        self.register(Box::new(detectors::implementation::excessive_clone::ExcessiveCloneDetector));
        self.register(Box::new(detectors::implementation::magic_numbers::MagicNumbersDetector));
        self.register(Box::new(detectors::implementation::large_enum::LargeEnumDetector));
        self.register(Box::new(detectors::implementation::cyclomatic_complexity::CyclomaticComplexityDetector));
        self.register(Box::new(detectors::implementation::deep_if_else::DeepIfElseDetector));
        self.register(Box::new(detectors::implementation::long_method_chain::LongMethodChainDetector));
        self.register(Box::new(detectors::implementation::unused_result::UnusedResultDetector));
        self.register(Box::new(detectors::implementation::panic_in_library::PanicInLibraryDetector));
        self.register(Box::new(detectors::implementation::unsafe_overuse::UnsafeOveruseDetector));
        self.register(Box::new(detectors::implementation::lifetime_explosion::LifetimeExplosionDetector));
        self.register(Box::new(detectors::implementation::copy_drop_conflict::CopyDropConflictDetector));
        self.register(Box::new(detectors::implementation::deeply_nested_type::DeeplyNestedTypeDetector));
        self.register(Box::new(detectors::implementation::interior_mutability_abuse::InteriorMutabilityAbuseDetector));

        // Concurrency
        self.register(Box::new(detectors::concurrency::blocking_in_async::BlockingInAsyncDetector));
        self.register(Box::new(detectors::concurrency::large_future::LargeFutureDetector));
        self.register(Box::new(detectors::concurrency::arc_mutex_overuse::ArcMutexOveruseDetector));
        self.register(Box::new(detectors::concurrency::deadlock_risk::DeadlockRiskDetector));
        self.register(Box::new(detectors::concurrency::spawn_without_join::SpawnWithoutJoinDetector));
        self.register(Box::new(detectors::concurrency::missing_send_bound::MissingSendBoundDetector));
        self.register(Box::new(detectors::concurrency::sync_drop_blocking::SyncDropBlockingDetector));
        self.register(Box::new(detectors::concurrency::async_trait_overhead::AsyncTraitOverheadDetector));

        // Unsafe
        self.register(Box::new(detectors::r#unsafe::unsafe_without_comment::UnsafeWithoutCommentDetector));
        self.register(Box::new(detectors::r#unsafe::transmute_usage::TransmuteUsageDetector));
        self.register(Box::new(detectors::r#unsafe::raw_pointer_arithmetic::RawPointerArithmeticDetector));
        self.register(Box::new(detectors::r#unsafe::multi_mut_ref_unsafe::MultiMutRefUnsafeDetector));
        self.register(Box::new(detectors::r#unsafe::ffi_without_wrapper::FfiWithoutWrapperDetector));
        self.register(Box::new(detectors::r#unsafe::inline_assembly::InlineAssemblyDetector));
    }

    /// Analyze all Rust files under `path` and return detected smells.
    pub fn analyze(&self, path: &Path) -> AnalysisReport {
        let walker = RustFileWalker::new(path, &self.config.exclude_paths);
        let files = walker.collect_files();

        let parse_errors: std::sync::Mutex<Vec<crate::domain::source::ParseError>> =
            std::sync::Mutex::new(Vec::new());

        let all_smells: Vec<Smell> = files
            .par_iter()
            .flat_map(|file_path| {
                match SourceFile::from_path(file_path.clone()) {
                    Ok(source) => {
                        let mut smells = Vec::new();
                        for detector in &self.detectors {
                            smells.extend(detector.detect(&source));
                        }
                        smells
                    }
                    Err(e) => {
                        parse_errors.lock().unwrap().push(e);
                        Vec::new()
                    }
                }
            })
            .filter(|smell| smell.severity >= self.config.min_severity)
            .collect();

        let errors = parse_errors.into_inner().unwrap();
        let total_files = files.len();

        AnalysisReport::new(all_smells, total_files, errors)
    }
}

/// Result of analyzing a codebase.
pub struct AnalysisReport {
    pub smells: Vec<Smell>,
    pub total_files: usize,
    pub parse_errors: Vec<crate::domain::source::ParseError>,
}

impl AnalysisReport {
    pub fn new(smells: Vec<Smell>, total_files: usize, parse_errors: Vec<crate::domain::source::ParseError>) -> Self {
        Self { smells, total_files, parse_errors }
    }
}

impl AnalysisReport {
    /// Smells grouped by category.
    #[allow(dead_code)]
    pub fn by_category(&self) -> std::collections::HashMap<crate::domain::smell::SmellCategory, Vec<&Smell>> {
        let mut map: std::collections::HashMap<crate::domain::smell::SmellCategory, Vec<&Smell>> = std::collections::HashMap::new();
        for smell in &self.smells {
            map.entry(smell.category).or_default().push(smell);
        }
        map
    }

    pub fn total_smells(&self) -> usize {
        self.smells.len()
    }

    pub fn count_by_severity(&self, severity: Severity) -> usize {
        self.smells.iter().filter(|s| s.severity == severity).count()
    }
}
