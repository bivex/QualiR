use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects async functions that spawn tasks without ensuring the future is Send.
///
/// Functions that capture non-Send types and pass them to spawn() will fail
/// at runtime. This detector flags functions that use spawn inside closures
/// without explicit Send bounds.
pub struct MissingSendBoundDetector;

impl Detector for MissingSendBoundDetector {
    fn name(&self) -> &str {
        "Missing Send Bound"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Fn(fn_item) = item {
                // Check if this is a generic function
                if fn_item.sig.generics.params.is_empty() {
                    continue;
                }

                // Look for spawn calls in the function body
                let has_spawn = contains_spawn_call(fn_item);

                // Check if generic params have Send bound
                if has_spawn {
                    let has_send_bound = fn_item.sig.generics.params.iter().any(|p| {
                        if let syn::GenericParam::Type(tp) = p {
                            tp.bounds.iter().any(|b| {
                                if let syn::TypeParamBound::Trait(tb) = b {
                                    let last = tb.path.segments.last()
                                        .map(|s| s.ident.to_string());
                                    last.as_deref() == Some("Send")
                                } else {
                                    false
                                }
                            })
                        } else {
                            false
                        }
                    });

                    if !has_send_bound {
                        let line = fn_item.sig.fn_token.span.start().line;
                        smells.push(Smell::new(
                            SmellCategory::Concurrency,
                            "Missing Send Bound",
                            Severity::Warning,
                            SourceLocation {
                                file: file.path.clone(),
                                line_start: line,
                                line_end: line,
                                column: None,
                            },
                            format!(
                                "Generic async function `{}` uses spawn but has no `Send` bound on generic params",
                                fn_item.sig.ident
                            ),
                            "Add `T: Send` bounds to generic parameters used in spawned tasks.",
                        ));
                    }
                }
            }
        }

        smells
    }
}

fn contains_spawn_call(fn_item: &syn::ItemFn) -> bool {
    let code = quote_fn_body(fn_item);
    let lower = code.to_lowercase();
    lower.contains("spawn(") || lower.contains("spawn_local(") || lower.contains("spawn_blocking(")
}

fn quote_fn_body(fn_item: &syn::ItemFn) -> String {
    // Simple stringification of the block
    format!("{:?}", fn_item.block)
}
