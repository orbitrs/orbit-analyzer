// Static analysis tool for Orbit UI framework files

mod config;
mod linter;
pub mod parser;
mod reporter;
mod rules;

use thiserror::Error;

/// Export public API
pub use config::{AnalyzerSettings, Config, RendererAnalysisConfig, ReporterConfig, RulesConfig};
pub use linter::Linter;
pub use reporter::{Issue, Reporter, Severity};
pub use rules::{
    ComponentNamingRule, NonEmptyTemplateRule, PropTypeRule, PublicFunctionRule,
    RendererCompatibilityRule, Rule, StateVariableRule,
};

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

    #[error("Configuration error: {0}")]
    Config(String),
}

/// Result type for Orbit Analyzer operations
pub type Result<T> = std::result::Result<T, AnalyzerError>;

/// Analyze an .orbit file for issues
pub fn analyze_file(file_path: &str) -> Result<Vec<reporter::Issue>> {
    let content = std::fs::read_to_string(file_path)?;
    let linter = Linter::new();
    linter.lint(&content, file_path)
}

/// Analyze an .orbit file using a specific configuration
pub fn analyze_file_with_config(file_path: &str, config: Config) -> Result<Vec<reporter::Issue>> {
    let content = std::fs::read_to_string(file_path)?;
    let linter = Linter::with_config(config);
    linter.lint(&content, file_path)
}

/// Analyze multiple .orbit files
pub fn analyze_files(file_paths: &[&str]) -> Result<Vec<reporter::Issue>> {
    let linter = Linter::new();
    let file_paths_vec: Vec<&str> = file_paths.to_vec();
    linter.lint_files(&file_paths_vec)
}

/// Analyze multiple .orbit files using a specific configuration
pub fn analyze_files_with_config(
    file_paths: &[&str],
    config: Config,
) -> Result<Vec<reporter::Issue>> {
    let linter = Linter::with_config(config);
    let file_paths_vec: Vec<&str> = file_paths.to_vec();
    linter.lint_files(&file_paths_vec)
}
