// Linter for checking .orbit files

use crate::parser;
use crate::reporter::Issue;
use crate::rules::Rule;
use crate::{AnalyzerError, Result};

/// Linter for .orbit files
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    /// Create a new linter with default rules
    pub fn new() -> Self {
        Self {
            rules: vec![
                // Add default rules here
            ],
        }
    }

    /// Add a rule to the linter
    pub fn add_rule<R: Rule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    /// Lint a file and return issues
    pub fn lint(&self, content: &str, file_path: &str) -> Result<Vec<Issue>> {
        let orbit_file = parser::parse_orbit_file(content, file_path)?;

        let mut issues = Vec::new();

        for rule in &self.rules {
            let rule_issues = rule
                .check(&orbit_file, file_path)
                .map_err(|e| AnalyzerError::Rule(e.to_string()))?;

            issues.extend(rule_issues);
        }

        Ok(issues)
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}
