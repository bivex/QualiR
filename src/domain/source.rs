use std::path::PathBuf;

/// A parsed Rust source file with its AST and metadata.
pub struct SourceFile {
    pub path: PathBuf,
    pub code: String,
    pub ast: syn::File,
    pub line_count: usize,
}

impl SourceFile {
    /// Parse a Rust source file from disk.
    pub fn from_path(path: PathBuf) -> Result<Self, ParseError> {
        let code = std::fs::read_to_string(&path)
            .map_err(|e| ParseError::Io(path.clone(), e))?;
        Self::from_source(path, code)
    }

    /// Parse Rust source code into an AST.
    pub fn from_source(path: PathBuf, code: String) -> Result<Self, ParseError> {
        let ast = syn::parse_file(&code)
            .map_err(|e| ParseError::Syntax(path.clone(), e))?;
        let line_count = code.lines().count();
        Ok(Self { path, code, ast, line_count })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("IO error reading {0}: {1}")]
    Io(PathBuf, std::io::Error),
    #[error("Syntax error in {0}: {1}")]
    Syntax(PathBuf, syn::Error),
}
