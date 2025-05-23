# Creating Custom Lint Rules for Orlint

This document explains how to create custom lint rules for Orlint, the Orbit Analyzer. Custom rules allow you to enforce project-specific coding standards, identify potential bugs, or encourage best practices within your `.orbit` files.

## Overview

Orlint's custom rule system is built around the `Rule` trait, typically found in `orlint/src/rules.rs` (or a similar path within the Orlint codebase). By implementing this trait, you can define the logic for identifying specific patterns or issues in the Abstract Syntax Tree (AST) of `.orbit` files.

## The `Rule` Trait

A custom rule is a Rust struct that implements the `Rule` trait. The trait generally requires the following methods:

```rust
// Note: The exact path and definition might vary slightly.
// Refer to the actual `rules.rs` in the Orlint source for the precise trait.
use orlint::core::{OrbitAst, Issue, Severity}; // Hypothetical paths

pub trait Rule {
    /// A unique identifier for the rule.
    /// Used in configuration files and error messages.
    /// Conventionally, use kebab-case (e.g., "my-custom-rule").
    fn name(&self) -> &'static str;

    /// A brief description of what the rule checks for and why it's important.
    /// This is often used for documentation generation (e.g., `orlint list-rules`).
    fn description(&self) -> &'static str;

    /// The core logic of the rule.
    /// This method receives the AST of an `.orbit` file and its path.
    /// It should return a `Result` containing a vector of `Issue` structs if problems are found,
    /// or an error string if the rule itself encounters an issue during processing.
    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String>;

    // Optional: Some rule systems might include a method to specify default severity
    // fn default_severity(&self) -> Severity {
    //     Severity::Warning
    // }

    // Optional: Some rule systems might allow rules to be configurable
    // fn configure(&mut self, config: &RuleConfig) -> Result<(), String> {
    //     // Process configuration for this rule
    //     Ok(())
    // }
}
```

The `Issue` struct typically contains:
- `rule_name`: The name of the rule that generated the issue.
- `message`: A descriptive message explaining the issue.
- `file_path`: The path to the file where the issue was found.
- `line`: The line number of the issue.
- `column`: The column number of the issue.
- `severity`: The severity of the issue (e.g., `Error`, `Warning`, `Info`).
- `suggestion`: (Optional) A suggested fix or code snippet.

## Steps to Create a New Rule

1.  **Define Your Rule Struct:**
    Create a new Rust struct for your rule. It can be a simple unit struct if it doesn't require internal state, or it can have fields if it needs to be configured.

    ```rust
    pub struct EnforceHeaderCommentRule;
    ```

2.  **Implement the `Rule` Trait:**
    Implement the `Rule` trait for your struct, providing the `name`, `description`, and the core `check` logic.

    ```rust
    // Assuming hypothetical paths for Issue, Severity, OrbitAst
    // use orlint::core::{OrbitAst, Issue, Severity, AstNode};

    impl Rule for EnforceHeaderCommentRule {
        fn name(&self) -> &'static str {
            "enforce-header-comment"
        }

        fn description(&self) -> &'static str {
            "Ensures that every .orbit file starts with a specific header comment."
        }

        fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String> {
            let mut issues = Vec::new();
            let expected_comment_prefix = "// OrbitFile v"; // Example prefix

            // This is a simplified check. A real AST would provide access to comments.
            // We'll assume `ast.raw_content()` gives us the file content for this example.
            // In a real scenario, you'd inspect the AST's representation of comments.
            if let Some(raw_content) = ast.raw_content() { // Assuming raw_content() method exists
                if !raw_content.trim_start().starts_with(expected_comment_prefix) {
                    issues.push(Issue {
                        rule_name: self.name().to_string(),
                        message: format!(
                            "File must start with a comment like '{}...'",
                            expected_comment_prefix
                        ),
                        file_path: file_path.to_string(),
                        line: 1, // Issue is at the beginning of the file
                        column: 1,
                        severity: Severity::Warning, // Or Severity::Error
                        suggestion: Some(format!("Add a comment like '{}1.0' at the top.", expected_comment_prefix)),
                    });
                }
            } else {
                // If raw content isn't available, or AST doesn't support this check easily,
                // this rule might not be implementable as shown.
                // This highlights the dependency on AST capabilities.
            }
            Ok(issues)
        }
    }
    ```

