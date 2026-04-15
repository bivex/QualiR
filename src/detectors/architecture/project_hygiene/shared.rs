use std::path::{Path, PathBuf};

pub fn is_entry_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == "lib.rs" || n == "main.rs")
        .unwrap_or(false)
}

pub fn find_upwards(path: &Path, filename: &str) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        let candidate = ancestor.join(filename);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

pub fn path_has_pair(text: &str, a: &str, b: &str) -> bool {
    text.contains(&format!("{a}::{b}")) || text.contains(&format!("{a} :: {b}"))
}
