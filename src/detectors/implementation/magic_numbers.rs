use syn::visit::{Visit, visit_expr, visit_expr_call, visit_expr_method_call, visit_field_value};

use crate::analysis::detector::Detector;
use crate::detectors::implementation::perf_utils::{expr_path_tail, path_to_string};
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
                if fn_item.sig.ident.to_string().starts_with("default_") {
                    continue;
                }

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
    fn visit_item_const(&mut self, _node: &'ast syn::ItemConst) {}

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        let ignored_arg = ignored_numeric_arg(&node.func, node.args.len());
        if let Some(ignored_arg) = ignored_arg {
            self.visit_expr(&node.func);
            for (index, arg) in node.args.iter().enumerate() {
                if index != ignored_arg {
                    self.visit_expr(arg);
                }
            }
            return;
        }

        visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if matches!(
            node.method.to_string().as_str(),
            "get" | "try_get" | "get_ref"
        ) && receiver_is_row_like(&node.receiver)
        {
            self.visit_expr(&node.receiver);
            for (index, arg) in node.args.iter().enumerate() {
                if index != 0 {
                    self.visit_expr(arg);
                }
            }
            return;
        }

        visit_expr_method_call(self, node);
    }

    fn visit_field_value(&mut self, node: &'ast syn::FieldValue) {
        if matches!(&node.member, syn::Member::Named(name) if name == "column") {
            return;
        }

        visit_field_value(self, node);
    }

    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        if let syn::Expr::Lit(lit_expr) = expr
            && let syn::Lit::Int(lit_int) = &lit_expr.lit
            && let Ok(val) = lit_int.base10_parse::<i64>()
            && !WHITELIST.contains(&val)
        {
            self.magic_numbers.push(val);
        }
        visit_expr(self, expr);
    }
}

fn ignored_numeric_arg(func: &syn::Expr, arg_count: usize) -> Option<usize> {
    let syn::Expr::Path(path) = func else {
        return None;
    };

    let call = path_to_string(&path.path);
    let name = call.rsplit("::").next().unwrap_or(call.as_str());
    match name {
        "get_value" | "get_string" | "get_optional_string" | "get_i64" | "get_optional_i64"
            if arg_count >= 2 =>
        {
            Some(1)
        }
        "domain_value" if arg_count >= 2 => Some(arg_count - 1),
        "conversion_error" if arg_count >= 1 => Some(0),
        "parse_uuid"
        | "parse_claims_json"
        | "parse_datetime"
        | "parse_scopes_json"
        | "parse_project_source"
        | "parse_artifact_kind"
        | "parse_provider"
        | "parse_attestation_source"
        | "parse_yank"
            if arg_count >= 2 =>
        {
            Some(arg_count - 1)
        }
        _ => None,
    }
}

fn receiver_is_row_like(expr: &syn::Expr) -> bool {
    expr_path_tail(expr).is_some_and(|tail| tail == "row" || tail.ends_with("_row"))
}
