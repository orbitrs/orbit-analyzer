# Troubleshooting Guide

This document covers common issues you might encounter when using Orbit Analyzer and how to resolve them.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Dependency Problems](#dependency-problems)
- [Analysis Failures](#analysis-failures)
- [Performance Issues](#performance-issues)
- [Common Rule Errors](#common-rule-errors)
- [CI/CD Integration Issues](#cicd-integration-issues)
- [Cross-Platform Compatibility](#cross-platform-compatibility)

## Installation Issues

### Cargo Installation Failures

**Problem**: `cargo install orlint` fails with compilation errors.

**Solution**:

1. Ensure you have the required system dependencies:
   - For Linux: `sudo apt-get install -y libfontconfig1-dev libfreetype6-dev`
   - For macOS: `brew install fontconfig freetype`

2. Try with specific version:
   ```bash
   cargo install orlint --version=0.1.9
   ```

3. Install from source:
   ```bash
   git clone https://github.com/orbitrs/orlint.git
   cd orlint
   cargo install --path .
   ```

### Missing Dependencies

**Problem**: You see errors about missing libraries when running the analyzer.

**Solution**:

For Linux systems:
```bash
sudo apt-get update
sudo apt-get install -y libfontconfig1-dev libfreetype6-dev
```

For macOS:
```bash
brew install fontconfig freetype
```

For Windows, use vcpkg to install the required dependencies.

## Dependency Problems

### OrbitRS Version Conflicts

**Problem**: You see errors about version conflicts with the orbitrs dependency.

**Solution**:

1. Update your local orbitrs repository:
   ```bash
   cd ../orbitrs
   git pull origin main
   ```

2. Update the orlint dependencies:
   ```bash
   cd ../orlint
   cargo update -p orbitrs
   ```

3. If needed, perform a clean build:
   ```bash
   cargo clean
   cargo build
   ```

### Missing OrbitRS Repository

**Problem**: The analyzer can't find the orbitrs repository.

**Solution**:

1. Ensure the orbitrs repository is cloned as a sibling to orlint:
   ```bash
   cd ..
   git clone https://github.com/orbitrs/orbitrs.git
   ```

2. Run the setup script to configure the environment:
   ```bash
   cd orlint
   ./scripts/setup-environment.sh
   ```

## Analysis Failures

### Parser Errors

**Problem**: The analyzer reports parser errors for valid .orbit files.

**Solution**:

1. Verify the .orbit file syntax:
   ```bash
   orlint validate path/to/component.orbit
   ```

2. Check if you're using syntax from a newer version of Orbit than the analyzer supports:
   ```bash
   orlint --version
   ```

3. Update to the latest version of the analyzer:
   ```bash
   cargo install orlint --force
   ```

### False Positives

**Problem**: The analyzer reports issues that aren't actually problems.

**Solution**:

1. Use rule suppression comments in your code:
   ```
   <!-- orlint-disable-next-line no-duplicate-ids -->
   <div id="intentionalDuplicate">...</div>
   ```

2. Adjust your configuration to ignore specific rules:
   ```toml
   # .orlint.toml
   [analyzer]
   disabled_rules = ["no-duplicate-ids"]
   ```

3. Create a custom rule that better matches your project's requirements.

## Performance Issues

### Slow Analysis on Large Projects

**Problem**: Analysis takes too long on large projects.

**Solution**:

1. Use parallel analysis:
   ```bash
   orlint analyze --parallel path/to/components/
   ```

2. Use incremental analysis in CI:
   ```bash
   orlint analyze --incremental --git-base main path/to/components/
   ```

3. Only analyze specific directories:
   ```bash
   orlint analyze --include "src/components/critical/**/*.orbit"
   ```

### High Memory Usage

**Problem**: The analyzer uses too much memory on large projects.

**Solution**:

1. Process files in batches:
   ```bash
   find src -name "*.orbit" | xargs -n 10 orlint analyze
   ```

2. Use the `--memory-limit` option:
   ```bash
   orlint analyze --memory-limit 1024 path/to/components/
   ```

## Common Rule Errors

### no-duplicate-ids

**Problem**: The analyzer reports duplicate IDs but you need them for your use case.

**Solution**:

1. Use unique IDs with a common prefix instead:
   ```
   <div id="section1-header">...</div>
   <div id="section2-header">...</div>
   ```

2. Disable the rule for specific lines:
   ```
   <!-- orlint-disable-next-line no-duplicate-ids -->
   <div id="common">...</div>
   ```

### use-state-properly

**Problem**: Issues with state management detection.

**Solution**:

1. Ensure you're following Orbit's state management patterns:
   ```rust
   let count = use_state(|| 0);
   ```

2. If using a custom state management approach, create a custom rule or disable this rule.

## CI/CD Integration Issues

### GitHub Actions Failures

**Problem**: The analyzer works locally but fails in GitHub Actions.

**Solution**:

1. Make sure you install the required system dependencies:
   ```yaml
   - name: Install system dependencies
     run: |
       sudo apt-get update
       sudo apt-get install -y libfontconfig1-dev libfreetype6-dev
   ```

2. If using a path override for orbitrs, ensure the CI workflow correctly clones both repositories:
   ```yaml
   - name: Create workspace directory
     run: mkdir -p $GITHUB_WORKSPACE/orbitrs-workspace
   
   - name: Checkout orlint
     uses: actions/checkout@v4
     with:
       path: orbitrs-workspace/orlint
   
   - name: Checkout orbitrs
     uses: actions/checkout@v4
     with:
       repository: orbitrs/orbitrs
       path: orbitrs-workspace/orbitrs
   ```

### Exit Code Issues

**Problem**: The analyzer doesn't return a non-zero exit code when issues are found.

**Solution**:

Use the `--fail-on` option to control when the analyzer should exit with a non-zero code:

```bash
orlint analyze --fail-on warning path/to/components/
```

This will cause the analyzer to exit with code 1 if any warnings or errors are found.

## Cross-Platform Compatibility

### Windows-Specific Issues

**Problem**: Analysis or tests fail only on Windows, but not on Linux/macOS. Example error: `Unexpected token: Equal` or test failures related to file paths.

**Solution**:

- Orlint now uses platform-independent path handling and improved test file detection. If you encounter issues:
  1. Ensure you are using the latest version of Orlint (reinstall if needed).
  2. Check for any hardcoded path separators (`/` or `\`) in your custom scripts or CI workflows and use platform-agnostic methods where possible.
  3. If you see errors related to file parsing, try normalizing line endings (use LF `\n` if possible) and ensure your `.orbit` files do not use unsupported syntax.
  4. Report any remaining platform-specific bugs to the Orlint issue tracker with details about your OS and error output.

**Note:** The Orlint test suite and parser are now validated on Linux, macOS, and Windows in CI.
