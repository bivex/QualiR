use std::path::{Path, PathBuf};

use crate::analysis::detector::Detector;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects dev-dependencies imported from production source files.
pub struct TestOnlyDependencyInProductionDetector;

impl Detector for TestOnlyDependencyInProductionDetector {
    fn name(&self) -> &str {
        "Test-only Dependency in Production"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        if is_test_path(&file.path) {
            return Vec::new();
        }
        let Some(manifest) = find_upwards(&file.path, "Cargo.toml") else {
            return Vec::new();
        };
        let dev_deps = dev_dependencies(&manifest);
        if dev_deps.is_empty() {
            return Vec::new();
        }

        let mut smells = Vec::new();
        for item in &file.ast.items {
            if let syn::Item::Use(use_item) = item {
                let used = use_tree_root(&use_item.tree);
                if dev_deps.contains(&used) {
                    let line = use_item.use_token.span.start().line;
                    smells.push(Smell::new(
                        SmellCategory::Architecture,
                        "Test-only Dependency in Production",
                        Severity::Warning,
                        SourceLocation::new(file.path.clone(), line, line, None),
                        format!("Production source imports dev-dependency `{used}`"),
                        "Move the dependency to [dependencies] or keep the import under test-only cfg.",
                    ));
                }
            }
        }
        smells
    }
}

/// Detects duplicate crate versions in Cargo.lock.
pub struct DuplicateDependencyVersionsDetector;

impl Detector for DuplicateDependencyVersionsDetector {
    fn name(&self) -> &str {
        "Duplicate Dependency Versions"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        if !is_entry_file(&file.path) {
            return Vec::new();
        }
        let Some(lockfile) = find_upwards(&file.path, "Cargo.lock") else {
            return Vec::new();
        };
        let Ok(content) = std::fs::read_to_string(lockfile) else {
            return Vec::new();
        };
        let duplicates = duplicate_packages(&content);
        duplicates
            .into_iter()
            .map(|name| {
                Smell::new(
                    SmellCategory::Architecture,
                    "Duplicate Dependency Versions",
                    Severity::Info,
                    SourceLocation::new(file.path.clone(), 1, 1, None),
                    format!("Cargo.lock contains multiple versions of `{name}`"),
                    "Align dependency version requirements to reduce duplicate builds.",
                )
            })
            .collect()
    }
}

/// Detects excessive feature gates in one file.
pub struct FeatureFlagSprawlDetector;

impl Detector for FeatureFlagSprawlDetector {
    fn name(&self) -> &str {
        "Feature Flag Sprawl"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let count = file.code.matches("cfg(feature").count();
        if count > 8 {
            vec![Smell::new(
                SmellCategory::Architecture,
                "Feature Flag Sprawl",
                Severity::Info,
                SourceLocation::new(file.path.clone(), 1, 1, None),
                format!("File contains {count} feature-gated cfgs"),
                "Consolidate feature-specific code behind modules or adapter layers.",
            )]
        } else {
            Vec::new()
        }
    }
}

/// Approximates circular module dependencies by finding reciprocal use paths in one file.
pub struct CircularModuleDependencyDetector;

impl Detector for CircularModuleDependencyDetector {
    fn name(&self) -> &str {
        "Circular Module Dependency"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let text = file.code.to_lowercase();
        let pairs = [
            ("domain", "infrastructure"),
            ("domain", "cli"),
            ("application", "infrastructure"),
            ("service", "repository"),
        ];
        for (a, b) in pairs {
            if path_has_pair(&text, a, b) && path_has_pair(&text, b, a) {
                return vec![Smell::new(
                    SmellCategory::Architecture,
                    "Circular Module Dependency",
                    Severity::Warning,
                    SourceLocation::new(file.path.clone(), 1, 1, None),
                    format!("File references both `{a} -> {b}` and `{b} -> {a}` paths"),
                    "Break the cycle with a trait, adapter, or dependency inversion boundary.",
                )];
            }
        }
        Vec::new()
    }
}

fn is_test_path(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "tests") || path.to_string_lossy().contains("_test")
}

fn is_entry_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == "lib.rs" || n == "main.rs")
        .unwrap_or(false)
}

fn find_upwards(path: &Path, filename: &str) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        let candidate = ancestor.join(filename);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

fn dev_dependencies(manifest: &Path) -> std::collections::HashSet<String> {
    let Ok(content) = std::fs::read_to_string(manifest) else {
        return std::collections::HashSet::new();
    };
    let mut in_dev = false;
    let mut deps = std::collections::HashSet::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_dev = trimmed == "[dev-dependencies]";
            continue;
        }
        if in_dev {
            if let Some((name, _)) = trimmed.split_once('=') {
                let name = name.trim().trim_matches('"').replace('-', "_");
                if !name.is_empty() {
                    deps.insert(name);
                }
            }
        }
    }
    deps
}

fn use_tree_root(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(path) => path.ident.to_string().replace('-', "_"),
        syn::UseTree::Name(name) => name.ident.to_string().replace('-', "_"),
        syn::UseTree::Rename(rename) => rename.ident.to_string().replace('-', "_"),
        syn::UseTree::Group(group) => group.items.first().map(use_tree_root).unwrap_or_default(),
        syn::UseTree::Glob(_) => String::new(),
    }
}

fn duplicate_packages(content: &str) -> Vec<String> {
    let mut versions: std::collections::HashMap<String, std::collections::HashSet<String>> =
        std::collections::HashMap::new();
    let mut current: Option<String> = None;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed.strip_prefix("name = ") {
            current = Some(name.trim_matches('"').to_string());
        } else if let Some(version) = trimmed.strip_prefix("version = ") {
            if let Some(name) = current.take() {
                versions
                    .entry(name)
                    .or_default()
                    .insert(version.trim_matches('"').to_string());
            }
        }
    }
    versions
        .into_iter()
        .filter_map(|(name, versions)| (versions.len() > 1).then_some(name))
        .collect()
}

fn path_has_pair(text: &str, a: &str, b: &str) -> bool {
    text.contains(&format!("{a}::{b}")) || text.contains(&format!("{a} :: {b}"))
}
