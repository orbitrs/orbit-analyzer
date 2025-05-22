// New rule implementation for the orbit-analyzer
// These rules enhance the analyzer's capabilities for static code analysis

use crate::reporter::{Issue, Severity};
use crate::rules::Rule;
use orbitrs::parser::OrbitAst;

/// Rule for checking component naming conventions
pub struct ComponentNamingRule {
    pattern: regex::Regex,
}

impl ComponentNamingRule {
    pub fn new() -> Self {
        Self {
            // Default pattern: PascalCase (starts with uppercase, contains only alphanumeric)
            pattern: regex::Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap(),
        }
    }

    pub fn with_pattern(pattern: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: regex::Regex::new(pattern)?,
        })
    }
}

impl Rule for ComponentNamingRule {
    fn name(&self) -> &'static str {
        "component-naming"
    }

    fn description(&self) -> &'static str {
        "Component names should follow naming conventions (default: PascalCase)"
    }

    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();

        // Component name is stored as a String, not an Option<String>
        // Only check if not empty
        if !ast.script.component_name.is_empty() {
            if !self.pattern.is_match(&ast.script.component_name) {
                issues.push(Issue {
                    rule: self.name().to_string(),
                    message: format!(
                        "Component name '{}' does not follow naming convention",
                        ast.script.component_name
                    ),
                    file: file_path.to_string(),
                    line: 1,   // Default line number since field doesn't exist in ScriptNode
                    column: 1, // Default column number since field doesn't exist in ScriptNode
                    severity: Severity::Warning,
                });
            }
        }

        Ok(issues)
    }
}

/// Rule for checking if all properties have type annotations
pub struct PropTypeRule;

impl Rule for PropTypeRule {
    fn name(&self) -> &'static str {
        "prop-type-required"
    }

    fn description(&self) -> &'static str {
        "All component properties should have type annotations"
    }

    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();

        for prop in &ast.script.props {
            if prop.ty.is_empty() {
                issues.push(Issue {
                    rule: self.name().to_string(),
                    message: format!("Property '{}' is missing a type annotation", prop.name),
                    file: file_path.to_string(),
                    line: 1,   // Default line
                    column: 1, // Default column
                    severity: Severity::Error,
                });
            }
        }

        Ok(issues)
    }
}

/// Rule for checking renderer compatibility
pub struct RendererCompatibilityRule {
    #[allow(dead_code)]
    renderer: String,
}

impl RendererCompatibilityRule {
    pub fn new(renderer: String) -> Self {
        Self { renderer }
    }
}

impl Rule for RendererCompatibilityRule {
    fn name(&self) -> &'static str {
        "renderer-compatibility"
    }

    fn description(&self) -> &'static str {
        "Check component compatibility with specific renderers"
    }

    fn check(&self, _ast: &OrbitAst, _file_path: &str) -> Result<Vec<Issue>, String> {
        // Simplified implementation that doesn't depend on the specific structure
        // of TemplateNode which appears to have changed
        // We'll implement a more basic check in the future

        let issues = Vec::new();

        // Return empty issues list for now
        Ok(issues)
    }
}

/// Rule for checking state variable usage
pub struct StateVariableRule;

impl Rule for StateVariableRule {
    fn name(&self) -> &'static str {
        "state-variable-usage"
    }

    fn description(&self) -> &'static str {
        "Check for proper state variable usage patterns"
    }

    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();

        // Check if state variables are properly defined
        for state_var in &ast.script.state {
            // Check if state variable has a type
            if state_var.ty.is_empty() {
                issues.push(Issue {
                    rule: self.name().to_string(),
                    message: format!(
                        "State variable '{}' is missing type annotation",
                        state_var.name
                    ),
                    file: file_path.to_string(),
                    line: 1,   // Default line
                    column: 1, // Default column
                    severity: Severity::Warning,
                });
            }

            // Check if state variable has an initial value
            // The field is named 'initial' and it's an Option<String>
            if state_var.initial.is_none() {
                issues.push(Issue {
                    rule: self.name().to_string(),
                    message: format!(
                        "State variable '{}' is missing initial value",
                        state_var.name
                    ),
                    file: file_path.to_string(),
                    line: 1,   // Default line
                    column: 1, // Default column
                    severity: Severity::Warning,
                });
            }
        }

        Ok(issues)
    }
}
