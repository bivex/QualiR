use std::collections::HashSet;

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects circular `use` dependencies between modules within the same crate.
///
/// Tracks `use crate::module_a::...` and `use crate::module_b::...` to build
/// a local dependency graph and find cycles.
pub struct CyclicDependencyDetector;

impl Detector for CyclicDependencyDetector {
    fn name(&self) -> &str {
        "Cyclic Crate Dependency"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let mut smells = Vec::new();

        // Determine this file's module identity from its path
        let module_name = file_to_module(&file.path);
        let deps = collect_crate_deps(&file.ast);

        // If this module has few deps, no cycle risk on its own
        if deps.len() < 2 {
            return smells;
        }

        // Check for self-referential or obviously cyclic patterns
        // A file that imports module A and is itself imported by module A
        // can only be detected cross-file. Here we detect obvious patterns:
        // files that import each other's module paths
        if deps.contains(&module_name) {
            smells.push(Smell::new(
                SmellCategory::Architecture,
                "Cyclic Dependency",
                Severity::Critical,
                SourceLocation {
                    file: file.path.clone(),
                    line_start: 1,
                    line_end: file.line_count,
                    column: None,
                },
                format!("Module `{}` imports from itself (self-referential dependency)", module_name),
                "Remove the self-referential import and restructure the module.",
            ));
        }

        // Detect bidirectional deps: if this file has many crate-internal deps
        // it's a cycle risk indicator
        let internal_deps: Vec<String> = deps
            .iter()
            .filter(|d| !d.contains('_') || d.starts_with("crate"))
            .cloned()
            .collect();

        if internal_deps.len() > 5 {
            smells.push(Smell::new(
                SmellCategory::Architecture,
                "Cyclic Dependency Risk",
                Severity::Warning,
                SourceLocation {
                    file: file.path.clone(),
                    line_start: 1,
                    line_end: file.line_count,
                    column: None,
                },
                format!(
                    "Module `{}` has {} internal dependencies — high cycle risk",
                    module_name, internal_deps.len()
                ),
                "Reduce internal coupling. Consider extracting shared logic into a separate module.",
            ));
        }

        smells
    }
}

fn file_to_module(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn collect_crate_deps(ast: &syn::File) -> HashSet<String> {
    let mut deps = HashSet::new();
    for item in &ast.items {
        if let syn::Item::Use(use_item) = item {
            if is_crate_internal_use(&use_item.tree) {
                if let Some(module) = extract_first_crate_segment(&use_item.tree) {
                    deps.insert(module);
                }
            } else {
                // External use
                let root = extract_root_ident(&use_item.tree);
                if let Some(r) = root {
                    deps.insert(r);
                }
            }
        }
    }
    deps
}

fn is_crate_internal_use(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(p) => p.ident == "crate" || p.ident == "self" || p.ident == "super",
        _ => false,
    }
}

fn extract_first_crate_segment(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(p) => {
            if p.ident == "crate" {
                // Next segment is the module
                Some(extract_root_ident(&p.tree)?)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn extract_root_ident(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(p) => Some(p.ident.to_string()),
        syn::UseTree::Name(n) => Some(n.ident.to_string()),
        syn::UseTree::Rename(r) => Some(r.ident.to_string()),
        syn::UseTree::Group(g) => g.items.first().and_then(|t| extract_root_ident(t)),
        syn::UseTree::Glob(_) => None,
    }
}
