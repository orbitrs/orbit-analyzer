// Static analysis tool for Orbit UI framework files

mod linter;
mod parser;
mod reporter;
mod rules;

use thiserror::Error;

/// Export public API
pub use linter::Linter;
pub use reporter::{Issue, Reporter, Severity};
pub use rules::Rule;

/// Version of the Orbit Analyzer
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Errors that can occur in the Orbit Analyzer
#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Rule error: {0}")]
    Rule(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for Orbit Analyzer operations
pub type Result<T> = std::result::Result<T, AnalyzerError>;

/// Analyze an .orbit file for issues
pub fn analyze_file(file_path: &str) -> Result<Vec<reporter::Issue>> {
    let content = std::fs::read_to_string(file_path)?;
    let linter = Linter::new();
    linter.lint(&content, file_path)
}
