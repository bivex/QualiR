use quote::ToTokens;

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects exported types with FFI-shaped names that lack repr(C).
pub struct FfiTypeNotReprCDetector;

impl Detector for FfiTypeNotReprCDetector {
    fn name(&self) -> &str {
        "FFI Type Not repr(C)"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();
        for item in &file.ast.items {
            match item {
                syn::Item::Struct(strukt)
                    if is_public(&strukt.vis)
                        && ffi_named(&strukt.ident.to_string())
                        && !has_repr_c(&strukt.attrs) =>
                {
                    let line = strukt.struct_token.span.start().line;
                    smells.push(smell(file, line, &strukt.ident.to_string()));
                }
                syn::Item::Enum(enm)
                    if is_public(&enm.vis)
                        && ffi_named(&enm.ident.to_string())
                        && !has_repr_c(&enm.attrs) =>
                {
                    let line = enm.enum_token.span.start().line;
                    smells.push(smell(file, line, &enm.ident.to_string()));
                }
                _ => {}
            }
        }
        smells
    }
}

fn smell(file: &SourceFile, line: usize, name: &str) -> Smell {
    Smell::new(
        SmellCategory::Unsafe,
        "FFI Type Not repr(C)",
        Severity::Warning,
        SourceLocation::new(file.path.clone(), line, line, None),
        format!("FFI-facing type `{name}` lacks #[repr(C)]"),
        "Add #[repr(C)] or avoid exposing the type across an FFI boundary.",
    )
}

fn is_public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

fn ffi_named(name: &str) -> bool {
    name.starts_with("C") || name.contains("Ffi") || name.contains("FFI")
}

fn has_repr_c(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("repr") && attr.meta.to_token_stream().to_string().contains('C')
    })
}
