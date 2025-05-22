# Orbit Analyzer Documentation

Welcome to the Orbit Analyzer documentation. This guide will help you understand how to use and extend the Orbit Analyzer tool to validate, analyze, and enforce best practices for Orbit UI components.

## Contents

- [Getting Started](#getting-started)
- [Configuration](#configuration)
- [Command Line Interface](cli-usage.md)
- [Advanced Usage](advanced-usage.md)
- [Renderer-Specific Analysis](renderer-specific-analysis.md)
- [Custom Lint Rules](custom-lint-rules.md)
- [CI Integration](ci-integration.md)
- [VSCode Integration](vscode-integration.md)
- [Troubleshooting](troubleshooting.md)
- [Development Guide](../DEVELOPMENT.md)

## Getting Started

### Installation

You can install Orbit Analyzer using cargo:

```bash
cargo install orbit-analyzer
```

Or clone the repository and build from source:

```bash
git clone https://github.com/orbitrs/orbit-analyzer.git
cd orbit-analyzer
cargo build --release
```

### Basic Usage

Once installed, you can run Orbit Analyzer on a single file:

```bash
orbit-analyzer analyze path/to/component.orbit
```

Or analyze an entire directory of .orbit files:

```bash
orbit-analyzer analyze path/to/components/
```

## Configuration

Orbit Analyzer can be configured through a `.orbit-analyzer.toml` file in your project root or by passing command-line arguments.

### Configuration File

Create a `.orbit-analyzer.toml` file in your project root:

```toml
# Analyzer configuration
[analyzer]
# Check for specific linting rules
rules = ["no-duplicate-ids", "use-state-properly", "renderer-compatibility"]

# Report format: "text", "json", "html"
report_format = "text"

# Severity level: "error", "warning", "info"
min_severity = "warning"
```

### Command-Line Options

```bash
orbit-analyzer analyze --help
```

Will show all available command-line options, including:

```
--rules <RULES>              Comma-separated list of rules to check
--report-format <FORMAT>     Output report format (text, json, html)
--min-severity <SEVERITY>    Minimum severity level to report (error, warning, info)
```

## Advanced Usage

See the detailed documentation for advanced usage scenarios:

- [Creating Custom Lint Rules](custom-lint-rules.md)
- [CI Integration](ci-integration.md)
- [Development Guide](../DEVELOPMENT.md)

### Basic Usage

To analyze .orbit files in your project:

```bash
# Check all .orbit files in the src directory
orbit-analyzer --check src/**/*.orbit

# Check a specific file
orbit-analyzer --check src/components/Button.orbit

# Generate a JSON report
orbit-analyzer --check src/**/*.orbit --format json --output report.json
```

### Command Line Options

- `--check <FILES>` - Analyze the specified files
- `--format <FORMAT>` - Output format (text, json)
- `--output <FILE>` - Write output to file instead of stdout
- `--config <FILE>` - Use a custom config file
- `--verbose` - Enable verbose logging
- `--quiet` - Only show errors, no warnings
- `--help` - Display help information

## Configuration

Orbit Analyzer can be configured with a `.orbit-analyzer.toml` file in your project root:

```toml
# .orbit-analyzer.toml

# Specify rules to enable
enabled_rules = [
  "non-empty-template",
  "script-tag-required",
  "style-tag-syntax",
  "component-naming"
]

# Configure specific rules
[rules.component-naming]
pattern = "^[A-Z][a-zA-Z0-9]*$"
```

## Rule Severity Levels

Rules can have different severity levels:

- `error` - The build will fail if this rule is violated
- `warning` - A warning will be displayed, but the build will not fail
- `info` - Informational messages will be displayed

You can configure severity levels in your `.orbit-analyzer.toml`:

```toml
[rule_severity]
"non-empty-template" = "error"
"script-tag-required" = "warning"
"component-naming" = "info"
```

## Common Workflow

A typical workflow for using Orbit Analyzer includes:

1. Install the tool locally
2. Configure the rules for your project
3. Run the analyzer as part of your development process
4. Integrate with your CI/CD pipeline
5. Create custom rules as needed for your specific requirements

For more detailed information, check out the [Custom Lint Rules](custom-lint-rules.md) documentation and [CI Integration](ci-integration.md) guide.
