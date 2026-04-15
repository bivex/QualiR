use syn::visit::{Visit, visit_expr_for_loop, visit_expr_loop, visit_expr_while};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects Regex::new in functions and loops.
pub struct RepeatedRegexConstructionDetector;

impl Detector for RepeatedRegexConstructionDetector {
    fn name(&self) -> &str {
        "Repeated Regex Construction"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = RegexVisitor {
            loop_depth: 0,
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|(line, in_loop)| {
                Smell::new(
                    SmellCategory::Performance,
                    "Repeated Regex Construction",
                    if in_loop {
                        Severity::Warning
                    } else {
                        Severity::Info
                    },
                    SourceLocation::new(file.path.clone(), line, line, None),
                    "Regex is constructed at runtime".to_string(),
                    "Store regexes in LazyLock, OnceLock, or lazy_static when they are reused.",
                )
            })
            .collect()
    }
}

struct RegexVisitor {
    loop_depth: usize,
    findings: Vec<(usize, bool)>,
}

impl<'ast> Visit<'ast> for RegexVisitor {
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

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func {
            let path_str = path
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            if path_str.ends_with("Regex::new") {
                let line = path.path.segments.last().unwrap().ident.span().start().line;
                self.findings.push((line, self.loop_depth > 0));
            }
        }
        syn::visit::visit_expr_call(self, node);
    }
}
