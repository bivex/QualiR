use std::collections::HashMap;

use syn::visit::{
    Visit, visit_expr_async, visit_expr_await, visit_expr_break, visit_expr_closure,
    visit_expr_continue, visit_expr_for_loop, visit_expr_if, visit_expr_loop, visit_expr_macro,
    visit_expr_match, visit_expr_method_call, visit_expr_unsafe, visit_expr_while,
    visit_impl_item_fn, visit_item_fn, visit_item_impl, visit_item_mod,
};

use crate::analysis::detector::Detector;
use crate::detectors::policy::has_test_cfg;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

const MAX_INLINE_BODY_LINES: usize = 8;
const MAX_INLINE_STATEMENTS: usize = 3;
const MIN_CALL_SITES: usize = 3;

/// Detects tiny hot helpers that may benefit from an explicit inline hint.
pub struct InlineCandidateDetector;

impl Detector for InlineCandidateDetector {
    fn name(&self) -> &str {
        "Inline Candidate"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut collector = CandidateCollector {
            candidates: Vec::new(),
        };
        collector.visit_file(&file.ast);

        let mut counter = CallCounter::default();
        counter.visit_file(&file.ast);

        collector
            .candidates
            .into_iter()
            .filter_map(|candidate| {
                let call_sites = counter.call_sites(&candidate);
                (call_sites >= MIN_CALL_SITES).then(|| {
                    Smell::new(
                        SmellCategory::Performance,
                        "Inline Candidate",
                        if call_sites >= MIN_CALL_SITES * 2 {
                            Severity::Warning
                        } else {
                            Severity::Info
                        },
                        SourceLocation::new(file.path.clone(), candidate.line, candidate.line, None),
                        format!(
                            "Function `{}` is tiny and called {call_sites} times in this file",
                            candidate.display_name
                        ),
                        "Consider #[inline] for small hot helpers after profiling confirms call overhead.",
                    )
                })
            })
            .collect()
    }
}

struct InlineCandidate {
    name: String,
    display_name: String,
    line: usize,
    call_kind: CallKind,
}

#[derive(Clone, Copy)]
enum CallKind {
    Function,
    Method,
}

struct CandidateCollector {
    candidates: Vec<InlineCandidate>,
}

