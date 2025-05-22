# Advanced Usage Guide

This document covers advanced usage scenarios for the Orbit Analyzer, including programmatic integration, custom reporters, and fine-grained control over the analysis process.

## Table of Contents

- [Programmatic API](#programmatic-api)
- [Custom Reporters](#custom-reporters)
- [Performance Optimization](#performance-optimization)
- [Handling Large Codebases](#handling-large-codebases)
- [Integrating with Other Tools](#integrating-with-other-tools)

## Programmatic API

The Orbit Analyzer can be used as a library in your Rust projects. This allows for deep integration with your own tools.

### Example: Basic Analysis

```rust
use orbit_analyzer::{Analyzer, Config, ReportFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom configuration
    let config = Config {
        rules: vec!["no-duplicate-ids", "use-state-properly"],
        report_format: ReportFormat::Json,
        min_severity: Severity::Warning,
    };

    // Create an analyzer with the configuration
    let analyzer = Analyzer::new(config);

    // Analyze a file or directory
    let results = analyzer.analyze_path("path/to/component.orbit")?;

    // Process the results
    for issue in results.issues {
        println!("{}: {} at {}:{}", 
            issue.severity,
            issue.message,
            issue.location.file,
            issue.location.line
        );
    }

    Ok(())
}
```

### Example: Custom Rule Integration

```rust
use orbit_analyzer::{Rule, Component, Issue, Severity, Location};

struct MyCustomRule;

impl Rule for MyCustomRule {
    fn name(&self) -> &'static str {
        "my-custom-rule"
    }

    fn description(&self) -> &'static str {
        "Checks for a specific pattern in Orbit components"
    }

    fn check(&self, component: &Component) -> Vec<Issue> {
        // Custom logic to check the component
        // ...

        // Return any issues found
        vec![
            Issue {
                rule: self.name().to_string(),
                message: "Found a problem".to_string(),
                severity: Severity::Error,
                location: Location {
                    file: component.path.clone(),
                    line: 42,
                    column: 10,
                },
            }
        ]
    }
}

// Register your custom rule
let mut analyzer = Analyzer::new(config);
analyzer.add_rule(Box::new(MyCustomRule));
```

## Custom Reporters

The Orbit Analyzer supports custom report formats, which can be useful for integration with specific tools or workflows.

### Creating a Custom Reporter

```rust
use orbit_analyzer::{Reporter, AnalysisResults};

struct MyCustomReporter;

impl Reporter for MyCustomReporter {
    fn report(&self, results: &AnalysisResults) -> String {
        // Generate a custom report format
        let mut report = String::new();
        
        // Add summary information
        report.push_str(&format!("Analyzed {} files\n", results.file_count));
        report.push_str(&format!("Found {} issues\n", results.issues.len()));
        
        // Add detailed issue information
        for issue in &results.issues {
            report.push_str(&format!("{} - {}\n", issue.severity, issue.message));
        }
        
        report
    }
}

// Use the custom reporter
let analyzer = Analyzer::new(config);
let results = analyzer.analyze_path("path/to/components/")?;
let report = MyCustomReporter.report(&results);
println!("{}", report);
```

## Performance Optimization

For large projects with many .orbit files, performance can be a concern. Here are techniques to optimize the analysis process:

### Parallel Analysis

The Orbit Analyzer can analyze multiple files in parallel:

```bash
orlint analyze --parallel path/to/components/
```

Or programmatically:

```rust
let mut config = Config::default();
config.parallel = true;
let analyzer = Analyzer::new(config);
```

### Incremental Analysis

For CI/CD pipelines, you can perform incremental analysis to only check files that have changed:

```bash
orlint analyze --incremental --git-base main path/to/components/
```

This will only analyze files that have changed compared to the specified git branch.

## Handling Large Codebases

For large codebases with hundreds or thousands of .orbit files:

### Filtering

Filter which files to analyze:

```bash
orlint analyze --include "src/components/**/*.orbit" --exclude "src/components/experimental/**"
```

### Staged Analysis

Break down analysis into stages:

```bash
# Stage 1: Quick syntax check
orlint analyze --quick-syntax-only path/to/components/

# Stage 2: Deep analysis on specific components
orlint analyze --deep src/components/critical/
```

## Integrating with Other Tools

### VSCode Extension

The Orbit Analyzer integrates with the VSCode extension for real-time feedback. See the [extension repository](https://github.com/orbitrs/vscode-orbit) for details.

### Pre-commit Hook

Use the Orbit Analyzer as a pre-commit hook:

```bash
#!/bin/sh
# .git/hooks/pre-commit

changed_orbit_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.orbit$')
if [ -n "$changed_orbit_files" ]; then
  orlint analyze $changed_orbit_files
  if [ $? -ne 0 ]; then
    echo "Orbit Analyzer found issues. Commit aborted."
    exit 1
  fi
fi
```

### Monorepo Integration

For monorepos with multiple Orbit-based projects:

```bash
orlint analyze --config-path ./projects/shared/.orlint.toml ./projects/*/src
```

This allows sharing a common configuration across multiple projects.
