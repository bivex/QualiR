use syn::visit::{Visit, visit_expr};

use crate::analysis::detector::Detector;
use crate::detectors::policy::is_test_path;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Well-known numbers that are NOT considered magic.
const WHITELIST: &[i64] = &[0, 1, -1, 2, 10, 100, 1000, 255, 256, 1024];

/// Detects magic number literals in function bodies.
pub struct MagicNumbersDetector;

impl Detector for MagicNumbersDetector {
    fn name(&self) -> &str {
        "Magic Numbers"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        if is_test_path(&file.path) {
            return smells;
        }

        for item in &file.ast.items {
            if let syn::Item::Fn(fn_item) = item {
                let mut visitor = MagicNumberVisitor {
                    magic_numbers: Vec::new(),
                };
                visitor.visit_block(&fn_item.block);

                if !visitor.magic_numbers.is_empty() {
                    let line = fn_item.sig.fn_token.span.start().line;

                    let mut unique = visitor.magic_numbers;
                    unique.sort();
                    unique.dedup();

                    smells.push(Smell::new(
                        SmellCategory::Implementation,
                        "Magic Numbers",
                        Severity::Info,
                        SourceLocation {
                            file: file.path.clone(),
                            line_start: line,
                            line_end: line,
                            column: None,
                        },
                        format!(
                            "Function `{}` contains magic numbers: {}",
                            fn_item.sig.ident,
                            unique
                                .iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        "Extract magic numbers into named constants.",
                    ));
                }
            }
        }

        smells
    }
}

struct MagicNumberVisitor {
    magic_numbers: Vec<i64>,
}

impl<'ast> Visit<'ast> for MagicNumberVisitor {
    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        if let syn::Expr::Lit(lit_expr) = expr {
            if let syn::Lit::Int(lit_int) = &lit_expr.lit {
                if let Ok(val) = lit_int.base10_parse::<i64>() {
                    if !WHITELIST.contains(&val) {
                        self.magic_numbers.push(val);
                    }
                }
            }
        }
        visit_expr(self, expr);
    }
}
