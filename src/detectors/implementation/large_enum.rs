use crate::analysis::detector::Detector;
use crate::domain::config::Thresholds;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects enums with too many variants.
pub struct LargeEnumDetector;

impl Detector for LargeEnumDetector {
    fn name(&self) -> &str {
        "Large Enum"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let thresholds = Thresholds::default();
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Enum(enum_item) = item {
                let variant_count = enum_item.variants.len();
                if variant_count > thresholds.r#impl.large_enum_variants {
                    let line = enum_item.brace_token.span.open().start().line;

                    smells.push(Smell::new(
                        SmellCategory::Implementation,
                        "Large Enum",
                        Severity::Warning,
                        SourceLocation {
                            file: file.path.clone(),
                            line_start: line,
                            line_end: line,
                            column: None,
                        },
                        format!(
                            "Enum `{}` has {} variants (threshold: {})",
                            enum_item.ident, variant_count, thresholds.r#impl.large_enum_variants
                        ),
                        "Consider splitting into multiple enums or using a trait-based dispatch.",
                    ));
                }
            }
        }

        smells
    }
}
