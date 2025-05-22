# Command Line Interface Guide

This document describes the command-line interface for Orbit Analyzer, including all available commands, options, and usage examples.

## Basic Commands

Orbit Analyzer provides several commands for different use cases:

```bash
# Basic analysis
orlint analyze [options] <path>

# Validate file syntax only
orlint validate [options] <path>

# Generate a configuration file
orlint init [options]

# List available rules
orlint list-rules

# Show version information
orlint --version
```

## Analysis Command

The `analyze` command is the main entry point for analyzing Orbit components:

```bash
orlint analyze [options] <path>
```

### Options

| Option | Description |
| ------ | ----------- |
| `--rules <RULES>` | Comma-separated list of rules to apply |
| `--exclude-rules <RULES>` | Comma-separated list of rules to exclude |
| `--config <FILE>` | Path to configuration file |
| `--format <FORMAT>` | Output format (text, json, html) |
| `--output <FILE>` | Write output to file instead of stdout |
| `--quiet` | Suppress informational output |
| `--verbose` | Show verbose output |
| `--fail-on <LEVEL>` | Exit with non-zero code on: error, warning, info |
| `--include <PATTERN>` | Include only files matching pattern |
| `--exclude <PATTERN>` | Exclude files matching pattern |
| `--parallel` | Run analysis in parallel |
| `--incremental` | Only analyze changed files |
| `--git-base <BRANCH>` | Git branch to compare against for incremental analysis |

### Examples

Basic analysis of a single file:
```bash
orlint analyze src/components/Button.orbit
```

Analyze an entire directory:
```bash
orlint analyze src/components/
```

Use specific rules:
```bash
orlint analyze --rules no-duplicate-ids,use-state-properly src/components/
```

Output in JSON format:
```bash
orlint analyze --format json src/components/ > analysis-report.json
```

Exclude specific files:
```bash
orlint analyze --exclude "**/deprecated/**" src/components/
```

Only analyze files that have changed since the main branch:
```bash
orlint analyze --incremental --git-base main src/components/
```

## Validate Command

The `validate` command performs a syntax check only:

```bash
orlint validate [options] <path>
```

### Options

| Option | Description |
| ------ | ----------- |
| `--format <FORMAT>` | Output format (text, json) |
| `--output <FILE>` | Write output to file instead of stdout |
| `--quiet` | Suppress informational output |

### Examples

Validate a single file:
```bash
orlint validate src/components/Button.orbit
```

Validate all components:
```bash
orlint validate src/components/
```

## Init Command

The `init` command generates a default configuration file:

```bash
orlint init [options]
```

### Options

| Option | Description |
| ------ | ----------- |
| `--path <DIR>` | Directory to create the configuration file in |
| `--force` | Overwrite existing configuration file |

### Examples

Create a configuration file in the current directory:
```bash
orlint init
```

Create a configuration file in a specific directory:
```bash
orlint init --path src/
```

## List-Rules Command

The `list-rules` command shows all available linting rules:

```bash
orlint list-rules [options]
```

### Options

| Option | Description |
| ------ | ----------- |
| `--format <FORMAT>` | Output format (text, json) |
| `--category <CATEGORY>` | Filter rules by category |

### Examples

List all available rules:
```bash
orlint list-rules
```

List rules as JSON:
```bash
orlint list-rules --format json
```

## Environment Variables

Orbit Analyzer respects the following environment variables:

| Variable | Description |
| -------- | ----------- |
| `ORLINT_CONFIG` | Path to configuration file |
| `ORLINT_FORMAT` | Default output format |
| `ORLINT_QUIET` | Suppress informational output if set to 1 |
| `ORLINT_VERBOSE` | Show verbose output if set to 1 |

## Exit Codes

| Code | Description |
| ---- | ----------- |
| `0` | Success - no issues found or all issues below threshold |
| `1` | Issues found at or above the fail-on threshold |
| `2` | Configuration or command-line error |
| `3` | Analysis error (parser error, file system error, etc.) |

## Configuration File

The Orbit Analyzer uses a TOML configuration file. By default, it looks for `.orlint.toml` in the current directory or any parent directory.

Example configuration:

```toml
# Analyzer configuration
[analyzer]
# Rules to check
rules = [
  "no-duplicate-ids",
  "use-state-properly",
  "renderer-compatibility"
]

# Rules to exclude
exclude_rules = []

# Report format (text, json, html)
format = "text"

# Minimum severity to report (error, warning, info)
min_severity = "warning"

# Minimum severity to cause non-zero exit code
fail_on = "error"

# File patterns
[files]
# Include patterns
include = ["**/*.orbit"]

# Exclude patterns
exclude = ["**/deprecated/**", "**/node_modules/**"]

# Performance settings
[performance]
# Enable parallel analysis
parallel = true

# Enable incremental analysis
incremental = false

# Git branch to compare against for incremental analysis
git_base = "main"
```
