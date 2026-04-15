use syn::visit::{
    Visit, visit_expr_closure, visit_expr_for_loop, visit_expr_loop, visit_expr_method_call,
    visit_expr_while, visit_item_fn, visit_item_mod,
};

use crate::analysis::detector::Detector;
use crate::detectors::policy::has_test_cfg;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects repeated string conversions in loops and iterator adapter closures.
pub struct RepeatedStringConversionDetector;

impl Detector for RepeatedStringConversionDetector {
    fn name(&self) -> &str {
        "Repeated String Conversion in Hot Path"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = StringConversionVisitor {
            loop_depth: 0,
            iterator_closure_depth: 0,
            pending_iterator_closure: 0,
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|(line, method)| {
                Smell::new(
                    SmellCategory::Performance,
                    "Repeated String Conversion in Hot Path",
                    Severity::Info,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    format!("Repeated `.{method}()` allocation appears in a loop or iterator chain"),
                    "Keep values borrowed where possible, or delay formatting until an owned String is actually needed.",
                )
            })
            .collect()
    }
}

struct StringConversionVisitor {
    loop_depth: usize,
    iterator_closure_depth: usize,
    pending_iterator_closure: usize,
    findings: Vec<(usize, String)>,
}

impl<'ast> Visit<'ast> for StringConversionVisitor {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if has_test_cfg(&node.attrs) {
            return;
        }
        visit_item_mod(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if has_test_cfg(&node.attrs) {
            return;
        }
        visit_item_fn(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.loop_depth += 1;
        visit_expr_for_loop(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.loop_depth += 1;
        visit_expr_while(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.loop_depth += 1;
        visit_expr_loop(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let method = node.method.to_string();
        let is_hot_conversion = matches!(method.as_str(), "to_string" | "to_owned")
            && (self.loop_depth > 0 || self.iterator_closure_depth > 0);

        if is_hot_conversion && !receiver_is_string_literal(&node.receiver) {
            self.findings
                .push((node.method.span().start().line, method.clone()));
        }

        if is_iterator_adapter(&method) {
            self.pending_iterator_closure += 1;
            for arg in &node.args {
                self.visit_expr(arg);
            }
            self.pending_iterator_closure -= 1;
            self.visit_expr(&node.receiver);
            return;
        }

        visit_expr_method_call(self, node);
    }

    fn visit_expr_closure(&mut self, node: &'ast syn::ExprClosure) {
        if self.pending_iterator_closure > 0 {
            self.iterator_closure_depth += 1;
            visit_expr_closure(self, node);
            self.iterator_closure_depth -= 1;
        } else {
            visit_expr_closure(self, node);
        }
    }
}

fn is_iterator_adapter(method: &str) -> bool {
    matches!(
        method,
        "map"
            | "filter_map"
            | "flat_map"
            | "for_each"
            | "fold"
            | "try_fold"
            | "reduce"
            | "scan"
            | "partition"
            | "any"
            | "all"
            | "find"
            | "position"
    )
}

fn receiver_is_string_literal(expr: &syn::Expr) -> bool {
    matches!(expr, syn::Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Str(_)))
}
