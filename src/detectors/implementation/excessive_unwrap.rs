use syn::visit::{visit_expr, Visit};

use crate::analysis::detector::Detector;
use crate::domain::config::Thresholds;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects functions with excessive `.unwrap()` or `.expect()` calls.
pub struct ExcessiveUnwrapDetector;

impl Detector for ExcessiveUnwrapDetector {
    fn name(&self) -> &str {
        "Excessive Unwrap"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let thresholds = Thresholds::default();
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Fn(fn_item) = item {
                let mut visitor = UnwrapCounter::new();
                visitor.visit_block(&fn_item.block);

                if visitor.unwrap_count > thresholds.r#impl.control_flow.excessive_unwrap {
                    let line = fn_item.sig.fn_token.span.start().line;

                    smells.push(Smell::new(
                        SmellCategory::Idiomaticity,
                        "Excessive Unwrap",
                        Severity::Warning,
                        SourceLocation {
                            file: file.path.clone(),
                            line_start: line,
                            line_end: line,
                            column: None,
                        },
                        format!(
                            "Function `{}` has {} unwrap/expect calls (threshold: {})",
                            fn_item.sig.ident, visitor.unwrap_count, thresholds.r#impl.control_flow.excessive_unwrap
                        ),
                        "Use proper error handling with ?, map_err, or match instead of unwrap().",
                    ));
                }
            }
        }

        smells
    }
}

struct UnwrapCounter {
    unwrap_count: usize,
}

impl UnwrapCounter {
    fn new() -> Self {
        Self { unwrap_count: 0 }
    }
}

impl<'ast> Visit<'ast> for UnwrapCounter {
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        if let syn::Expr::MethodCall(call) = expr {
            if call.method == "unwrap" || call.method == "expect" {
                self.unwrap_count += 1;
            }
        }
        visit_expr(self, expr);
    }
}
