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
                    let trait_name = trait_path
                        .segments
                        .last()
                        .map(|seg| seg.ident.to_string())
                        .unwrap_or_default();
                    if matches!(
                        trait_name.as_str(),
                        "Debug" | "Clone" | "Default" | "PartialEq" | "Eq" | "Hash"
                    ) && imp.items.len() <= 2
                    {
                        let line = imp.impl_token.span.start().line;
                        smells.push(Smell::new(
                            SmellCategory::Idiomaticity,
                            "Derivable Impl",
                            Severity::Info,
                            SourceLocation::new(file.path.clone(), line, line, None),
                            format!("Manual `{trait_name}` impl may be derivable"),
                            "Prefer #[derive(...)] when the implementation is mechanical.",
                        ));
                    }
                }
            }
        }

        smells
    }
}
