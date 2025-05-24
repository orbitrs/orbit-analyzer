# Orlint Configuration Guide

Orlint, the Orbit Linter, is a powerful tool for analyzing your Orbit project to identify potential issues, enforce best practices, and optimize performance. It can be configured using an `.orlint.toml` file in your project root. This guide details the available configuration options with practical examples and use cases.

## Configuration File (`.orlint.toml`)

Create a file named `.orlint.toml` in the root directory of your Orbit project. If this file is not present, Orlint will use its default settings.

### Quick Start Configuration

For most projects, start with this basic configuration:

```toml
# Basic .orlint.toml for getting started
format = "text"
min_severity = "warning"
fail_on = "error"

[analyzer]
rules = [
    "accessibility",
    "performance",
    "best-practices",
    "security"
]

[performance]
parallel = true
incremental = true
```

## Top-Level Configuration

These settings apply globally to Orlint's behavior.

### `format` - Output Format

Controls how analysis results are presented.

*   **Type**: String  
*   **Default**: `"text"`  
*   **Options**: `"text"`, `"json"`, `"html"`

**Examples:**

```toml
# Human-readable output for development
format = "text"

# Machine-readable output for CI/CD
format = "json"

# Rich visual reports for team reviews
format = "html"
```

**Use Cases:**
- **`text`**: Local development, quick feedback
- **`json`**: CI/CD integration, automated processing
- **`html`**: Code reviews, team reports, documentation

### `min_severity` - Minimum Reporting Level

Sets the minimum severity level for issues to be reported.

*   **Type**: String  
*   **Default**: `"warning"`  
*   **Options**: `"error"`, `"warning"`, `"info"`

**Examples:**

```toml
# Show only critical issues during development
min_severity = "error"

# Show warnings and errors (recommended)
min_severity = "warning"

# Show all issues including informational
min_severity = "info"
```

**Use Cases:**
- **`error`**: Production builds, critical issue focus
- **`warning`**: Regular development, balanced feedback
- **`info`**: Code quality audits, comprehensive analysis

### `fail_on` - Exit Code Control

Determines when Orlint exits with a non-zero status code for CI/CD integration.

*   **Type**: String  
*   **Default**: `"error"`  
*   **Options**: `"error"`, `"warning"`, `"info"`

**Examples:**

```toml
# Fail CI only on errors
fail_on = "error"

# Fail CI on warnings (strict mode)
fail_on = "warning"

# Never fail CI (monitoring mode)
fail_on = "none"  # If supported
```

## `[analyzer]` Section

This section configures the core analysis engine and rule selection.

### `rules` - Rule Selection

Specifies which rule categories or individual rules to enable.

*   **Type**: Array of Strings  
*   **Default**: Orlint's default rule set  

**Rule Categories:**
- `accessibility` - WCAG compliance, screen reader support
- `performance` - Rendering optimization, memory usage
- `security` - XSS prevention, input validation
- `best-practices` - Code organization, naming conventions
- `maintainability` - Code complexity, documentation
- `compatibility` - Cross-platform, renderer compatibility

**Examples:**

```toml
# Enable specific categories
[analyzer]
rules = ["accessibility", "performance", "security"]

# Enable all built-in rules
rules = ["*"]

# Enable specific rules by ID
rules = [
    "no-duplicate-ids",
    "require-alt-text", 
    "optimize-render-loops",
    "validate-props"
]

# Mix categories and specific rules
rules = [
    "accessibility",      # All accessibility rules
    "perf-render-time",   # Specific performance rule
    "sec-input-validation" # Specific security rule
]
```

### `exclude_rules` - Rule Exclusion

Disables specific rules or categories.

*   **Type**: Array of Strings  
*   **Default**: Empty array

**Examples:**

```toml
[analyzer]
rules = ["*"]  # Enable all rules
exclude_rules = [
    "experimental-features",
    "skia-specific-rule",
    "overly-strict-naming"
]

# Exclude entire categories
exclude_rules = ["experimental"]

# Exclude specific rules from a category
rules = ["accessibility"]
exclude_rules = ["a11y-color-contrast"]  # Skip color contrast checks
```

