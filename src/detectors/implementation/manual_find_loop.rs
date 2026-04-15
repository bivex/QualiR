use quote::ToTokens;
use syn::visit::{visit_expr_for_loop, Visit};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects loops that look like manual find/any/all operations.
pub struct ManualFindLoopDetector;

impl Detector for ManualFindLoopDetector {
    fn name(&self) -> &str {
        "Manual Find/Any Loop"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = ManualFindVisitor {
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|line| {
                Smell::new(
                    SmellCategory::Idiomaticity,
                    "Manual Find/Any Loop",
                    Severity::Info,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    "Loop returns early based on a predicate",
                    "Consider iterator adapters such as find, any, all, or position.",
                )
            })
            .collect()
    }
}

struct ManualFindVisitor {
    findings: Vec<usize>,
}

impl<'ast> Visit<'ast> for ManualFindVisitor {
    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        let text = node.body.to_token_stream().to_string();
        if text.contains("return") && text.contains("if") {
            self.findings.push(node.for_token.span.start().line);
        }
        visit_expr_for_loop(self, node);
    }
}
