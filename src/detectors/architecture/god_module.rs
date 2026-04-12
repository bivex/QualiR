use crate::analysis::detector::Detector;
use crate::domain::config::Thresholds;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects modules that are too large (too many lines or too many items).
pub struct GodModuleDetector;

impl Detector for GodModuleDetector {
    fn name(&self) -> &str {
        "God Module"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let thresholds = Thresholds::default();
        let mut smells = Vec::new();

        let item_count = file.ast.items.len();

        if file.line_count > thresholds.god_module_loc {
            smells.push(Smell::new(
                SmellCategory::Architecture,
                "God Module",
                Severity::Warning,
                SourceLocation {
                    file: file.path.clone(),
                    line_start: 1,
                    line_end: file.line_count,
                    column: None,
                },
                format!(
                    "Module has {} lines (threshold: {})",
                    file.line_count, thresholds.god_module_loc
                ),
                "Split this module into smaller, focused modules with clear responsibilities.",
            ));
        }

        if item_count > thresholds.god_module_items {
            smells.push(Smell::new(
                SmellCategory::Architecture,
                "God Module (items)",
                Severity::Warning,
                SourceLocation {
                    file: file.path.clone(),
                    line_start: 1,
                    line_end: file.line_count,
                    column: None,
                },
                format!(
                    "Module has {} top-level items (threshold: {})",
                    item_count, thresholds.god_module_items
                ),
                "Decompose into multiple modules, each with a single responsibility.",
            ));
        }

        smells
    }
}
