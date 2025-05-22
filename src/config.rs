// Configuration handler for Orbit Analyzer
// Parses and manages the .orbit-analyzer.toml configuration file

use crate::reporter::Severity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration for the Orbit Analyzer
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    /// Analyzer settings
    #[serde(default)]
    pub analyzer: AnalyzerSettings,

    /// Rules configuration
    #[serde(default)]
    pub rules: RulesConfig,

    /// Reporter configuration
    #[serde(default)]
    pub reporter: ReporterConfig,

    /// Renderer analysis settings
    #[serde(default)]
    pub renderer_analysis: RendererAnalysisConfig,

    /// Rule severity levels (flattened into rules.rule_severity during deserialization)
    #[serde(default, rename = "rule_severity")]
    rule_severity_tmp: HashMap<String, Severity>,
}

/// Analyzer settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyzerSettings {
    /// Whether to perform syntax checking
    #[serde(default = "default_true")]
    pub syntax_check: bool,

    /// Whether to perform semantic analysis
    #[serde(default = "default_true")]
    pub semantic_analysis: bool,

    /// Whether to calculate metrics
    #[serde(default = "default_false")]
    pub metrics_enabled: bool,

    /// List of enabled rules (if empty, all rules are enabled)
    #[serde(default)]
    pub enabled_rules: Vec<String>,

    /// List of disabled rules
    #[serde(default)]
    pub disabled_rules: Vec<String>,

    /// Whether to run analysis in parallel
    #[serde(default = "default_false")]
    pub parallel: bool,

    /// Whether to use incremental analysis
    #[serde(default = "default_false")]
    pub incremental: bool,
}

/// Rules configuration
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RulesConfig {
    /// Component naming rule configuration
    #[serde(default)]
    pub component_naming: ComponentNamingConfig,

    /// Custom severity levels for rules
    #[serde(default)]
    pub rule_severity: HashMap<String, Severity>,
}

/// Component naming rule configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentNamingConfig {
    /// Regex pattern for component names
    #[serde(default = "default_component_pattern")]
    pub pattern: String,
}

/// Reporter configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReporterConfig {
    /// Output format (text, json, html)
    #[serde(default = "default_format")]
    pub format: String,

    /// Output file path (if not specified, output to stdout)
    pub output_path: Option<String>,

    /// Minimum severity level to report
    #[serde(default)]
    pub min_severity: Severity,
}

/// Renderer analysis configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RendererAnalysisConfig {
    /// Whether renderer analysis is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Default renderer to use for analysis
    #[serde(default = "default_renderer")]
    pub default_renderer: String,

    /// Whether to check for renderer metadata in components
    #[serde(default = "default_true")]
    pub check_renderer_metadata: bool,
}

// Default function implementations
fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_component_pattern() -> String {
    "^[A-Z][a-zA-Z0-9]*$".to_string()
}

fn default_format() -> String {
    "text".to_string()
}

fn default_renderer() -> String {
    "auto".to_string()
}

// Default implementation is now derived

impl Default for AnalyzerSettings {
    fn default() -> Self {
        Self {
            syntax_check: true,
            semantic_analysis: true,
            metrics_enabled: false,
            enabled_rules: Vec::new(),
            disabled_rules: Vec::new(),
            parallel: false,
            incremental: false,
        }
    }
}

// Default implementation is now derived

impl Default for ComponentNamingConfig {
    fn default() -> Self {
        Self {
            pattern: default_component_pattern(),
        }
    }
}

impl Default for ReporterConfig {
    fn default() -> Self {
        Self {
            format: default_format(),
            output_path: None,
            min_severity: Severity::Warning,
        }
    }
}

impl Default for RendererAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_renderer: default_renderer(),
            check_renderer_metadata: true,
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;

        // Move rule severities from the temporary field to the rules config
        if !config.rule_severity_tmp.is_empty() {
            config.rules.rule_severity = config.rule_severity_tmp.clone();
            config.rule_severity_tmp.clear();
        }

        Ok(config)
    }

    /// Find and load configuration from the default location
    pub fn find_and_load() -> Result<Self, Box<dyn std::error::Error>> {
        // Search in the current directory and parent directories
        let mut current_dir = std::env::current_dir()?;

        loop {
            let config_path = current_dir.join(".orbit-analyzer.toml");
            if config_path.exists() {
                return Self::from_file(config_path);
            }

            if !current_dir.pop() {
                break;
            }
        }

        // No config found, return default
        Ok(Self::default())
    }

    /// Check if a rule is enabled
    pub fn is_rule_enabled(&self, rule_name: &str) -> bool {
        if !self.analyzer.disabled_rules.is_empty()
            && self
                .analyzer
                .disabled_rules
                .contains(&rule_name.to_string())
        {
            return false;
        }

        if !self.analyzer.enabled_rules.is_empty() {
            return self.analyzer.enabled_rules.contains(&rule_name.to_string());
        }

        true
    }

    /// Get the severity level for a rule
    pub fn get_rule_severity(&self, rule_name: &str, default_severity: Severity) -> Severity {
        self.rules
            .rule_severity
            .get(rule_name)
            .cloned()
            .unwrap_or(default_severity)
    }
}