impl<'ast> Visit<'ast> for CandidateCollector {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if has_test_cfg(&node.attrs) {
            return;
        }
        visit_item_mod(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if has_test_cfg(&node.attrs) {
            return;
        }

        if let Some(candidate) = candidate_from_item_fn(node) {
            self.candidates.push(candidate);
        }
        visit_item_fn(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if has_test_cfg(&node.attrs) || node.trait_.is_some() {
            return;
        }

        for item in &node.items {
            if let syn::ImplItem::Fn(method) = item
                && !has_test_cfg(&method.attrs)
                && let Some(candidate) = candidate_from_impl_fn(method)
            {
                self.candidates.push(candidate);
            }
        }

        visit_item_impl(self, node);
    }
}

#[derive(Default)]
struct CallCounter {
    function_calls: HashMap<String, usize>,
    method_calls: HashMap<String, usize>,
}

impl CallCounter {
    fn call_sites(&self, candidate: &InlineCandidate) -> usize {
        match candidate.call_kind {
            CallKind::Function => self
                .function_calls
                .get(&candidate.name)
                .copied()
                .unwrap_or(0),
            CallKind::Method => self.method_calls.get(&candidate.name).copied().unwrap_or(0),
        }
    }
}

impl<'ast> Visit<'ast> for CallCounter {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if has_test_cfg(&node.attrs) {
            return;
        }
        visit_item_mod(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if has_test_cfg(&node.attrs) {
            return;
        }
        visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if has_test_cfg(&node.attrs) {
            return;
        }
        visit_impl_item_fn(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func
            && let Some(segment) = path.path.segments.last()
        {
            *self
                .function_calls
                .entry(segment.ident.to_string())
                .or_default() += 1;
        }

        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        *self
            .method_calls
            .entry(node.method.to_string())
            .or_default() += 1;
        visit_expr_method_call(self, node);
    }
}

fn candidate_from_item_fn(func: &syn::ItemFn) -> Option<InlineCandidate> {
    let name = func.sig.ident.to_string();
    is_inline_candidate(&func.attrs, &func.sig, &func.block).then(|| InlineCandidate {
        display_name: name.clone(),
        name,
        line: func.sig.ident.span().start().line,
        call_kind: CallKind::Function,
    })
}

fn candidate_from_impl_fn(func: &syn::ImplItemFn) -> Option<InlineCandidate> {
    let name = func.sig.ident.to_string();
    is_inline_candidate(&func.attrs, &func.sig, &func.block).then(|| InlineCandidate {
        display_name: name.clone(),
        name,
        line: func.sig.ident.span().start().line,
        call_kind: if has_receiver(&func.sig) {
            CallKind::Method
        } else {
            CallKind::Function
        },
    })
}

fn is_inline_candidate(attrs: &[syn::Attribute], sig: &syn::Signature, block: &syn::Block) -> bool {
    if has_exclusion_attr(attrs) || sig.asyncness.is_some() || sig.unsafety.is_some() {
        return false;
    }

    if block.stmts.is_empty() || block.stmts.len() > MAX_INLINE_STATEMENTS {
        return false;
    }

    let body_lines = body_line_count(block);
    body_lines <= MAX_INLINE_BODY_LINES && block_is_simple(block)
}

fn body_line_count(block: &syn::Block) -> usize {
    let open_line = block.brace_token.span.open().start().line;
    let close_line = block.brace_token.span.close().start().line;
    close_line.saturating_sub(open_line).saturating_add(1)
}

fn block_is_simple(block: &syn::Block) -> bool {
    let mut visitor = ComplexityVisitor::default();
    visitor.visit_block(block);
    visitor.complex_nodes == 0
}

#[derive(Default)]
struct ComplexityVisitor {
    complex_nodes: usize,
}

impl<'ast> Visit<'ast> for ComplexityVisitor {
    fn visit_item(&mut self, _node: &'ast syn::Item) {
        self.complex_nodes += 1;
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.complex_nodes += 1;
        visit_expr_if(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.complex_nodes += 1;
        visit_expr_match(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.complex_nodes += 1;
        visit_expr_for_loop(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.complex_nodes += 1;
        visit_expr_while(self, node);
    }

    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.complex_nodes += 1;
        visit_expr_loop(self, node);
    }

    fn visit_expr_closure(&mut self, node: &'ast syn::ExprClosure) {
        self.complex_nodes += 1;
        visit_expr_closure(self, node);
    }

    fn visit_expr_async(&mut self, node: &'ast syn::ExprAsync) {
        self.complex_nodes += 1;
        visit_expr_async(self, node);
    }

    fn visit_expr_unsafe(&mut self, node: &'ast syn::ExprUnsafe) {
        self.complex_nodes += 1;
        visit_expr_unsafe(self, node);
    }

    fn visit_expr_await(&mut self, node: &'ast syn::ExprAwait) {
        self.complex_nodes += 1;
        visit_expr_await(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast syn::ExprMacro) {
        self.complex_nodes += 1;
        visit_expr_macro(self, node);
    }

    fn visit_expr_break(&mut self, node: &'ast syn::ExprBreak) {
        self.complex_nodes += 1;
        visit_expr_break(self, node);
    }

    fn visit_expr_continue(&mut self, node: &'ast syn::ExprContinue) {
        self.complex_nodes += 1;
        visit_expr_continue(self, node);
    }
}

fn has_receiver(sig: &syn::Signature) -> bool {
    sig.inputs
        .first()
        .is_some_and(|arg| matches!(arg, syn::FnArg::Receiver(_)))
}

fn has_exclusion_attr(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr_is_named(attr, EXCLUDED_ATTRS) || attr_list_contains(attr, EXCLUDED_ATTRS))
}

const EXCLUDED_ATTRS: &[&str] = &[
    "inline",
    "cold",
    "no_mangle",
    "export_name",
    "test",
    "bench",
    "proc_macro",
    "proc_macro_derive",
    "proc_macro_attribute",
];

fn attr_is_named(attr: &syn::Attribute, names: &[&str]) -> bool {
    attr.path()
        .get_ident()
        .is_some_and(|ident| names.iter().any(|name| ident == name))
}

fn attr_list_contains(attr: &syn::Attribute, names: &[&str]) -> bool {
    let syn::Meta::List(list) = &attr.meta else {
        return false;
    };

    list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .is_ok_and(|metas| metas.iter().any(|meta| meta_matches(meta, names)))
}

fn meta_matches(meta: &syn::Meta, names: &[&str]) -> bool {
    match meta {
        syn::Meta::Path(path) | syn::Meta::List(syn::MetaList { path, .. }) => path
            .get_ident()
            .is_some_and(|ident| names.iter().any(|name| ident == name)),
        syn::Meta::NameValue(name_value) => name_value
            .path
            .get_ident()
            .is_some_and(|ident| names.iter().any(|name| ident == name)),
    }
}
