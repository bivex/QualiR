use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::analysis::detector::Detector;
use crate::domain::config::Thresholds;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects excessively long method chains like `a.b().c().d().e()`.
pub struct LongMethodChainDetector;

impl Detector for LongMethodChainDetector {
    fn name(&self) -> &str {
        "Long Method Chain"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let thresholds = Thresholds::default();
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Fn(fn_item) = item {
                let mut visitor = ChainVisitor {
                    threshold: thresholds.long_method_chain,
                    candidates: Vec::new(),
                };
                visitor.visit_block(&fn_item.block);

                // Deduplicate: keep only the longest chain per line
                visitor.candidates.sort_by(|a, b| b.0.cmp(&a.0));
                let mut seen_lines = std::collections::HashSet::new();
                for (depth, line) in visitor.candidates {
                    if seen_lines.insert(line) {
                        smells.push(Smell::new(
                            SmellCategory::Implementation,
                            "Long Method Chain",
                            Severity::Info,
                            SourceLocation {
                                file: file.path.clone(),
                                line_start: line,
                                line_end: line,
                                column: None,
                            },
                            format!(
                                "Function `{}` has a method chain of length {} (threshold: {})",
                                fn_item.sig.ident, depth, thresholds.long_method_chain
                            ),
                            "Break long chains into intermediate variables with descriptive names.",
                        ));
                    }
                }
            }
        }

        smells
    }
}

struct ChainVisitor {
    threshold: usize,
    candidates: Vec<(usize, usize)>, // (depth, line)
}

impl ChainVisitor {
    fn chain_depth(expr: &syn::Expr) -> usize {
        match expr {
            syn::Expr::MethodCall(call) => 1 + Self::chain_depth(&call.receiver),
            syn::Expr::Field(field) => 1 + Self::chain_depth(&field.base),
            syn::Expr::Await(a) => 1 + Self::chain_depth(&a.base),
            syn::Expr::Try(t) => 1 + Self::chain_depth(&t.expr),
            _ => 0,
        }
    }
}

impl<'ast> Visit<'ast> for ChainVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let depth = 1 + Self::chain_depth(&node.receiver);
        if depth > self.threshold {
            let line = node.span().start().line;
            self.candidates.push((depth, line));
        }
        // Don't recurse into the receiver — it would produce shorter chains
        // that we don't care about. Only visit the method arguments.
        for arg in &node.args {
            syn::visit::visit_expr(self, arg);
        }
        // Also visit the turbofish if present
        if let Some(turbofish) = &node.turbofish {
            for arg in &turbofish.args {
                syn::visit::visit_generic_argument(self, arg);
            }
        }
    }
}
