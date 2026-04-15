use syn::visit::Visit;

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects usage of the `#[async_trait]` macro.
///
/// Under modern Rust (>= 1.75), native async traits are stabilized.
/// The `async_trait` macro introduces a performance penalty by boxing the Future
/// and using dynamic dispatch, which is often unnecessary now.
pub struct AsyncTraitOverheadDetector;

impl Detector for AsyncTraitOverheadDetector {
    fn name(&self) -> &str {
        "Async Trait Overhead"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        let mut visitor = MacroAttributeVisitor { violations: Vec::new() };
        visitor.visit_file(&file.ast);

        for line in visitor.violations {
            smells.push(Smell::new(
                SmellCategory::Performance,
                "Async Trait Overhead",
                Severity::Info,
                SourceLocation::new(file.path.clone(), line, line, None),
                "Usage of `#[async_trait]` macro incurs unnecessary Future boxing overhead".to_string(),
                "Migrate to native async fn in traits (stabilized in Rust 1.75) if possible.",
            ));
        }

        smells
    }
}

struct MacroAttributeVisitor {
    violations: Vec<usize>,
}

impl<'ast> Visit<'ast> for MacroAttributeVisitor {
    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        if let Some(seg) = node.path().segments.last() {
            if seg.ident == "async_trait" {
                // Approximate line number since span might encompass the whole attribute
                let line = seg.ident.span().start().line;
                self.violations.push(line);
            }
        }
        syn::visit::visit_attribute(self, node);
    }
}