### `renderers` - Renderer-Specific Analysis

Configure analysis for specific rendering backends.

```toml
[analyzer.renderers]
# Analyze compatibility with specific renderers
target_renderers = ["skia", "wgpu"]

# Enable renderer-specific rules
skia_rules = ["skia-font-rendering", "skia-canvas-optimization"]
wgpu_rules = ["wgpu-shader-validation", "wgpu-resource-management"]

# Cross-renderer compatibility
check_compatibility = true
warn_renderer_specific = true
```

## `[files]` Section

Control which files are included in analysis.

```toml
[files]
# Include patterns (glob syntax)
include = [
    "src/**/*.orbit",
    "components/**/*.orbit",
    "pages/**/*.orbit"
]

# Exclude patterns
exclude = [
    "**/node_modules/**",
    "**/dist/**",
    "**/*.test.orbit",
    "**/*.spec.orbit",
    "**/legacy/**"
]

# Custom file extensions
extensions = [".orbit", ".orb"]  # If custom extensions are supported

# Follow symbolic links
follow_symlinks = false

# Maximum file size (in bytes)
max_file_size = 1048576  # 1MB
```

## `[performance]` Section

Configure Orlint's performance characteristics and optimization settings.

### Performance Configuration Options

| Key             | Type    | Default   | Description                                                                                                                                                              |
|-----------------|---------|-----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `parallel`      | Boolean | `true`    | Enable parallel analysis across multiple CPU cores for faster processing                                                                                                |
| `incremental`   | Boolean | `false`   | Only analyze files changed since last analysis or Git branch comparison                                                                                                 |
| `git_base`      | String  | `"main"`  | Git branch to compare against for incremental analysis                                                                                                                  |
| `memory_limit`  | Integer | `0`       | Memory limit in MB (0 = no limit)                                                                                                                                       |
| `cache_size`    | Integer | `1000`    | Maximum number of analysis results to cache                                                                                                                              |
| `timeout`       | Integer | `300`     | Analysis timeout per file in seconds                                                                                                                                    |

### Performance Examples

```toml
# High-performance development setup
[performance]
parallel = true
incremental = true
git_base = "develop"
memory_limit = 2048      # 2GB limit
cache_size = 5000        # Large cache for monorepos

# CI/CD optimized setup
[performance]
parallel = true
incremental = true
git_base = "main"
memory_limit = 1024      # Conservative memory usage
timeout = 120            # Shorter timeout for CI

# Memory-constrained environment
[performance]
parallel = false         # Single-threaded to save memory
incremental = true
memory_limit = 512       # 512MB limit
cache_size = 100         # Small cache
```

## `[rules.<rule_name>]` Sections

Configure individual rules with specific options. Each rule may have different configuration parameters.

### Common Rule Configurations

#### `[rules.accessibility]`

```toml
[rules.accessibility]
# WCAG compliance level
wcag_level = "AA"        # Options: "A", "AA", "AAA"

# Specific checks
check_alt_attributes = true
check_aria_roles = true
check_color_contrast = true
check_keyboard_navigation = true

# Color contrast thresholds
contrast_ratio_normal = 4.5
contrast_ratio_large = 3.0

# Skip certain elements
ignore_decorative_images = true
ignore_hidden_elements = true
```

#### `[rules.performance]`

```toml
[rules.performance]
# Rendering performance thresholds
max_render_time_ms = 16          # 60 FPS target
max_component_size_kb = 100      # Component file size limit
max_nesting_depth = 10           # Template nesting limit

# Memory usage
max_memory_usage_mb = 50         # Per-component memory limit
check_memory_leaks = true

# Bundle optimization
check_unused_imports = true
check_dead_code = true
suggest_lazy_loading = true
```

#### `[rules.security]`

```toml
[rules.security]
# Input validation
check_xss_vulnerabilities = true
check_injection_attacks = true
validate_user_inputs = true

# Content Security Policy
enforce_csp = true
allow_inline_styles = false
allow_eval = false

# Data handling
check_sensitive_data = true
enforce_encryption = true
```

#### `[rules.best-practices]`

