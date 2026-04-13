use crate::domain::smell::Smell;
use crate::domain::source::SourceFile;
use std::any::type_name;

/// A detector inspects source files and produces smells.
///
/// Every smell category (architecture, design, implementation, etc.)
/// implements this trait. The engine runs all registered detectors
/// across every source file and collects results.
pub trait Detector: Send + Sync {
    /// Human-readable name, e.g. "God Module".
    #[allow(dead_code)]
    fn name(&self) -> &str;

    /// Detect smells in a single parsed source file.
    fn detect(&self, file: &SourceFile) -> Vec<Smell>;
}

/// Helper: get a short type name for a detector (for diagnostics).
#[allow(dead_code)]
pub fn detector_type_name<D: Detector>() -> &'static str {
    type_name::<D>()
}
