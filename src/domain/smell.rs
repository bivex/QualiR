use std::fmt;
use std::path::PathBuf;

/// High-level category grouping related smells together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SmellCategory {
    Architecture,
    Design,
    Implementation,
    Performance,
    Idiomaticity,
    Concurrency,
    Unsafe,
}

impl fmt::Display for SmellCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Architecture => write!(f, "Architecture"),
            Self::Design => write!(f, "Design"),
            Self::Implementation => write!(f, "Implementation"),
            Self::Performance => write!(f, "Performance"),
            Self::Idiomaticity => write!(f, "Idiomaticity"),
            Self::Concurrency => write!(f, "Concurrency"),
            Self::Unsafe => write!(f, "Unsafe"),
        }
    }
}

impl std::str::FromStr for SmellCategory {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_lowercase().as_str() {
            "architecture" | "arch" => Ok(Self::Architecture),
            "design" => Ok(Self::Design),
            "implementation" | "impl" => Ok(Self::Implementation),
            "performance" | "perf" => Ok(Self::Performance),
            "idiomaticity" | "idiomatic" | "idiom" => Ok(Self::Idiomaticity),
            "concurrency" | "concurrent" => Ok(Self::Concurrency),
            "unsafe" => Ok(Self::Unsafe),
            other => Err(format!(
                "Unknown category: {other}. Use: architecture, design, implementation, performance, idiomaticity, concurrency, unsafe"
            )),
        }
    }
}

/// How severe a detected smell is.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    #[serde(alias = "Info")]
    Info,
    #[serde(alias = "Warning")]
    Warning,
    #[serde(alias = "Critical")]
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_smell_categories() {
        assert_eq!(
            "architecture".parse::<SmellCategory>(),
            Ok(SmellCategory::Architecture)
        );
        assert_eq!("design".parse::<SmellCategory>(), Ok(SmellCategory::Design));
        assert_eq!(
            "implementation".parse::<SmellCategory>(),
            Ok(SmellCategory::Implementation)
        );
        assert_eq!(
            "performance".parse::<SmellCategory>(),
            Ok(SmellCategory::Performance)
        );
        assert_eq!(
            "idiomaticity".parse::<SmellCategory>(),
            Ok(SmellCategory::Idiomaticity)
        );
        assert_eq!(
            "concurrency".parse::<SmellCategory>(),
            Ok(SmellCategory::Concurrency)
        );
        assert_eq!("unsafe".parse::<SmellCategory>(), Ok(SmellCategory::Unsafe));
    }

    #[test]
    fn parses_common_category_aliases() {
        assert_eq!(
            "arch".parse::<SmellCategory>(),
            Ok(SmellCategory::Architecture)
        );
        assert_eq!(
            "impl".parse::<SmellCategory>(),
            Ok(SmellCategory::Implementation)
        );
        assert_eq!(
            "perf".parse::<SmellCategory>(),
            Ok(SmellCategory::Performance)
        );
        assert_eq!(
            "idiom".parse::<SmellCategory>(),
            Ok(SmellCategory::Idiomaticity)
        );
        assert_eq!(
            "idiomatic".parse::<SmellCategory>(),
            Ok(SmellCategory::Idiomaticity)
        );
    }
}
