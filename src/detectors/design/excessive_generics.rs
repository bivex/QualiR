use crate::analysis::detector::Detector;
use crate::domain::config::Thresholds;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects functions, structs, and enums with too many generic parameters.
pub struct ExcessiveGenericsDetector;

impl Detector for ExcessiveGenericsDetector {
    fn name(&self) -> &str {
        "Excessive Generics"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let thresholds = Thresholds::default();
        let mut smells = Vec::new();

        for item in &file.ast.items {
            match item {
                syn::Item::Fn(fn_item) => {
                    check_generics(
                        &file.path,
                        &fn_item.sig.generics,
                        &format!("Function `{}`", fn_item.sig.ident),
                        &thresholds,
                        &mut smells,
                    );
                }
                syn::Item::Struct(s) => {
                    check_generics(
                        &file.path,
                        &s.generics,
                        &format!("Struct `{}`", s.ident),
                        &thresholds,
                        &mut smells,
                    );
                }
                syn::Item::Enum(e) => {
                    check_generics(
                        &file.path,
                        &e.generics,
                        &format!("Enum `{}`", e.ident),
                        &thresholds,
                        &mut smells,
                    );
                }
                _ => {}
            }
        }

        smells
    }
}

fn check_generics(
    file_path: &std::path::Path,
    generics: &syn::Generics,
    context: &str,
    thresholds: &Thresholds,
    smells: &mut Vec<Smell>,
) {
    let count = generics.params.len();
    if count > thresholds.excessive_generics {
        let line = generics
            .lt_token
            .map(|lt| lt.span.start().line)
            .unwrap_or(1);

        smells.push(Smell::new(
            SmellCategory::Design,
            "Excessive Generics",
            Severity::Warning,
            SourceLocation {
                file: file_path.to_path_buf(),
                line_start: line,
                line_end: line,
                column: None,
            },
            format!("{context} has {count} generic parameters (threshold: {})", thresholds.excessive_generics),
            "Reduce generic parameters. Consider concrete types or trait objects for complex cases.",
        ));
    }

    // Check deep trait bounds: T: A + B + C + ...
    for param in &generics.params {
        if let syn::GenericParam::Type(tp) = param {
            for bound in &tp.bounds {
                if let syn::TypeParamBound::Trait(_) = bound {
                    let bound_count = tp.bounds.len();
                    if bound_count > thresholds.deep_trait_bounds {
                        let line = tp
                            .colon_token
                            .map(|c| c.span.start().line)
                            .unwrap_or(1);

                        smells.push(Smell::new(
                            SmellCategory::Design,
                            "Deep Trait Bounds",
                            Severity::Info,
                            SourceLocation {
                                file: file_path.to_path_buf(),
                                line_start: line,
                                line_end: line,
                                column: None,
                            },
                            format!("{context}: type parameter `{}` has {bound_count} trait bounds (threshold: {})", tp.ident, thresholds.deep_trait_bounds),
                            "Consider creating a supertrait that combines common bounds.",
                        ));
                        break;
                    }
                }
            }
        }
    }
}
