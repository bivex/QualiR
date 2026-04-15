use syn::visit::{visit_item_fn, Visit};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects blocking channel operations inside async functions.
pub struct BlockingChannelInAsyncDetector;

impl Detector for BlockingChannelInAsyncDetector {
    fn name(&self) -> &str {
        "Blocking Channel in Async"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = BlockingChannelVisitor {
            in_async: false,
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|line| {
                Smell::new(
                    SmellCategory::Concurrency,
                    "Blocking Channel in Async",
                    Severity::Warning,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    "Blocking channel operation appears inside an async function",
                    "Use async channels or spawn blocking work explicitly.",
                )
            })
            .collect()
    }
}

struct BlockingChannelVisitor {
    in_async: bool,
    findings: Vec<usize>,
}

impl<'ast> Visit<'ast> for BlockingChannelVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let prev = self.in_async;
        self.in_async = node.sig.asyncness.is_some();
        visit_item_fn(self, node);
        self.in_async = prev;
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if self.in_async {
            let method = node.method.to_string();
            if matches!(
                method.as_str(),
                "recv" | "send" | "recv_timeout" | "send_timeout"
            ) {
                self.findings.push(node.method.span().start().line);
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}
