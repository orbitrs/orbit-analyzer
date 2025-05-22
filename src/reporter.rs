// Reporter for lint issues
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub enum Severity {
    /// Error - must be fixed
    Error,
    /// Warning - should be fixed
    #[default]
    Warning,
    /// Info - suggested improvement
    Info,
}

// Custom deserialization to handle case-insensitive strings
impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "error" => Ok(Severity::Error),
            "warning" => Ok(Severity::Warning),
            "info" => Ok(Severity::Info),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown severity level: {}. Expected one of: Error, Warning, Info",
                s
            ))),
        }
    }
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
    /// Output file path
    output_path: Option<String>,
}

/// Report format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ReportFormat {
    /// Plain text
    Text,
    /// JSON
    Json,
    /// HTML
    Html,
}

impl Reporter {
    /// Create a new text reporter
    pub fn new_text() -> Self {
        Self {
            format: ReportFormat::Text,
            output_path: None,
        }
    }

    /// Create a new JSON reporter
    pub fn new_json() -> Self {
        Self {
            format: ReportFormat::Json,
            output_path: None,
        }
    }

    /// Create a new HTML reporter
    pub fn new_html() -> Self {
        Self {
            format: ReportFormat::Html,
            output_path: None,
        }
    }

    /// Set output file path
    pub fn with_output_path(mut self, path: &str) -> Self {
        self.output_path = Some(path.to_string());
        self
    }

    /// Report issues for a single file
    pub fn report_issues(&self, file_path: &str, issues: &[Issue]) {
        let output = match self.format {
            ReportFormat::Text => self.generate_text_report(file_path, issues),
            ReportFormat::Json => self.generate_json_report(issues),
            ReportFormat::Html => self.generate_html_report(file_path, issues),
        };

        self.write_output(&output);
    }

    /// Report issues for multiple files
    pub fn report_all_issues(&self, issues: &[Issue]) {
        let output = match self.format {
            ReportFormat::Text => self.generate_text_report_all(issues),
            ReportFormat::Json => self.generate_json_report(issues),
            ReportFormat::Html => self.generate_html_report_all(issues),
        };

        self.write_output(&output);
    }

    /// Generate a text report for a single file
    fn generate_text_report(&self, file_path: &str, issues: &[Issue]) -> String {
        let mut output = String::new();

        if issues.is_empty() {
            output.push_str(&format!("{}: No issues found\n", file_path));
            return output;
        }

        output.push_str(&format!("{}: {} issues found\n", file_path, issues.len()));

        for issue in issues {
            output.push_str(&format!(
                "  {}:{}:{}: [{}] {} ({})\n",
                issue.file, issue.line, issue.column, issue.severity, issue.message, issue.rule,
            ));
        }

        output
    }

    /// Generate a text report for multiple files
    fn generate_text_report_all(&self, issues: &[Issue]) -> String {
        let mut output = String::new();

        if issues.is_empty() {
            output.push_str("No issues found\n");
            return output;
        }

        // Group issues by file
        let mut files = std::collections::HashMap::new();
        for issue in issues {
            files
                .entry(issue.file.clone())
                .or_insert_with(Vec::new)
                .push(issue);
        }

        output.push_str(&format!(
            "Found {} issues in {} files\n",
            issues.len(),
            files.len()
        ));

        // Sort files by name for consistent output
        let mut files: Vec<_> = files.into_iter().collect();
        files.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (file, file_issues) in files {
            output.push_str(&format!("\n{}: {} issues\n", file, file_issues.len()));

            // Sort issues by line number
            let mut sorted_issues = file_issues;
            sorted_issues.sort_by_key(|i| (i.line, i.column));

            for issue in sorted_issues {
                output.push_str(&format!(
                    "  {}:{}: [{}] {} ({})\n",
                    issue.line, issue.column, issue.severity, issue.message, issue.rule,
                ));
            }
        }

        output
    }

    /// Generate a JSON report
    fn generate_json_report(&self, issues: &[Issue]) -> String {
        serde_json::to_string_pretty(&issues)
            .unwrap_or_else(|_| "Error serializing issues".to_string())
    }

    /// Generate an HTML report for a single file
    fn generate_html_report(&self, _file_path: &str, issues: &[Issue]) -> String {
        self.generate_html_report_all(issues)
    }

