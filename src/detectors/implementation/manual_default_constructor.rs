use quote::ToTokens;

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects `new()` constructors that simply return default field values.
pub struct ManualDefaultConstructorDetector;

impl Detector for ManualDefaultConstructorDetector {
    fn name(&self) -> &str {
        "Manual Default Constructor"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Impl(imp) = item {
                for impl_item in &imp.items {
                    if let syn::ImplItem::Fn(func) = impl_item {
                        if func.sig.ident == "new"
                            && returns_self(&func.sig.output)
                            && body_is_defaultish(&func.block)
                        {
                            let line = func.sig.fn_token.span.start().line;
                            smells.push(Smell::new(
                                SmellCategory::Idiomaticity,
                                "Manual Default Constructor",
                                Severity::Info,
                                SourceLocation::new(file.path.clone(), line, line, None),
                                "Constructor `new` appears to return only default field values",
                                "Implement or derive Default and delegate `new()` to `Self::default()`.",
                            ));
                        }
                    }
                }
            }
        }

        smells
    }
}

fn returns_self(output: &syn::ReturnType) -> bool {
    matches!(output, syn::ReturnType::Type(_, ty) if matches!(&**ty, syn::Type::Path(path) if path.path.is_ident("Self")))
}

fn body_is_defaultish(block: &syn::Block) -> bool {
    let text = block.to_token_stream().to_string();
    (text.contains("Self") || text.contains("default"))
        && (text.contains("Default :: default")
            || text.contains("Self :: default")
            || text.contains("String :: new")
            || text.contains("Vec :: new"))
        && !text.contains("return")
}
