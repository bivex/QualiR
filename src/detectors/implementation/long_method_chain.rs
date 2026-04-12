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
                    max_chain: 0,
                    threshold: thresholds.long_method_chain,
                    chains: Vec::new(),
                };
                visitor.visit_block(&fn_item.block);

                for (depth, line) in &visitor.chains {
                    smells.push(Smell::new(
                        SmellCategory::Implementation,
                        "Long Method Chain",
                        Severity::Info,
                        SourceLocation {
                            file: file.path.clone(),
                            line_start: *line,
                            line_end: *line,
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

        smells
    }
}

struct ChainVisitor {
    max_chain: usize,
    threshold: usize,
    chains: Vec<(usize, usize)>,
}

impl ChainVisitor {
    fn count_chain_depth(expr: &syn::Expr) -> usize {
        match expr {
            syn::Expr::MethodCall(call) => {
                1 + Self::count_chain_depth(&call.receiver)
            }
            syn::Expr::Field(field) => {
                1 + Self::count_chain_depth(&field.base)
            }
            syn::Expr::Await(await_expr) => {
                1 + Self::count_chain_depth(&await_expr.base)
            }
            syn::Expr::Try(try_expr) => {
                1 + Self::count_chain_depth(&try_expr.expr)
            }
            _ => 0,
        }
    }
}

impl<'ast> Visit<'ast> for ChainVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let depth = Self::count_chain_depth(&syn::Expr::MethodCall(node.clone()));
        if depth > self.max_chain {
            self.max_chain = depth;
        }
        if depth > self.threshold && Self::is_chain_root(node) {
            let line = node.receiver.span().start().line;
            // Avoid duplicate reports: only report if this is the outermost chain
            self.chains.push((depth, line));
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

impl ChainVisitor {
    fn is_chain_root(call: &syn::ExprMethodCall) -> bool {
        // We are at the root of the chain if the parent wouldn't be a method call
        // We detect this by checking if receiver is also a method call with same chain
        // Simple heuristic: report only when receiver is NOT a method call itself
        !matches!(&*call.receiver, syn::Expr::MethodCall(_))
    }
}
