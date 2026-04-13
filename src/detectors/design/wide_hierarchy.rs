use crate::analysis::detector::Detector;
use crate::domain::config::Thresholds;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects enums with too many variants or structs with too many fields.
pub struct WideHierarchyDetector;

impl Detector for WideHierarchyDetector {
    fn name(&self) -> &str {
        "Wide Hierarchy"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let thresholds = Thresholds::default();
        let mut smells = Vec::new();

        for item in &file.ast.items {
            match item {
                syn::Item::Enum(e) => {
                    let count = e.variants.len();
                    if count > thresholds.design.wide_hierarchy {
                        let line = e.brace_token.span.open().start().line;
                        smells.push(Smell::new(
                            SmellCategory::Design,
                            "Wide Hierarchy",
                            Severity::Warning,
                            SourceLocation {
                                file: file.path.clone(),
                                line_start: line,
                                line_end: line,
                                column: None,
                            },
                            format!(
                                "Enum `{}` has {} variants (threshold: {})",
                                e.ident, count, thresholds.design.wide_hierarchy
                            ),
                            "Consider grouping variants into sub-enums or using the type-state pattern.",
                        ));
                    }
                }
                syn::Item::Struct(s) => {
                    if let syn::Fields::Named(named) = &s.fields {
                        let count = named.named.len();
                        if count > thresholds.design.wide_hierarchy {
                            let line = named.brace_token.span.open().start().line;
                            smells.push(Smell::new(
                                SmellCategory::Design,
                                "Wide Hierarchy",
                                Severity::Info,
                                SourceLocation {
                                    file: file.path.clone(),
                                    line_start: line,
                                    line_end: line,
                                    column: None,
                                },
                                format!(
                                    "Struct `{}` has {} fields (threshold: {})",
                                    s.ident, count, thresholds.design.wide_hierarchy
                                ),
                                "Consider grouping related fields into sub-structs.",
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        smells
    }
}
