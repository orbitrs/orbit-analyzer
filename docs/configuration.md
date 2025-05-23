# Orlint Configuration (`.orlint.toml`)

Orlint, the Orbit Analyzer, is configured using a TOML file named `.orlint.toml` located in your project's root directory. This file allows you to customize various aspects of the linting process, including enabled rules, file inclusion/exclusion, output formatting, and performance settings.

You can generate a sample configuration file by running `orlint init`.

## Top-Level Structure

The `.orlint.toml` file is organized into several sections (tables in TOML):

- `[analyzer]`: Core settings for the analysis process.
- `[analyzer.renderers]`: Configuration for renderer-specific rules.
- `[files]`: Patterns for including and excluding files from analysis.
- `[performance]`: Settings related to the performance of the analyzer.
- `[rules.<rule_name>]`: Specific configuration for individual rules.
- `[severity]`: Overrides for the default severity of specific rules.

## `[analyzer]` Section

This section controls the main behavior of Orlint.

| Key             | Type          | Default   | Description                                                                                                                               |
|-----------------|---------------|-----------|-------------------------------------------------------------------------------------------------------------------------------------------|
| `rules`         | Array of Strings | `[]`      | A list of rule IDs to be enabled for analysis. Example: `["no-duplicate-ids", "accessibility"]`.                                       |
| `exclude_rules` | Array of Strings | `[]`      | A list of rule IDs to be disabled, even if they are part of a group or enabled by default. Example: `["experimental-features"]`.         |
| `format`        | String        | `"text"`  | The default output format for reports. Supported values: `"text"`, `"json"`, `"html"`. This can be overridden by the `--format` CLI option. |
| `min_severity`  | String        | `"warning"` | The minimum severity level for issues to be reported. Supported values: `"error"`, `"warning"`, `"info"`.                             |
| `fail_on`       | String        | `"error"`   | The minimum severity level that will cause Orlint to exit with a non-zero status code. Supported values: `"error"`, `"warning"`, `"info"`. |

### `[analyzer.renderers]` Sub-Section

This sub-section within `[analyzer]` configures renderer-specific linting.

| Key              | Type    | Default | Description                                                                    |
|------------------|---------|---------|--------------------------------------------------------------------------------|
| `skia`           | Boolean | `false` | If `true`, enables rules specific to the Skia rendering pipeline.                |
| `webgpu`         | Boolean | `false` | If `true`, enables rules specific to the WebGPU rendering pipeline.              |
| `cross_renderer` | Boolean | `true`  | If `true`, enables rules that check for compatibility across multiple renderers. |

## `[files]` Section

This section defines which files Orlint should analyze.

| Key       | Type          | Default                                  | Description                                                                                                                                  |
|-----------|---------------|------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| `include` | Array of Strings | `["**/*.orbit"]`                       | An array of glob patterns specifying which files to include in the analysis.                                                                 |
| `exclude` | Array of Strings | `["**/node_modules/**", "**/dist/**"]` | An array of glob patterns specifying which files or directories to exclude from analysis. Exclusions typically cover third-party code or build outputs. |

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
