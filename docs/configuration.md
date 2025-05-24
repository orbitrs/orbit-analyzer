# Orlint Configuration Guide

Orlint, the Orbit Linter, is a powerful tool for analyzing your Orbit project to identify potential issues, enforce best practices, and optimize performance. It can be configured using an `.orlint.toml` file in your project root. This guide details the available configuration options.

## Configuration File (`.orlint.toml`)

Create a file named `.orlint.toml` in the root directory of your Orbit project. If this file is not present, Orlint will use its default settings.

## Top-Level Configuration

These settings apply globally to Orlint's behavior.

*   `format`: Defines the output format for the analysis report.
    *   Type: String
    *   Default: `"text"`
    *   Options: `"text"`, `"json"`, `"html"`
    *   Example: `format = "json"`

*   `min_severity`: Sets the minimum severity level for issues to be reported. Issues below this level will be ignored.
    *   Type: String
    *   Default: `"warning"`
    *   Options: `"error"`, `"warning"`, `"info"`
    *   Example: `min_severity = "info"`

*   `fail_on`: Sets the minimum severity level that will cause Orlint to exit with a non-zero status code. This is useful for CI/CD pipelines.
    *   Type: String
    *   Default: `"error"`
    *   Options: `"error"`, `"warning"`, `"info"`
    *   Example: `fail_on = "warning"`

## `[analyzer]` Section

This section configures the core analysis engine.

*   `rules`: An array of rule IDs or rule categories to be enabled for the analysis.
    *   Type: Array of Strings
    *   Default: (Orlint's default set of enabled rules)
    *   Example: `rules = ["no-duplicate-ids", "accessibility", "performance-*"]` (Note: `performance-*` would be a glob pattern if supported, otherwise list individual rules)

*   `exclude_rules`: An array of rule IDs or rule categories to be disabled.
    *   Type: Array of Strings
    *   Default: Empty array
    *   Example: `exclude_rules = ["experimental-features", "skia-specific-rule"]`

### `[analyzer.renderers]` Sub-Section

Configure renderer-specific rule sets.

## `[performance]` Section

This section allows you to tune the performance characteristics of Orlint.

| Key             | Type    | Default   | Description                                                                                                                                                              |
|-----------------|---------|-----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `parallel`      | Boolean | `true`    | If `true`, Orlint will attempt to run analysis tasks in parallel, potentially speeding up the process on multi-core systems.                                                |
| `incremental`   | Boolean | `false`   | If `true`, Orlint will only analyze files that have changed since the last analysis or compared to a specific Git branch (see `git_base`). Requires a cache mechanism. |
| `git_base`      | String  | `"main"`  | When `incremental` is `true`, this specifies the Git branch to compare against for determining changed files.                                                            |
| `memory_limit`  | Integer | `0`       | A memory limit in megabytes (MB) for the analyzer process. `0` means no specific limit is enforced by Orlint itself (system limits still apply).                             |

## `[rules.<rule_name>]` Sections

You can provide specific configurations for individual rules by creating a table named `rules` followed by the rule's ID. For example, `[rules.accessibility]` would configure the rule named `accessibility`.

The options available under these sections are specific to each rule. Refer to the documentation for individual rules to see their available configurations.

**Example from `sample-config.toml`:**

```toml
[rules.accessibility]
# Check for missing alt attributes on images
check_alt_attributes = true

# Check for proper ARIA roles
check_aria_roles = true

# Check for color contrast (example: disabled by default)
check_color_contrast = false

[rules.renderer-compatibility]
# List of renderers to check compatibility with
renderers = ["skia", "webgpu"]
```

This configures the `accessibility` rule to check for alt attributes and ARIA roles but not color contrast. It also tells the `renderer-compatibility` rule to check against Skia and WebGPU.

## `[severity]` Section

This section allows you to override the default severity level for specific rules. This is useful if you want to treat certain warnings as errors, or downgrade some errors to warnings for your project.

The keys are rule IDs, and the values are the desired severity levels (`"error"`, `"warning"`, or `"info"`).

**Example from `sample-config.toml`:**

```toml
[severity]
"no-duplicate-ids" = "error"
"accessibility" = "warning"
"experimental-features" = "info"
```

In this example:
- The `no-duplicate-ids` rule will always be treated as an error.
- The `accessibility` rule will be treated as a warning (which might be its default, but this makes it explicit).
- The `experimental-features` rule (if enabled) will be treated as an informational message.

## Default Behavior

If an `.orlint.toml` file is not found in the project root or any parent directory, Orlint will run with its default settings, which typically means:
- A core set of recommended rules are enabled.
- Output is to `stdout` in `text` format.
- Analysis covers all `*.orbit` files not in common exclusion directories like `node_modules`.

It's highly recommended to create an `.orlint.toml` file using `orlint init` and customize it to your project's needs.
