// Linter for checking .orbit files

use crate::config::Config;
use crate::parser;
use crate::reporter::Issue;
use crate::rules::Rule;
use crate::{AnalyzerError, Result};
use rayon::prelude::*;
use std::path::Path;

/// Linter for .orbit files
pub struct Linter {
    rules: Vec<Box<dyn Rule + Send + Sync>>,
    config: Config,
}

impl Linter {
    /// Create a new linter with default rules and configuration
    pub fn new() -> Self {
        Self::with_config(Config::default())
    }

    /// Create a new linter with the given configuration
    pub fn with_config(config: Config) -> Self {
        let mut linter = Self {
            rules: Vec::new(),
            config,
        };

        // Add default rules
        linter.add_rule(crate::rules::NonEmptyTemplateRule);
        linter.add_rule(crate::rules::PublicFunctionRule);
        linter.add_rule(crate::rules::ComponentNamingRule::new());
        linter.add_rule(crate::rules::PropTypeRule);
        linter.add_rule(crate::rules::StateVariableRule);

        // Add renderer-specific rules if enabled
        if linter.config.renderer_analysis.enabled {
            linter.add_rule(crate::rules::RendererCompatibilityRule::new(
                linter.config.renderer_analysis.default_renderer.clone(),
            ));
        }

        linter
    }

    /// Add a rule to the linter
    pub fn add_rule<R: Rule + Send + Sync + 'static>(&mut self, rule: R) {
        // Check if the rule is enabled in the configuration
        if self.config.is_rule_enabled(rule.name()) {
            self.rules.push(Box::new(rule));
        }
    }

    /// Lint a file and return issues
    pub fn lint(&self, content: &str, file_path: &str) -> Result<Vec<Issue>> {
        let orbit_file = parser::parse_orbit_file(content, file_path)?;

        let mut issues = Vec::new();

        for rule in &self.rules {
            let rule_issues = rule
                .check(&orbit_file, file_path)
                .map_err(|e| AnalyzerError::Rule(e.to_string()))?;

            // Filter issues by severity
            let filtered_issues = rule_issues
                .into_iter()
                .map(|mut issue| {
                    // Apply custom severity from config if available
                    issue.severity = self.config.get_rule_severity(&issue.rule, issue.severity);
                    issue
                })
                .filter(|issue| issue.severity as u8 >= self.config.reporter.min_severity as u8)
                .collect::<Vec<_>>();

            issues.extend(filtered_issues);
        }

        Ok(issues)
    }

    /// Lint multiple files in parallel
    pub fn lint_files<P: AsRef<Path> + Send + Sync>(&self, file_paths: &[P]) -> Result<Vec<Issue>> {
        if self.config.analyzer.parallel {
            // Parallel linting
            let issues: Result<Vec<Vec<Issue>>> = file_paths
                .par_iter()
                .map(|file_path| {
                    let content = std::fs::read_to_string(file_path)?;
                    self.lint(&content, file_path.as_ref().to_str().unwrap_or("unknown"))
                })
                .collect();

            // Flatten the results
            issues.map(|v| v.into_iter().flatten().collect())
        } else {
            // Sequential linting
            let mut all_issues = Vec::new();
            for file_path in file_paths {
                let content = std::fs::read_to_string(file_path)?;
                let issues =
                    self.lint(&content, file_path.as_ref().to_str().unwrap_or("unknown"))?;
                all_issues.extend(issues);
            }
            Ok(all_issues)
        }
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}
