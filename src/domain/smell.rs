use std::fmt;
use std::path::PathBuf;

/// High-level category grouping related smells together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SmellCategory {
    Architecture,
    Design,
    Implementation,
    Concurrency,
    Unsafe,
}

impl fmt::Display for SmellCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Architecture => write!(f, "Architecture"),
            Self::Design => write!(f, "Design"),
            Self::Implementation => write!(f, "Implementation"),
            Self::Concurrency => write!(f, "Concurrency"),
            Self::Unsafe => write!(f, "Unsafe"),
        }
    }
}

/// How severe a detected smell is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARN"),
            Self::Critical => write!(f, "CRIT"),
        }
    }
}

/// Location in source code where a smell was detected.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line_start: usize,
    pub line_end: usize,
    pub column: Option<usize>,
}

impl SourceLocation {
    #[allow(dead_code)]
    pub fn new(file: PathBuf, line_start: usize, line_end: usize, column: Option<usize>) -> Self {
        Self {
            file,
            line_start,
            line_end,
            column,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file.display(), self.line_start)
    }
}

/// A single detected code smell with full context.
#[derive(Debug, Clone)]
pub struct Smell {
    pub category: SmellCategory,
    pub name: String,
    pub severity: Severity,
    pub location: SourceLocation,
    pub message: String,
    pub suggestion: String,
}

impl Smell {
    pub fn new(
        category: SmellCategory,
        name: impl Into<String>,
        severity: Severity,
        location: SourceLocation,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            category,
            name: name.into(),
            severity,
            location,
            message: message.into(),
            suggestion: suggestion.into(),
        }
    }
}
