use syn::visit::Visit;

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects `let _ = expr()` where the expression returns a `Result` or `Option`.
///
/// Silently discarding Results is a common source of hidden bugs.
pub struct UnusedResultDetector;

impl Detector for UnusedResultDetector {
    fn name(&self) -> &str {
        "Unused Result Ignored"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Fn(fn_item) = item {
                let mut visitor = UnusedResultVisitor {
                    findings: Vec::new(),
                };
                visitor.visit_block(&fn_item.block);

                for (line, expr_desc) in visitor.findings {
                    smells.push(Smell::new(
                        SmellCategory::Idiomaticity,
                        "Unused Result Ignored",
                        Severity::Warning,
                        SourceLocation {
                            file: file.path.clone(),
                            line_start: line,
                            line_end: line,
                            column: None,
                        },
                        format!(
                            "Function `{}` discards a Result/Option with `let _ = ...` ({})",
                            fn_item.sig.ident, expr_desc
                        ),
                        "Handle the error explicitly with match, if let, or propagate with ?.",
                    ));
                }
            }
        }

        smells
    }
}

struct UnusedResultVisitor {
    findings: Vec<(usize, String)>,
}

impl<'ast> Visit<'ast> for UnusedResultVisitor {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        // Check for `let _ = expr;` pattern
        if let syn::Pat::Wild(wild) = &local.pat {
            if let Some(init) = &local.init {
                let description = describe_expr(&init.expr);
                let line = wild.underscore_token.span.start().line;
                self.findings.push((line, description));
            }
        }
        syn::visit::visit_local(self, local);
    }
}

fn describe_expr(expr: &syn::Expr) -> String {
    match expr {
        syn::Expr::Call(call) => {
            let func_name = extract_path_string(&call.func);
            format!("call to `{}`", func_name)
        }
        syn::Expr::MethodCall(call) => {
            format!("`.{}()` call", call.method)
        }
        syn::Expr::Path(path) => {
            format!("`{}`", path_to_string(&path.path))
        }
        _ => String::from("expression"),
    }
}

fn extract_path_string(expr: &syn::Expr) -> String {
    if let syn::Expr::Path(p) = expr {
        path_to_string(&p.path)
    } else {
        String::from("...")
    }
}

fn path_to_string(path: &syn::Path) -> String {
    let idents: Vec<String> = path.segments.iter()
        .map(|s| s.ident.to_string())
        .collect();
    idents.join("::")
}
