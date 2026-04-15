use quote::ToTokens;
use syn::visit::{visit_item_fn, Visit};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Heuristically detects lock guards that may live across an await point.
pub struct HoldingLockAcrossAwaitDetector;

impl Detector for HoldingLockAcrossAwaitDetector {
    fn name(&self) -> &str {
        "Holding Lock Across Await"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = LockAwaitVisitor {
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|line| {
                Smell::new(
                    SmellCategory::Concurrency,
                    "Holding Lock Across Await",
                    Severity::Critical,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    "Async function appears to hold a lock across an await point",
                    "Drop the guard before awaiting or move the awaited work outside the critical section.",
                )
            })
            .collect()
    }
}

struct LockAwaitVisitor {
    findings: Vec<usize>,
}

impl<'ast> Visit<'ast> for LockAwaitVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if node.sig.asyncness.is_some() {
            let text = node.block.to_token_stream().to_string();
            let lock_pos = text
                .find(". lock (")
                .or_else(|| text.find(". write ("))
                .or_else(|| text.find(". read ("));
            let await_pos = text.find(". await");
            if matches!((lock_pos, await_pos), (Some(lock), Some(await_)) if lock < await_) {
                self.findings.push(node.sig.fn_token.span.start().line);
            }
        }
        visit_item_fn(self, node);
    }
}