    /// Generate an HTML report for multiple files
    fn generate_html_report_all(&self, issues: &[Issue]) -> String {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("    <title>Orbit Analyzer Report</title>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif; line-height: 1.6; color: #333; max-width: 1200px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("        h1, h2, h3 { color: #0066cc; }\n");
        html.push_str("        .summary { background-color: #f5f5f5; border-radius: 5px; padding: 15px; margin-bottom: 20px; }\n");
        html.push_str("        .file { margin-bottom: 30px; border: 1px solid #ddd; border-radius: 5px; padding: 10px; }\n");
        html.push_str("        .file-header { background-color: #f0f0f0; padding: 10px; margin: -10px -10px 10px -10px; border-bottom: 1px solid #ddd; border-radius: 5px 5px 0 0; }\n");
        html.push_str(
            "        .issue { margin-bottom: 10px; padding: 10px; border-radius: 3px; }\n",
        );
        html.push_str(
            "        .error { background-color: #ffeeee; border-left: 5px solid #ff6666; }\n",
        );
        html.push_str(
            "        .warning { background-color: #fff8e6; border-left: 5px solid #ffcc66; }\n",
        );
        html.push_str(
            "        .info { background-color: #e6f5ff; border-left: 5px solid #66b3ff; }\n",
        );
        html.push_str(
            "        .location { font-family: monospace; font-size: 0.9em; color: #666; }\n",
        );
        html.push_str("        .rule { font-size: 0.8em; color: #666; float: right; }\n");
        html.push_str("        .clear { clear: both; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("    <h1>Orbit Analyzer Report</h1>\n");

        // Summary
        let error_count = issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count();
        let warning_count = issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count();
        let info_count = issues
            .iter()
            .filter(|i| i.severity == Severity::Info)
            .count();

        // Group issues by file
        let mut files = std::collections::HashMap::new();
        for issue in issues {
            files
                .entry(issue.file.clone())
                .or_insert_with(Vec::new)
                .push(issue);
        }

        html.push_str("    <div class=\"summary\">\n");
        html.push_str("        <h2>Summary</h2>\n");
        html.push_str(&format!(
            "        <p>Found {} issues in {} files</p>\n",
            issues.len(),
            files.len()
        ));
        html.push_str("        <ul>\n");
        html.push_str(&format!("            <li>Errors: {}</li>\n", error_count));
        html.push_str(&format!(
            "            <li>Warnings: {}</li>\n",
            warning_count
        ));
        html.push_str(&format!(
            "            <li>Information: {}</li>\n",
            info_count
        ));
        html.push_str("        </ul>\n");
        html.push_str("    </div>\n");

        if !issues.is_empty() {
            html.push_str("    <h2>Issues</h2>\n");

            // Sort files by name
            let mut files: Vec<_> = files.into_iter().collect();
            files.sort_by(|(a, _), (b, _)| a.cmp(b));

            for (file, file_issues) in files {
                html.push_str("    <div class=\"file\">\n");
                html.push_str("        <div class=\"file-header\">\n");
                html.push_str(&format!("            <h3>{}</h3>\n", file));
                html.push_str(&format!(
                    "            <p>{} issues</p>\n",
                    file_issues.len()
                ));
                html.push_str("        </div>\n");

                // Sort issues by line number
                let mut sorted_issues = file_issues;
                sorted_issues.sort_by_key(|i| (i.line, i.column));

                for issue in sorted_issues {
                    let severity_class = match issue.severity {
                        Severity::Error => "error",
                        Severity::Warning => "warning",
                        Severity::Info => "info",
                    };

                    html.push_str(&format!(
                        "        <div class=\"issue {}\">\n",
                        severity_class
                    ));
                    html.push_str(&format!(
                        "            <div class=\"rule\">{}</div>\n",
                        issue.rule
                    ));
                    html.push_str(&format!(
                        "            <div class=\"message\">{}</div>\n",
                        issue.message
                    ));
                    html.push_str(&format!(
                        "            <div class=\"location\">Line {}, Column {}</div>\n",
                        issue.line, issue.column
                    ));
                    html.push_str("            <div class=\"clear\"></div>\n");
                    html.push_str("        </div>\n");
                }

                html.push_str("    </div>\n");
            }
        }

        // HTML footer
        html.push_str("</body>\n</html>");

        html
    }

    /// Write output to a file or stdout
    fn write_output(&self, output: &str) {
        match &self.output_path {
            Some(path) => {
                let path = Path::new(path);
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        if let Err(e) = std::fs::create_dir_all(parent) {
                            eprintln!("Error creating directory {}: {}", parent.display(), e);
                            return;
                        }
                    }
                }

                match File::create(path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(output.as_bytes()) {
                            eprintln!("Error writing to {}: {}", path.display(), e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error creating file {}: {}", path.display(), e);
                    }
                }
            }
            None => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                if let Err(e) = handle.write_all(output.as_bytes()) {
                    eprintln!("Error writing to stdout: {}", e);
                }
            }
        }
    }
}
