use syn::visit::Visit;

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects spawned tasks/threads where the JoinHandle is not used.
///
/// Discarding JoinHandle means fire-and-forget spawning, which can silently
/// lose errors and panic propagation.
pub struct SpawnWithoutJoinDetector;

impl Detector for SpawnWithoutJoinDetector {
    fn name(&self) -> &str {
        "Spawn Without Join"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        for item in &file.ast.items {
            if let syn::Item::Fn(fn_item) = item {
                let mut visitor = SpawnVisitor {
                    spawns: Vec::new(),
                };
                visitor.visit_item_fn(fn_item);

                for (spawn_call, line) in &visitor.spawns {
                    smells.push(Smell::new(
                        SmellCategory::Concurrency,
                        "Spawn Without Join",
                        Severity::Warning,
                        SourceLocation {
                            file: file.path.clone(),
                            line_start: *line,
                            line_end: *line,
                            column: None,
                        },
                        format!(
                            "Function `{}` spawns with `{}` but discards JoinHandle",
                            fn_item.sig.ident, spawn_call
                        ),
                        "Store the JoinHandle and await/join it. Use spawn_blocking or abort on drop if intentional.",
                    ));
                }
            }
        }

        smells
    }
}

struct SpawnVisitor {
    spawns: Vec<(String, usize)>,
}

impl<'ast> Visit<'ast> for SpawnVisitor {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        // Check for `let _ = spawn(...)` pattern
        if let Some(init) = &local.init {
            if is_underscore_binding(&local.pat) {
                if let syn::Expr::Call(call) = &*init.expr {
                    if let syn::Expr::Path(path) = &*call.func {
                        let func_str = path_to_string(&path.path);
                        if is_spawn(&func_str) {
                            let line = local.let_token.span.start().line;
                            self.spawns.push((func_str, line));
                            // We return early to avoid visit_expr catching this same call
                            return;
                        }
                    }
                }
            }
        }
        syn::visit::visit_local(self, local);
    }

    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        // Check for bare spawn(...) calls
        if let syn::Expr::Call(call) = expr {
            if let syn::Expr::Path(path) = &*call.func {
                let func_str = path_to_string(&path.path);
                if is_spawn(&func_str) {
                    let line = call.paren_token.span.open().start().line;
                    // Check if we already have this line (simple heuristic to avoid double counts)
                    if !self.spawns.iter().any(|(_, l)| *l == line) {
                        self.spawns.push((func_str, line));
                    }
                }
            }
        }
        syn::visit::visit_expr(self, expr);
    }
}

fn is_underscore_binding(pat: &syn::Pat) -> bool {
    match pat {
        syn::Pat::Wild(_) => true,
        syn::Pat::Ident(pi) => pi.ident == "_",
        _ => false,
    }
}

fn is_spawn(func: &str) -> bool {
    let lower = func.to_lowercase();
    lower.contains("spawn") || lower.contains("thread::spawn") || lower.contains("tokio::spawn")
        || lower.contains("async_std::task::spawn") || lower.contains("std::thread::spawn")
}

fn path_to_string(path: &syn::Path) -> String {
    let idents: Vec<String> = path.segments.iter()
        .map(|s| s.ident.to_string())
        .collect();
    idents.join("::")
}
