use syn::visit::{visit_expr_for_loop, visit_expr_loop, visit_expr_while, Visit};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects allocation-heavy calls inside loops.
pub struct UnnecessaryAllocationInLoopDetector;

impl Detector for UnnecessaryAllocationInLoopDetector {
    fn name(&self) -> &str {
        "Unnecessary Allocation in Loop"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut visitor = AllocationLoopVisitor {
            loop_depth: 0,
            findings: Vec::new(),
        };
        visitor.visit_file(&file.ast);

        visitor
            .findings
            .into_iter()
            .map(|(line, call)| {
                Smell::new(
                    SmellCategory::Performance,
                    "Unnecessary Allocation in Loop",
                    Severity::Info,
                    SourceLocation::new(file.path.clone(), line, line, None),
                    format!("Allocation-like call `{call}` appears inside a loop"),
                    "Move reusable allocation outside the loop or borrow data where possible.",
                )
            })
            .collect()
    }
}

struct AllocationLoopVisitor {
    loop_depth: usize,
    findings: Vec<(usize, String)>,
}

impl<'ast> Visit<'ast> for AllocationLoopVisitor {
    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.loop_depth += 1;
        visit_expr_for_loop(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.loop_depth += 1;
        visit_expr_while(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.loop_depth += 1;
        visit_expr_loop(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if self.loop_depth > 0 && is_finding_push(node) {
            return;
        }

        if self.loop_depth > 0 {
            let method = node.method.to_string();
            if method == "to_owned" {
                self.findings
                    .push((node.method.span().start().line, method));
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if self.loop_depth > 0 && is_diagnostic_constructor(node) {
            return;
        }

        if self.loop_depth > 0 {
            if let syn::Expr::Path(path) = &*node.func {
                let call = path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                if call == "String::from" {
                    self.findings.push((
                        path.path.segments.last().unwrap().ident.span().start().line,
                        call,
                    ));
                }
            }
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_reference(&mut self, node: &'ast syn::ExprReference) {
        if self.loop_depth > 0
            && matches!(&*node.expr, syn::Expr::Macro(expr) if expr.mac.path.is_ident("format"))
        {
            return;
        }

        syn::visit::visit_expr_reference(self, node);
    }

    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        if self.loop_depth > 0 && is_visitor_struct(&node.path) {
            return;
        }

        syn::visit::visit_expr_struct(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if self.loop_depth > 0 {
            if node.path.is_ident("format") {
                self.findings.push((
                    node.path.segments[0].ident.span().start().line,
                    "format!".into(),
                ));
            }
        }
        syn::visit::visit_macro(self, node);
    }
}

fn is_diagnostic_constructor(node: &syn::ExprCall) -> bool {
    let syn::Expr::Path(path) = &*node.func else {
        return false;
    };

    let mut segments = path.path.segments.iter().rev();
    matches!(
        (segments.next(), segments.next()),
        (Some(method), Some(receiver))
            if method.ident == "new"
                && (receiver.ident == "Smell" || receiver.ident == "SourceLocation")
    )
}

fn is_visitor_struct(path: &syn::Path) -> bool {
    path.segments
        .last()
        .map(|segment| segment.ident.to_string().ends_with("Visitor"))
        .unwrap_or(false)
}

fn is_finding_push(node: &syn::ExprMethodCall) -> bool {
    if node.method != "push" {
        return false;
    }

    receiver_path_tail(&node.receiver).is_some_and(|name| {
        name == "smells"
            || name == "findings"
            || name == "usages"
            || name == "blocking_calls"
            || name == "lock_calls"
            || name == "spawns"
            || name == "parts"
    })
}

fn receiver_path_tail(expr: &syn::Expr) -> Option<&syn::Ident> {
    match expr {
        syn::Expr::Path(path) => path.path.segments.last().map(|segment| &segment.ident),
        syn::Expr::Field(field) => match &field.member {
            syn::Member::Named(name) => Some(name),
            syn::Member::Unnamed(_) => None,
        },
        _ => None,
    }
}
