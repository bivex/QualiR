use quote::ToTokens;
use syn::visit::{visit_expr_match, Visit};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects match arms with repeated bodies.
pub struct DuplicateMatchArmsDetector;

impl Detector for DuplicateMatchArmsDetector {
    fn name(&self) -> &str {
        "Duplicate Match Arms"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = DuplicateArmVisitor {
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|line| {
                Smell::new(
                    SmellCategory::Implementation,
                    "Duplicate Match Arms",
                    Severity::Info,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    "Match expression has duplicate arm bodies",
                    "Combine equivalent arms with `|` patterns or extract the repeated body.",
                )
            })
            .collect()
    }
}

struct DuplicateArmVisitor {
    findings: Vec<usize>,
}

impl<'ast> Visit<'ast> for DuplicateArmVisitor {
    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        let mut seen = std::collections::HashSet::new();
        for arm in &node.arms {
            let body = normalize(&arm.body.to_token_stream().to_string());
            if body.len() > 3 && !seen.insert(body) {
                self.findings
                    .push(arm.fat_arrow_token.spans[0].start().line);
                break;
            }
        }
        visit_expr_match(self, node);
    }
}

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<String>()
}
