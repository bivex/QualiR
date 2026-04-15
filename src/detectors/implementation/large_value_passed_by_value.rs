use syn::spanned::Spanned;

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects large arrays and containers passed by value.
pub struct LargeValuePassedByValueDetector;

impl Detector for LargeValuePassedByValueDetector {
    fn name(&self) -> &str {
        "Large Value Passed By Value"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();
        for item in &file.ast.items {
            if let syn::Item::Fn(func) = item {
                for input in &func.sig.inputs {
                    if let syn::FnArg::Typed(arg) = input {
                        if is_large_by_value(&arg.ty) {
                            let line = arg.pat.span().start().line;
                            smells.push(Smell::new(
                                SmellCategory::Performance,
                                "Large Value Passed By Value",
                                Severity::Info,
                                SourceLocation::new(file.path.clone(), line, line, None),
                                format!(
                                    "Function `{}` takes a potentially large value by value",
                                    func.sig.ident
                                ),
                                "Take a reference or a slice unless ownership is required.",
                            ));
                        }
                    }
                }
            }
        }
        smells
    }
}

fn is_large_by_value(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Array(arr) => match &arr.len {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Int(int) => int.base10_parse::<usize>().map(|n| n > 32).unwrap_or(false),
                _ => false,
            },
            _ => false,
        },
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|seg| {
                matches!(
                    seg.ident.to_string().as_str(),
                    "Vec" | "String" | "HashMap" | "BTreeMap" | "HashSet" | "BTreeSet"
                )
            })
            .unwrap_or(false),
        _ => false,
    }
}
