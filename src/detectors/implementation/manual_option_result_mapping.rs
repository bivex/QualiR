use quote::ToTokens;
use syn::visit::{visit_expr_match, Visit};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects manual matches that can usually be written with map/map_err/and_then.
pub struct ManualOptionResultMappingDetector;

impl Detector for ManualOptionResultMappingDetector {
    fn name(&self) -> &str {
        "Manual Option/Result Mapping"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = ManualMappingVisitor {
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|line| {
                Smell::new(
                    SmellCategory::Idiomaticity,
                    "Manual Option/Result Mapping",
                    Severity::Info,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    "Match expression manually maps Option or Result variants",
                    "Use map, map_err, and_then, or the ? operator where it keeps the code clearer.",
                )
            })
            .collect()
    }
}

struct ManualMappingVisitor {
    findings: Vec<usize>,
}

impl<'ast> Visit<'ast> for ManualMappingVisitor {
    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        if node.arms.len() == 2 {
            let arms = node
                .arms
                .iter()
                .map(|arm| {
                    (
                        arm.pat.to_token_stream().to_string(),
                        arm.body.to_token_stream().to_string(),
                    )
                })
                .collect::<Vec<_>>();
            let text = format!("{} {}", arms[0].0, arms[1].0);
            let bodies = format!("{} {}", arms[0].1, arms[1].1);
            let option_map = text.contains("Some")
                && text.contains("None")
                && bodies.contains("Some")
                && bodies.contains("None");
            let result_map = text.contains("Ok")
                && text.contains("Err")
                && bodies.contains("Ok")
                && bodies.contains("Err");
            if option_map || result_map {
                self.findings.push(node.match_token.span.start().line);
            }
        }
        visit_expr_match(self, node);
    }
}