3.  **Register Your Rule:**
    Your new rule needs to be registered with Orlint's rule runner, usually in a central place like `orlint/src/linter.rs` or `orlint/src/rules/mod.rs`. This typically involves adding an instance of your rule to a list of active rules.

    ```rust
    // In orlint/src/linter.rs or similar
    pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
        vec![
            // ... other existing rules ...
            Box::new(crate::rules::style::StyleTagRule),
            Box::new(crate::rules::naming::ComponentNamingRule),
            // Add your new rule
            Box::new(crate::rules::custom::EnforceHeaderCommentRule), // Assuming it's in `rules/custom.rs`
        ]
    }
    ```
    *(The exact registration mechanism might differ. Consult the Orlint codebase.)*

## Example: NoInlineStylesRule

Let's create a rule that discourages or disallows the use of inline `style` attributes on template elements.

```rust
// in orlint/src/rules/style.rs or a new file like orlint/src/rules/best_practices.rs

// Assuming hypothetical paths for Issue, Severity, OrbitAst, AstNode, ElementNode
// use orlint::core::{OrbitAst, Issue, Severity, AstNode, ElementNode, Attribute};

pub struct NoInlineStylesRule;

impl Rule for NoInlineStylesRule {
    fn name(&self) -> &'static str {
        "no-inline-styles"
    }

    fn description(&self) -> &'static str {
        "Disallows the use of inline 'style' attributes on HTML elements in templates."
    }

    fn check(&self, ast: &OrbitAst, file_path: &str) -> Result<Vec<Issue>, String> {
        let mut issues = Vec::new();

        // Hypothetical AST traversal
        // You would need a way to iterate over all elements in the template AST
        if let Some(template_ast) = &ast.template { // Assuming `ast.template` gives access to template nodes
            for node in template_ast.walk_elements() { // Assuming a `walk_elements()` method
                if let Some(element_node) = node.as_element() { // Assuming `as_element()`
                    for attr in element_node.attributes() { // Assuming `attributes()`
                        if attr.name.to_lowercase() == "style" {
                            issues.push(Issue {
                                rule_name: self.name().to_string(),
                                message: "Avoid using inline 'style' attributes. Prefer using scoped CSS or external stylesheets.".to_string(),
                                file_path: file_path.to_string(),
                                line: attr.span.start_line, // Assuming attributes have span info
                                column: attr.span.start_column,
                                severity: Severity::Warning,
                                suggestion: Some("Move styles to the <style> block or a dedicated CSS file.".to_string()),
                            });
                        }
                    }
                }
            }
        }
        Ok(issues)
    }
}
```
**Note:** The AST traversal (`walk_elements`, `as_element`, `attributes`, `attr.span`) is highly dependent on the actual structure and API provided by Orlint's `OrbitAst`. The example above is illustrative. You'll need to adapt it to the real AST.

## Testing Your Custom Rule

Thoroughly testing your custom rules is crucial. Orlint likely has a testing framework or conventions in `orlint/tests/` or `orlint/src/rules/tests/`.

**General Approach to Testing:**

1.  **Create Test Files:** Prepare `.orbit` files with code that should trigger your rule and code that should not.
2.  **Write Test Cases:** Use Rust's testing framework (`#[test]`).
    *   Parse the test `.orbit` files into an `OrbitAst`.
    *   Instantiate your rule.
    *   Call the `check` method.
    *   Assert that the correct number of issues (or no issues) are reported.
    *   Verify the details of reported issues (message, line, column, severity).

**Example Test Case (Conceptual):**

```rust
// In orlint/tests/rules/test_no_inline_styles.rs or similar

#[cfg(test)]
mod tests {
    use orlint::core::{OrbitAst, Issue, Severity}; // Hypothetical
    use orlint::parser::parse_orbit_file; // Hypothetical
    use crate::rules::style::NoInlineStylesRule; // Path to your rule
    use orlint::Rule; // The trait

    fn run_rule_on_content(content: &str) -> Vec<Issue> {
        let ast = parse_orbit_file(content, "test.orbit").expect("Failed to parse test content");
        let rule = NoInlineStylesRule;
        rule.check(&ast, "test.orbit").expect("Rule check failed")
    }

    #[test]
    fn test_no_inline_styles_positive() {
        let content = r#"
            <template>
                <div style="color: red;">Hello</div>
            </template>
            <script></script>
            <style></style>
        "#;
        let issues = run_rule_on_content(content);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "no-inline-styles");
        assert_eq!(issues[0].line, 3); // Adjust line/column based on your parser
        assert_eq!(issues[0].severity, Severity::Warning);
    }

    #[test]
    fn test_no_inline_styles_negative() {
        let content = r#"
            <template>
                <div class="my-class">Hello</div>
            </template>
            <script></script>
            <style>.my-class { color: red; }</style>
        "#;
        let issues = run_rule_on_content(content);
        assert!(issues.is_empty(), "Should find no issues for valid code");
    }
}
```

