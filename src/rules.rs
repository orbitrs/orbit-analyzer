// Rules for checking .orbit files

use crate::reporter::Issue;
use orbit::parser::OrbitFile;

/// Trait for lint rules
pub trait Rule {
    /// Name of the rule
    fn name(&self) -> &'static str;
    
    /// Description of the rule
    fn description(&self) -> &'static str;
    
    /// Check an .orbit file for issues
    fn check(&self, file: &OrbitFile, file_path: &str) -> Result<Vec<Issue>, String>;
}

/// Rule for checking if template is empty
pub struct NonEmptyTemplateRule;

impl Rule for NonEmptyTemplateRule {
    fn name(&self) -> &'static str {
        "non-empty-template"
    }
    
    fn description(&self) -> &'static str {
        "Template section should not be empty"
    }
    
    fn check(&self, file: &OrbitFile, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();
        
        if file.template.is_empty() {
            issues.push(Issue {
                rule: self.name().to_string(),
                message: "Template section is empty".to_string(),
                file: file_path.to_string(),
                line: 1, // Placeholder
                column: 1, // Placeholder
                severity: crate::reporter::Severity::Warning,
            });
        }
        
        Ok(issues)
    }
}

/// Rule for checking if script contains public functions
pub struct PublicFunctionRule;

impl Rule for PublicFunctionRule {
    fn name(&self) -> &'static str {
        "public-function"
    }
    
    fn description(&self) -> &'static str {
        "Component should have at least one public function"
    }
    
    fn check(&self, file: &OrbitFile, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();
        
        // This is a placeholder implementation
        // In a real implementation, we would parse the Rust code and check for public functions
        
        Ok(issues)
    }
}
