use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects structs that have fields but no associated methods (impl blocks).
///
/// This often indicates data-only types with logic scattered elsewhere.
pub struct AnemicStructDetector;

impl Detector for AnemicStructDetector {
    fn name(&self) -> &str {
        "Anemic Struct"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        // Collect struct names that have at least one field
        let structs_with_fields: Vec<&syn::ItemStruct> = file
            .ast
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Struct(s) => has_fields(s).then_some(s),
                _ => None,
            })
            .collect();

        if structs_with_fields.is_empty() {
            return smells;
        }

        // Collect struct names that have impl blocks in this file
        let impl_targets: Vec<String> = file
            .ast
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Impl(imp) if imp.trait_.is_none() => {
                    extract_type_ident(&imp.self_ty)
                }
                _ => None,
            })
            .collect();

        for s in &structs_with_fields {
            let has_impl = impl_targets.iter().any(|id| *id == s.ident.to_string());
            if !has_impl {
                let line = line_of_struct(s);

                smells.push(Smell::new(
                    SmellCategory::Design,
                    "Anemic Struct",
                    Severity::Info,
                    SourceLocation {
                        file: file.path.clone(),
                        line_start: line,
                        line_end: line,
                        column: None,
                    },
                    format!("Struct `{}` has fields but no impl block in this file", s.ident),
                    "Consider adding behavior to the struct or convert to a value object if data-only is intentional.",
                ));
            }
        }

        smells
    }
}

fn has_fields(s: &syn::ItemStruct) -> bool {
    match &s.fields {
        syn::Fields::Named(f) => !f.named.is_empty(),
        syn::Fields::Unnamed(f) => !f.unnamed.is_empty(),
        syn::Fields::Unit => false,
    }
}

fn extract_type_ident(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(tp) = ty {
        tp.path.segments.first().map(|s| s.ident.to_string())
    } else {
        None
    }
}

fn line_of_struct(s: &syn::ItemStruct) -> usize {
    match &s.fields {
        syn::Fields::Named(f) => f.brace_token.span.open().start().line,
        syn::Fields::Unnamed(f) => f.paren_token.span.open().start().line,
        syn::Fields::Unit => s.struct_token.span.start().line,
    }
}
