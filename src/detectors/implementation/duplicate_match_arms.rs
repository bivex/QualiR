use quote::ToTokens;
use syn::visit::{visit_expr_match, Visit};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects match arms with repeated bodies.
pub struct DuplicateMatchArmsDetector;

impl Detector for DuplicateMatchArmsDetector {
    fn name(&self) -> &str {
        "Duplicate Match Arms"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = DuplicateArmVisitor {
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|line| {
                Smell::new(
                    SmellCategory::Implementation,
                    "Duplicate Match Arms",
                    Severity::Info,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    "Match expression has duplicate arm bodies",
                    "Combine equivalent arms with `|` patterns or extract the repeated body.",
                )
            })
            .collect()
    }
}

struct DuplicateArmVisitor {
    findings: Vec<usize>,
}

impl<'ast> Visit<'ast> for DuplicateArmVisitor {
    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        let mut seen = std::collections::HashSet::new();
        for arm in &node.arms {
            if is_bound_field_projection(&arm.pat, &arm.body) {
                continue;
            }
            let body = normalized_body(&arm.body);
            if body.len() > 3 && !seen.insert(body) {
                self.findings
                    .push(arm.fat_arrow_token.spans[0].start().line);
                break;
            }
        }
        visit_expr_match(self, node);
    }
}

fn is_bound_field_projection(pat: &syn::Pat, body: &syn::Expr) -> bool {
    field_projection_base_ident(body)
        .map(|ident| pattern_binds_ident(pat, ident))
        .unwrap_or(false)
}

fn field_projection_base_ident(expr: &syn::Expr) -> Option<&syn::Ident> {
    match expr {
        syn::Expr::Field(field) => match &*field.base {
            syn::Expr::Path(path) => path.path.get_ident(),
            expr => field_projection_base_ident(expr),
        },
        syn::Expr::Reference(reference) => field_projection_base_ident(&reference.expr),
        syn::Expr::Paren(paren) => field_projection_base_ident(&paren.expr),
        _ => None,
    }
}

fn pattern_binds_ident(pat: &syn::Pat, ident: &syn::Ident) -> bool {
    match pat {
        syn::Pat::Ident(pat_ident) => pat_ident.ident == *ident,
        syn::Pat::Or(or) => or.cases.iter().any(|pat| pattern_binds_ident(pat, ident)),
        syn::Pat::Paren(paren) => pattern_binds_ident(&paren.pat, ident),
        syn::Pat::Reference(reference) => pattern_binds_ident(&reference.pat, ident),
        syn::Pat::Slice(slice) => pat_elems_bind_ident(&slice.elems, ident),
        syn::Pat::Struct(strukt) => strukt
            .fields
            .iter()
            .any(|field| pattern_binds_ident(&field.pat, ident)),
        syn::Pat::Tuple(tuple) => tuple_pat_binds_ident(tuple, ident),
        syn::Pat::TupleStruct(tuple) => tuple_struct_pat_binds_ident(tuple, ident),
        syn::Pat::Type(typed) => pattern_binds_ident(&typed.pat, ident),
        _ => false,
    }
}

fn pat_elems_bind_ident(
    elems: &syn::punctuated::Punctuated<syn::Pat, syn::token::Comma>,
    ident: &syn::Ident,
) -> bool {
    elems.iter().any(|pat| pattern_binds_ident(pat, ident))
}

fn tuple_pat_binds_ident(tuple: &syn::PatTuple, ident: &syn::Ident) -> bool {
    pat_elems_bind_ident(&tuple.elems, ident)
}

fn tuple_struct_pat_binds_ident(tuple: &syn::PatTupleStruct, ident: &syn::Ident) -> bool {
    pat_elems_bind_ident(&tuple.elems, ident)
}

fn normalized_body(body: &syn::Expr) -> String {
    normalize(&body.to_token_stream().to_string())
}

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<String>()
}