## Best Practices for Writing Rules

1.  **Clear Purpose & Specificity:**
    *   Each rule should have a single, well-defined responsibility. Avoid creating "catch-all" rules.
    *   **Good:** `disallow-specific-prop`, `require-alt-text`.
    *   **Less Good:** `check-all-accessibility-issues`.

2.  **Descriptive Names and Messages:**
    *   **Name:** Use a clear, kebab-case name (e.g., `require-button-type`).
    *   **Messages:** Error/warning messages should be:
        *   **Clear:** Explain *what* is wrong.
        *   **Concise:** Get to the point.
        *   **Actionable:** Explain *why* it's wrong and *how to fix it*. Provide suggestions if possible.

3.  **Performance Considerations:**
    *   The `check` method will be called for many files, and potentially many times within each file if traversing nodes.
    *   Avoid unnecessary computations or allocations within loops.
    *   Profile your rules if you suspect performance issues.

4.  **Minimize False Positives:**
    *   A rule that frequently flags correct code is frustrating and will likely be disabled. Test with a wide variety of valid and invalid code samples.
    *   Consider edge cases carefully.

5.  **AST-Driven Logic:**
    *   Rely on the structure and information provided by the AST rather than string matching or regex on raw file content whenever possible. The AST provides a more reliable and semantic understanding of the code.
    *   Understand the capabilities of your `OrbitAst`. What information does it provide about nodes, attributes, comments, script content, style content, etc.?

6.  **Configurability (Optional but often useful):**
    *   If a rule might have variations or thresholds (e.g., maximum line length, allowed/disallowed attributes), consider making it configurable through the `.orlint.toml` file. This usually involves adding fields to your rule struct and a way to deserialize configuration into it.
    *   Example: A `max-nesting-depth` rule might take an integer `depth` from the config.

7.  **Appropriate Severity:**
    *   Use `Severity::Error` for things that are definitely wrong or will cause problems.
    *   Use `Severity::Warning` for potential issues, style violations, or best practice deviations.
    *   Use `Severity::Info` for informational messages if needed (less common for lint rules).
    *   Allow users to override severities in their `.orlint.toml`.

8.  **Provide Suggestions:**
    *   If an automated or semi-automated fix is possible, include it in the `Issue::suggestion` field. This greatly improves the developer experience.

9.  **Documentation:**
    *   Document your rule's purpose, why it's important, and how to fix violations. If it's configurable, document the options. This information can be part of the `description` or in separate markdown files.

10. **Idempotency:**
    *   Running the rule multiple times on the same unchanged code should produce the same result.

## Orlint's AST (`OrbitAst`)

To write effective rules, you need a good understanding of the `OrbitAst` structure provided by Orlint's parser. This AST is your primary interface to the code. Key things to look for in the AST definition:

*   How are template elements, attributes, and text nodes represented?
*   How can you access the content of `<script>` and `<style>` tags?
*   Is there information about source spans (line and column numbers) for all relevant nodes and attributes? This is crucial for reporting accurate issue locations.
*   Are comments preserved and accessible?
*   How does the AST represent Orbit-specific syntax (e.g., directives, bindings)?

Consult the Orlint source code, particularly around the parser and AST definitions, to understand how to navigate and query it effectively.

## Built-in Rules as Reference

Examine Orlint's built-in rules. They serve as excellent examples of how to interact with the AST and structure your rule logic. The existing `ScriptTagRule`, `NonEmptyTemplateRule`, etc., mentioned in the original document are good starting points.

By following these guidelines and understanding the Orlint framework, you can create powerful custom lint rules that significantly improve code quality and consistency in your Orbit projects.
