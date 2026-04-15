use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects impls that are often better expressed with derive.
pub struct DerivableImplDetector;

impl Detector for DerivableImplDetector {
    fn name(&self) -> &str {
        "Derivable Impl"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Impl(imp) = item {
                if let Some((_, trait_path, _)) = &imp.trait_ {
                    if let Some(trait_ident) = trait_path.segments.last().map(|seg| &seg.ident) {
                        if !is_derivable_trait(trait_ident) || imp.items.len() > 2 {
                            continue;
                        }
                        let line = imp.impl_token.span.start().line;
                        smells.push(derivable_impl_smell(file, trait_ident, line));
                    }
                }
            }
        }

        smells
    }
}

fn is_derivable_trait(ident: &syn::Ident) -> bool {
    ident == "Debug"
        || ident == "Clone"
        || ident == "Default"
        || ident == "PartialEq"
        || ident == "Eq"
        || ident == "Hash"
}

fn derivable_impl_smell(file: &SourceFile, trait_ident: &syn::Ident, line: usize) -> Smell {
    Smell::new(
        SmellCategory::Idiomaticity,
        "Derivable Impl",
        Severity::Info,
        SourceLocation::new(file.path.clone(), line, line, None),
        format!("Manual `{trait_ident}` impl may be derivable"),
        "Prefer #[derive(...)] when the implementation is mechanical.",
    )
}
