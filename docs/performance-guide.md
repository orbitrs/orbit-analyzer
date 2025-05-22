# Performance Optimization Guide

This document provides strategies for optimizing the performance of Orbit Analyzer when working with large codebases or resource-constrained environments.

## Table of Contents

- [Understanding Performance Factors](#understanding-performance-factors)
- [Parallel Analysis](#parallel-analysis)
- [Incremental Analysis](#incremental-analysis)
- [Selective Analysis](#selective-analysis)
- [Memory Optimization](#memory-optimization)
- [CI/CD Optimization](#cicd-optimization)
- [Benchmarking](#benchmarking)

## Understanding Performance Factors

Orbit Analyzer's performance is affected by several factors:

1. **Number of files** - The total number of .orbit files being analyzed
2. **File complexity** - More complex components take longer to analyze
3. **Rule complexity** - Some rules are more CPU-intensive than others
4. **System resources** - Available CPU cores, memory, and I/O speed
5. **Configuration** - Parallel execution, incremental analysis settings

## Parallel Analysis

Enabling parallel analysis can significantly improve performance on multi-core systems:

### Configuration

In your `.orlint.toml`:

```toml
[performance]
parallel = true
```

Or via command line:

```bash
orlint analyze --parallel path/to/components/
```

### Controlling Concurrency

You can control the level of concurrency:

```toml
[performance]
parallel = true
max_concurrency = 4  # Limit to 4 parallel processes
```

Or via environment variable:

```bash
ORBIT_ANALYZER_MAX_CONCURRENCY=4 orlint analyze --parallel path/to/components/
```

### When to Avoid Parallel Analysis

- On memory-constrained systems
- When running alongside other CPU-intensive tasks
- When analyzing only a few files

## Incremental Analysis

Incremental analysis only analyzes files that have changed since a baseline:

### Configuration

In your `.orlint.toml`:

```toml
[performance]
incremental = true
git_base = "main"  # Compare against main branch
```

Or via command line:

```bash
orlint analyze --incremental --git-base main path/to/components/
```

### Cache Configuration

Control where the incremental cache is stored:

```toml
[performance]
incremental = true
cache_dir = ".orbit-cache"
```

### Manual Baseline

You can also specify a manual baseline instead of using git:

```bash
orlint analyze --incremental --baseline previous-analysis.json path/to/components/
```

## Selective Analysis

Analyze only the files you need:

### File Filtering

In your `.orlint.toml`:

```toml
[files]
include = ["src/components/critical/**/*.orbit"]
exclude = ["**/test/**", "**/deprecated/**"]
```

Or via command line:

```bash
orlint analyze --include "src/components/critical/**/*.orbit" --exclude "**/test/**" .
```

### Rule Selection

Only run the rules you need:

```toml
[analyzer]
rules = ["no-duplicate-ids", "accessibility"]
```

Or via command line:

```bash
orlint analyze --rules no-duplicate-ids,accessibility path/to/components/
```

### Quick Mode

For a fast preliminary check:

```bash
orlint analyze --quick path/to/components/
```

This runs only low-complexity rules for a faster result.

## Memory Optimization

Control memory usage for large codebases:

### Memory Limits

In your `.orlint.toml`:

```toml
[performance]
memory_limit = 1024  # Limit to 1024 MB
```

Or via command line:

```bash
orlint analyze --memory-limit 1024 path/to/components/
```

### Batch Processing

Process files in batches:

```bash
# Using zsh globbing
for dir in src/components/*/; do
  orlint analyze "$dir"
done
```

Or with find:

```bash
find src/components -name "*.orbit" -print0 | xargs -0 -n 50 orlint analyze
```

### Minimizing Rule Memory Usage

Some rules consume more memory than others. For very large codebases, consider:

```toml
[analyzer]
exclude_rules = ["memory-intensive-rule"]
```

## CI/CD Optimization

Optimize analyzer performance in CI/CD pipelines:

### Only Analyze Changed Files

```yaml
# GitHub Actions example
- name: Get changed files
  id: changed-files
  uses: tj-actions/changed-files@v23
  with:
    files: '**/*.orbit'

- name: Run Orbit Analyzer on changed files
  if: steps.changed-files.outputs.all_changed_files
  run: |
    orlint analyze ${{ steps.changed-files.outputs.all_changed_files }}
```

### Split Analysis Across Jobs

For monorepos or very large projects:

```yaml
jobs:
  analyze-critical:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Analyze critical components
        run: orlint analyze src/components/critical/

  analyze-experimental:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Analyze experimental components
        run: orlint analyze src/components/experimental/
```

### Caching

Use caching to speed up repeated analyses:

```yaml
- name: Cache Orbit Analyzer data
  uses: actions/cache@v3
  with:
    path: .orbit-cache
    key: ${{ runner.os }}-orlint-${{ hashFiles('**/*.orbit') }}
    restore-keys: |
      ${{ runner.os }}-orlint-
```

## Benchmarking

Measure and compare analyzer performance:

### Basic Timing

```bash
time orlint analyze path/to/components/
```

### Detailed Performance Report

```bash
orlint analyze --performance-report path/to/components/
```

This generates a detailed breakdown of time spent on each phase of analysis.

### Comparing Configurations

Use the benchmark helper script:

```bash
# From the orlint directory
./scripts/benchmark.sh --config1 config1.toml --config2 config2.toml path/to/components/
```

### Memory Profiling

```bash
# On macOS
/usr/bin/time -l orlint analyze path/to/components/

# On Linux
/usr/bin/time -v orlint analyze path/to/components/
```

## Advanced Optimization Techniques

### Custom Rules Optimization

If you've created custom rules, ensure they're optimized:

1. Use early returns for non-applicable cases
2. Avoid excessive string allocations
3. Reuse data structures when possible
4. Consider adding a `is_applicable` method to quickly skip inapplicable components

### Disabling Expensive Built-in Rules

Some built-in rules are more expensive:

```toml
[analyzer]
exclude_rules = [
  "deep-state-analysis",  # Expensive state graph analysis
  "cross-component-validation",  # Requires whole-project analysis
  "advanced-accessibility"  # Complex DOM traversal
]
```

### Parser Optimization

If you're only interested in specific aspects of components:

```bash
orlint analyze --parser-mode lightweight path/to/components/
```

This uses a faster parser that extracts only essential information.
