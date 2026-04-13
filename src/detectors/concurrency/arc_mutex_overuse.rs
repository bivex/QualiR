use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::analysis::detector::Detector;
use crate::domain::config::Thresholds;
use crate::domain::smell::{Severity, Smell, SmellCategory, SourceLocation};
use crate::domain::source::SourceFile;

/// Detects overuse of Arc<Mutex<T>> patterns in a file.
///
/// Excessive Arc<Mutex<...>> suggests poor concurrency design where
/// message passing or finer-grained locking would be better.
pub struct ArcMutexOveruseDetector;

impl Detector for ArcMutexOveruseDetector {
    fn name(&self) -> &str {
        "Arc Mutex Overuse"
    }

    fn detect(&self, file: &SourceFile) -> Vec<Smell> {
        let thresholds = Thresholds::default();
        let mut smells = Vec::new();

        let mut visitor = ArcMutexVisitor {
            arc_mutex_count: 0,
            first_line: None,
        };
        syn::visit::visit_file(&mut visitor, &file.ast);

        if visitor.arc_mutex_count > thresholds.arc_mutex_overuse {
            let line = visitor.first_line.unwrap_or(1);
            smells.push(Smell::new(
                SmellCategory::Concurrency,
                "Arc Mutex Overuse",
                Severity::Warning,
                SourceLocation {
                    file: file.path.clone(),
                    line_start: line,
                    line_end: line,
                    column: None,
                },
                format!(
                    "File uses Arc<Mutex<...>> {} times (threshold: {})",
                    visitor.arc_mutex_count, thresholds.arc_mutex_overuse
                ),
                "Consider message passing (channels) or finer-grained locking (RwLock, atomics).",
            ));
        }

        smells
    }
}

struct ArcMutexVisitor {
    arc_mutex_count: usize,
    first_line: Option<usize>,
}

impl<'ast> Visit<'ast> for ArcMutexVisitor {
    fn visit_type(&mut self, node: &'ast syn::Type) {
        if let syn::Type::Path(tp) = node {
            let segments: Vec<String> = tp.path.segments.iter()
                .map(|s| s.ident.to_string())
                .collect();

            if is_arc_mutex(&segments) {
                self.arc_mutex_count += 1;
                if self.first_line.is_none() {
                    self.first_line = Some(tp.path.span().start().line);
                }
            }
        }
        syn::visit::visit_type(self, node);
    }
}

fn is_arc_mutex(segments: &[String]) -> bool {
    segments.iter().any(|s| s == "Arc" || s == "Mutex" || s == "RwLock")
}
