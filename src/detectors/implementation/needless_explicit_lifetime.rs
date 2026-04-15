use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects simple function signatures where a named lifetime can be elided.
pub struct NeedlessExplicitLifetimeDetector;

impl Detector for NeedlessExplicitLifetimeDetector {
    fn name(&self) -> &str {
        "Needless Explicit Lifetime"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Fn(func) = item {
                if func.sig.generics.lifetimes().count() == 1
                    && has_one_reference_input(&func.sig.inputs)
                {
                    let line = func.sig.fn_token.span.start().line;
                    smells.push(Smell::new(
                        SmellCategory::Idiomaticity,
                        "Needless Explicit Lifetime",
                        Severity::Info,
                        SourceLocation::new(file.path.clone(), line, line, None),
                        format!("Function `{}` appears to use an elidable explicit lifetime", func.sig.ident),
                        "Remove the named lifetime when lifetime elision rules can express the signature.",
                    ));
                }
            }
        }

        smells
    }
}

fn has_one_reference_input(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> bool {
    inputs
        .iter()
        .filter(|input| matches!(input, syn::FnArg::Typed(pat_ty) if matches!(&*pat_ty.ty, syn::Type::Reference(_))))
        .count()
        == 1
}
