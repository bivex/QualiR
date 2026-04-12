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

        // Design
        self.register(Box::new(detectors::design::large_trait::LargeTraitDetector));
        self.register(Box::new(detectors::design::excessive_generics::ExcessiveGenericsDetector));
        self.register(Box::new(detectors::design::anemic_struct::AnemicStructDetector));

        // Implementation
        self.register(Box::new(detectors::implementation::long_function::LongFunctionDetector));
        self.register(Box::new(detectors::implementation::too_many_arguments::TooManyArgumentsDetector));
        self.register(Box::new(detectors::implementation::excessive_unwrap::ExcessiveUnwrapDetector));
        self.register(Box::new(detectors::implementation::deep_match::DeepMatchDetector));
        self.register(Box::new(detectors::implementation::excessive_clone::ExcessiveCloneDetector));
        self.register(Box::new(detectors::implementation::magic_numbers::MagicNumbersDetector));
        self.register(Box::new(detectors::implementation::large_enum::LargeEnumDetector));

        // Unsafe
        self.register(Box::new(detectors::r#unsafe::unsafe_without_comment::UnsafeWithoutCommentDetector));
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

        AnalysisReport {
            smells: all_smells,
            total_files,
            parse_errors: errors,
        }
    }
}

/// Result of analyzing a codebase.
pub struct AnalysisReport {
    pub smells: Vec<Smell>,
    pub total_files: usize,
    pub parse_errors: Vec<crate::domain::source::ParseError>,
}

impl AnalysisReport {
    /// Smells grouped by category.
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
