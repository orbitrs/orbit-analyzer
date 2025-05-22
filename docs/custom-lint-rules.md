# Creating Custom Lint Rules

This document explains how to create custom lint rules for the Orbit Analyzer.

## Overview

The Orbit Analyzer provides a flexible way to create custom lint rules to help validate .orbit files according to your team's coding standards and best practices. This is achieved through the `Rule` trait defined in `src/rules.rs`.

## Rule Trait

To create a custom rule, you need to implement the `Rule` trait, which requires:

```rust
pub trait Rule {
    /// Name of the rule
    fn name(&self) -> &'static str;

    /// Description of the rule
    fn description(&self) -> &'static str;

    /// Check an .orbit file for issues
    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String>;
}
```

## Creating a New Rule

Here's a step-by-step guide to creating a new rule:

1. Create a new struct for your rule:

```rust
pub struct MyCustomRule;
```

2. Implement the `Rule` trait for your struct:

```rust
impl Rule for MyCustomRule {
    fn name(&self) -> &'static str {
        "my-custom-rule"
    }

    fn description(&self) -> &'static str {
        "Description of what the rule checks for and why it's important"
    }

    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();
        
        // Your rule implementation here
        // Check the AST for potential issues
        
        // If you find an issue, add it to the issues vector:
        // issues.push(Issue {
        //    rule_name: self.name().to_string(),
        //    message: "Descriptive message about the issue".to_string(),
        //    file_path: file_path.to_string(),
        //    line: line_number,
        //    column: column_number,
        //    severity: crate::reporter::Severity::Warning,
        // });
        
        Ok(issues)
    }
}
```

3. Register your rule with the analyzer (in `src/linter.rs`):

```rust
let rules: Vec<Box<dyn Rule>> = vec![
    // Existing rules
    Box::new(NonEmptyTemplateRule),
    // Your new rule
    Box::new(MyCustomRule),
];
```

## Example: Script Tag Rule

Here's an example of a rule that checks if a script tag is present:

```rust
pub struct ScriptTagRule;

impl Rule for ScriptTagRule {
    fn name(&self) -> &'static str {
        "script-tag-required"
    }

    fn description(&self) -> &'static str {
        "Components should have a script section"
    }

    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();
        
        if ast.script.is_none() {
            issues.push(Issue {
                rule_name: self.name().to_string(),
                message: "Component is missing a script section".to_string(),
                file_path: file_path.to_string(),
                line: 1, // Default to first line if script section is missing
                column: 1,
                severity: crate::reporter::Severity::Warning,
            });
        }
        
        Ok(issues)
    }
}
```

## Testing Your Rule

To test your rule, you can create a test file in `src/tests/` with appropriate test cases:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_orbit_file;
    
    #[test]
    fn test_my_custom_rule() {
        let content = r#"
        <template>
            <div>Hello, world!</div>
        </template>
        "#;
        
        let ast = parse_orbit_file(content, "test.orbit").unwrap();
        let rule = MyCustomRule;
        let issues = rule.check(&ast, "test.orbit").unwrap();
        
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "my-custom-rule");
    }
}
```

## Guidelines for Good Rules

When creating custom rules, consider the following guidelines:

1. **Clear purpose**: Each rule should have a clear and specific purpose.
2. **Descriptive messages**: Error messages should clearly explain what the issue is and how to fix it.
3. **Performance**: Rules should be efficient, especially when analyzing large files.
4. **False positives**: Minimize false positives to avoid frustrating developers.
5. **Customization**: Consider providing configuration options for your rule if appropriate.

## Built-in Rules

Orbit Analyzer comes with several built-in rules you can use as references:

- `NonEmptyTemplateRule`: Ensures the template section isn't empty
- `StyleTagRule`: Validates the style section's syntax
- `ScriptTagRule`: Ensures proper script section usage
- `ComponentNamingRule`: Enforces component naming conventions
