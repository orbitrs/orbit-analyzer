// Reporter for lint issues
use serde::{Deserialize, Serialize};

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Error - must be fixed
    Error,
    /// Warning - should be fixed
    Warning,
    /// Info - suggested improvement
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
        }
    }
}

/// Lint issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Name of the rule that found the issue
    pub rule: String,
    /// Message describing the issue
    pub message: String,
    /// File where the issue was found
    pub file: String,
    /// Line number where the issue was found
    pub line: usize,
    /// Column number where the issue was found
    pub column: usize,
    /// Severity of the issue
    pub severity: Severity,
}

/// Reporter for lint issues
pub struct Reporter {
    /// Format to use when reporting issues
    format: ReportFormat,
}

/// Report format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ReportFormat {
    /// Plain text
    Text,
    /// JSON
    Json,
}

impl Reporter {
    /// Create a new text reporter
    pub fn new_text() -> Self {
        Self {
            format: ReportFormat::Text,
        }
    }

    /// Create a new JSON reporter
    pub fn new_json() -> Self {
        Self {
            format: ReportFormat::Json,
        }
    }

    /// Report issues
    pub fn report_issues(&self, file_path: &str, issues: &[Issue]) {
        match self.format {
            ReportFormat::Text => self.report_text(file_path, issues),
            ReportFormat::Json => self.report_json(file_path, issues),
        }
    }

    /// Report issues in text format
    fn report_text(&self, file_path: &str, issues: &[Issue]) {
        if issues.is_empty() {
            println!("{}: No issues found", file_path);
            return;
        }

        println!("{}: {} issues found", file_path, issues.len());

        for issue in issues {
            println!(
                "  {}:{}:{}: [{}] {} ({})",
                issue.file, issue.line, issue.column, issue.severity, issue.message, issue.rule,
            );
        }
    }

    /// Report issues in JSON format
    fn report_json(&self, _file_path: &str, issues: &[Issue]) {
        let json = serde_json::to_string_pretty(&issues)
            .unwrap_or_else(|_| "Error serializing issues".to_string());

        println!("{}", json);
    }
}