```toml
[rules.best-practices]
# Naming conventions
enforce_pascal_case_components = true
enforce_camel_case_props = true
enforce_kebab_case_files = false

# Code organization
max_component_lines = 300
max_function_lines = 50
require_prop_types = true
require_default_props = true

# Documentation
require_component_docs = true
require_prop_docs = true
suggest_examples = true
```

#### `[rules.maintainability]`

```toml
[rules.maintainability]
# Complexity limits
max_cyclomatic_complexity = 10
max_cognitive_complexity = 15
max_function_parameters = 5

# Code quality
disallow_duplicate_code = true
enforce_consistent_style = true
require_error_handling = true

# Dependencies
check_circular_dependencies = true
limit_dependency_depth = 5
```

#### `[rules.compatibility]`

```toml
[rules.compatibility]
# Renderer compatibility
check_skia_compatibility = true
check_wgpu_compatibility = true
warn_renderer_specific_features = true

# Platform compatibility
target_platforms = ["web", "desktop", "mobile"]
check_platform_apis = true

# Browser compatibility (for web targets)
min_browser_versions = { chrome = "90", firefox = "85", safari = "14" }
```

## `[severity]` Section

Override default severity levels for specific rules to match your project's quality standards.

```toml
[severity]
# Critical issues that should fail builds
"no-duplicate-ids" = "error"
"security-xss-vulnerable" = "error"
"accessibility-missing-alt" = "error"

# Important but not build-breaking
"performance-slow-render" = "warning"
"best-practices-naming" = "warning"

# Informational for code quality
"maintainability-complexity" = "info"
"compatibility-renderer-specific" = "info"

# Category-wide severity overrides
"experimental-*" = "info"           # All experimental rules as info
"performance-*" = "warning"         # All performance rules as warnings
```

## Environment-Specific Configuration

### Development Configuration

```toml
# .orlint.dev.toml
format = "text"
min_severity = "warning"
fail_on = "error"

[performance]
parallel = true
incremental = true

[analyzer]
rules = ["accessibility", "security", "best-practices"]
exclude_rules = ["experimental", "strict-performance"]

[rules.performance]
max_render_time_ms = 32    # More lenient for development
```

### Production Configuration

```toml
# .orlint.prod.toml
format = "json"
min_severity = "info"
fail_on = "warning"

[performance]
parallel = true
incremental = false        # Full analysis for production

[analyzer]
rules = ["*"]              # All rules enabled
exclude_rules = []         # No exclusions

[rules.performance]
max_render_time_ms = 16    # Strict performance requirements
```

### CI/CD Configuration

```toml
# .orlint.ci.toml
format = "json"
min_severity = "warning"
fail_on = "error"

[performance]
parallel = true
incremental = true
git_base = "main"
memory_limit = 2048
timeout = 300

[analyzer]
rules = ["security", "accessibility", "performance"]

# Strict rules for CI
[rules.security]
check_xss_vulnerabilities = true
check_injection_attacks = true

[rules.accessibility]
wcag_level = "AA"
check_color_contrast = true
```

## Configuration Patterns

### Monorepo Configuration

```toml
# Root .orlint.toml for workspace
[workspace]
members = ["packages/*", "apps/*"]
shared_rules = "shared/orlint-rules.toml"

# Package-specific overrides
[packages.ui]
rules = ["accessibility", "performance", "maintainability"]

[packages.core]
rules = ["security", "performance", "best-practices"]

[apps.web]
rules = ["*"]
exclude_rules = ["desktop-specific"]
```

### Team Configuration

```toml
# Balanced configuration for team development
format = "text"
min_severity = "warning"
fail_on = "error"

[analyzer]
rules = [
    "accessibility",
    "security", 
    "best-practices",
    "performance"
]

# Reasonable thresholds
[rules.performance]
max_render_time_ms = 20
max_component_size_kb = 150

[rules.accessibility]
wcag_level = "AA"
check_color_contrast = false  # Can be challenging during development

[rules.best-practices]
max_component_lines = 400      # Flexible for complex components
```

### Open Source Project Configuration

