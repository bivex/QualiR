use std::path::{Path, PathBuf};

/// Discovers `.rs` files in a directory tree, respecting exclude patterns.
pub struct RustFileWalker<'a> {
    root: &'a Path,
    excludes: &'a [String],
}

impl<'a> RustFileWalker<'a> {
    pub fn new(root: &'a Path, excludes: &'a [String]) -> Self {
        Self { root, excludes }
    }

    /// Collect all Rust source file paths under the root directory.
    pub fn collect_files(&self) -> Vec<PathBuf> {
        let mut builder = ignore::WalkBuilder::new(self.root);
        builder.hidden(true).git_ignore(true).git_exclude(true);

        // Add exclude patterns via a custom override
        let mut overrides = ignore::overrides::OverrideBuilder::new(self.root);
        for excl in self.excludes {
            let _ = overrides.add(&format!("!{excl}"));
        }
        if let Ok(built) = overrides.build() {
            builder.overrides(built);
        }

        builder
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().map_or(false, |ft| ft.is_file())
                    && entry.path().extension().map_or(false, |ext| ext == "rs")
            })
            .map(|entry| entry.into_path())
            .collect()
    }
}