```toml
# Strict configuration for open source quality
format = "text"
min_severity = "info"
fail_on = "warning"

[analyzer]
rules = ["*"]
exclude_rules = ["experimental"]

# High quality standards
[rules.accessibility]
wcag_level = "AAA"
check_color_contrast = true

[rules.maintainability]
require_component_docs = true
require_prop_docs = true
max_cyclomatic_complexity = 8

[rules.performance]
max_render_time_ms = 12    # High performance standard
```

## Default Behavior

If an `.orlint.toml` file is not found in the project root or any parent directory, Orlint will run with its default settings:

- **Rules**: Core set of recommended rules (accessibility, security, best-practices)
- **Format**: Text output to stdout
- **Severity**: Minimum reporting level of "warning"
- **Files**: All `*.orbit` files excluding common directories (node_modules, dist, target)
- **Performance**: Parallel processing enabled, no incremental analysis

### Creating Default Configuration

Generate a default configuration file:

```bash
# Create basic .orlint.toml with recommended settings
orlint init

# Create configuration with all available options
orlint init --full

# Create configuration for specific use case
orlint init --template=strict
orlint init --template=development
orlint init --template=ci
```

## Configuration Validation

Orlint validates configuration files and provides helpful error messages:

### Common Validation Errors

```bash
❌ Error: Invalid severity level 'critical' in .orlint.toml:15
   Valid options: error, warning, info

❌ Error: Unknown rule 'invalid-rule-name' in rules array
   Run 'orlint list-rules' to see available rules

❌ Error: Invalid TOML syntax in .orlint.toml:23
   Missing closing quote on line 23

⚠️  Warning: Rule 'experimental-rule' is deprecated
   Consider using 'new-rule' instead
```

### Configuration Validation Commands

```bash
# Validate configuration file
orlint config validate

# Check configuration with verbose output
orlint config validate --verbose

# Validate specific configuration file
orlint config validate --config=.orlint.prod.toml

# Dry run with configuration
orlint analyze --dry-run --config=.orlint.toml
```

## Best Practices

### 1. Start Simple

Begin with a minimal configuration and gradually add rules:

```toml
# Week 1: Basic safety
[analyzer]
rules = ["security", "accessibility"]

# Week 2: Add performance
rules = ["security", "accessibility", "performance"]

# Week 3: Add code quality
rules = ["security", "accessibility", "performance", "best-practices"]
```

### 2. Use Environment-Specific Configs

```bash
# Development
orlint analyze --config=.orlint.dev.toml

# CI/CD
orlint analyze --config=.orlint.ci.toml

# Production audit
orlint analyze --config=.orlint.strict.toml
```

### 3. Document Your Configuration

```toml
# Project-specific Orlint configuration
# Maintained by: Frontend Team
# Last updated: 2025-05-23
# 
# This configuration enforces:
# - WCAG AA accessibility compliance
# - 60 FPS rendering performance
# - Security best practices
# - Team coding standards

format = "text"
min_severity = "warning"
fail_on = "error"

[analyzer]
rules = ["accessibility", "performance", "security", "best-practices"]

# Performance: Target 60 FPS
[rules.performance]
max_render_time_ms = 16
```

### 4. Regular Configuration Maintenance

```bash
# Monthly configuration review
orlint config check-updates

# Upgrade configuration format
orlint config migrate --from-version=1.0 --to-version=1.1

# Audit rule effectiveness
orlint analyze --report=rule-usage
```

## Troubleshooting

### Configuration Not Loading

```bash
# Check configuration discovery
orlint config show

# Verify configuration file location
orlint config path

# Debug configuration loading
ORLINT_DEBUG=config orlint analyze
```

### Performance Issues

```toml
# For large codebases
[performance]
parallel = true
memory_limit = 4096
cache_size = 10000

# For resource-constrained environments
[performance]
parallel = false
memory_limit = 512
incremental = true
```

### Rule Conflicts

```toml
# When rules conflict, use severity to prioritize
[severity]
"strict-naming" = "info"      # Downgrade conflicting rule
"team-naming" = "warning"     # Keep team standard
```

It's highly recommended to create an `.orlint.toml` file using `orlint init` and customize it to your project's needs. This ensures consistent code quality and helps catch issues early in the development process.
