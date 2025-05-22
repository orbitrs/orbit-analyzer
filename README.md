# Orbit Analyzer

![CI Status](https://github.com/orbitrs/orlint/actions/workflows/ci.yml/badge.svg)
![Release Status](https://github.com/orbitrs/orlint/actions/workflows/release.yml/badge.svg)
[![codecov](https://codecov.io/gh/orbitrs/orlint/branch/main/graph/badge.svg?token=CODECOV_TOKEN)](https://codecov.io/gh/orbitrs/orlint)
[![crates.io](https://img.shields.io/crates/v/orlint.svg)](https://crates.io/crates/orlint)

## 📖 Project Overview

**Orbit Analyzer** is a Rust-based static analysis tool designed specifically to parse, analyze, and validate `.orbit` component files. It helps developers maintain code quality, enforce best practices, and catch errors early within the Orbit UI framework ecosystem.

Orbit Analyzer enables deeper insights into your codebase by combining syntax checking, semantic analysis, and customizable linting rules, all tailored to the unique `.orbit` file format and Rust-powered component logic.

---

## 🎯 Goals & Purpose

* **Improve Code Quality:** Detect syntax errors, inconsistencies, and anti-patterns in Orbit components.
* **Enhance Developer Productivity:** Provide actionable feedback through warnings, errors, and suggestions.
* **Customizable Rules:** Allow teams to define and enforce their own style guides and conventions.
* **Integrate Seamlessly:** Fit into CI/CD pipelines for automated code health checks.
* **Support Orbit Growth:** Facilitate scalability and maintainability of large Orbit codebases.

---

## 🚀 Features

* **Syntax Validation:** Parse `.orbit` files and verify their structure (markup, style, script).
* **Semantic Analysis:** Analyze Rust code embedded in components for logical correctness.
* **Renderer-Aware Analysis:** 
    * Validate component metadata for renderer preferences (Skia/WGPU).
    * Check for optimal usage of renderer-specific features.
* **Custom Lint Rules:** Easily add or modify linting rules, including renderer-specific ones.
* **Code Quality Enforcement:**
    * Component naming conventions
    * Type checking for props and state
    * State initialization verification
    * Public method requirements
* **Multiple Output Formats:**
    * Text console output
    * JSON for programmatic consumption
    * HTML reports with issue visualization
* **Performance Features:**
    * Parallel analysis for large codebases
    * Configurable severity levels
    * Rule filtering options
* **IDE Integration:**
    * VSCode extension for real-time linting
    * Syntax highlighting for `.orbit` files
    * Quick fixes for common issues
* **Cross-platform CLI:** Command-line interface for easy integration and usage.

## 🧩 Development

For information about setting up the project for development, dependency management, and contributing, see the [Development Guide](DEVELOPMENT.md).

## 📚 Documentation

For detailed documentation on using and extending Orbit Analyzer, check out our [Documentation](docs/README.md), which includes:

* [Creating Custom Lint Rules](docs/custom-lint-rules.md)
* [CI Integration Guide](docs/ci-integration.md)

---
* **WASM Support:** Potential to run analysis in-browser or in other WASM environments.

---

## 🧱 Architecture Overview

* **Parser:** Converts `.orbit` files into an abstract syntax tree (AST).
* **Analyzer:** Traverses the AST to perform validation and gather metrics.
* **Rule Engine:** Applies built-in and custom linting rules on the AST.
* **Reporter:** Formats and outputs analysis results (console, JSON, etc.).
* **CLI Tool:** User-facing command-line utility to run analyses on projects or files.

---

## 🛠️ Getting Started

### Prerequisites

* Rust toolchain installed (stable recommended)
* Basic familiarity with Orbit components and Rust

### Installation

```bash
git clone https://github.com/orbitrs/orlint.git
cd orlint
cargo build --release
```

### Usage

Analyze your Orbit project:

```bash
./target/release/orlint path/to/your/components
```

Generate a JSON report:

```bash
./target/release/orlint --output report.json path/to/your/components
```

---

## ⚙️ Configuration

Use a configuration file `.orlint.toml` to customize behavior:

```toml
[settings]
syntax_check = true
semantic_analysis = true
metrics_enabled = true

[renderer_analysis]
enabled = true
default_renderer = "auto" # options: "auto", "skia", "wgpu"
check_renderer_metadata = true

[lint]
no_unused_variables = true
prefer_const_let = true
max_function_length = 50
# Renderer-specific lint rules
wgpu_specific_optimizations = "warn"
skia_vector_usage = "info"

[report]
format = "json"  # options: "json", "text"
output_path = "analysis_report.json"
```

---

## 🛣️ Roadmap & Future Plans

* **v0.1:** Basic syntax validation, CLI tool, simple reporting.
* **v0.2:** Semantic analysis, enhanced lint rules, configurable CLI options.
* **v0.3:** Custom lint rule plugins, integration with Orbit CLI (`orbiton`).
* **v0.4:** IDE integration (VSCode extension), improved performance optimizations.
* **v1.0:** Stable release, comprehensive documentation, community-driven rules repository.

---

## 🤝 Contribution Guidelines

We welcome contributions from the community!

* Fork the repo and open pull requests for new features or bug fixes.
* Follow Rust coding standards and Orbit conventions.
* Write tests for new lint rules or features.
* Join discussions and help improve the analyzer on GitHub Issues.

---

## 📚 References & Resources

* [Orbit Framework](https://github.com/orbitrs/orbit)
* [Rust Lang](https://www.rust-lang.org/)
* [Rust Clippy](https://github.com/rust-lang/rust-clippy)
* [Static Analysis Fundamentals](https://en.wikipedia.org/wiki/Static_program_analysis)

---

## 📞 Contact & Support

For questions, feature requests, or support, please open an issue on the GitHub repository:
[https://github.com/orbitrs/orlint/issues](https://github.com/orbitrs/orlint/issues)

---

## 🧩 Development

For information about setting up the project for development, dependency management, and contributing, see the [Development Guide](DEVELOPMENT.md).

## 📚 Documentation

- [Getting Started](docs/README.md)
- [Custom Lint Rules](docs/custom-lint-rules.md)
- [CI Integration](docs/ci-integration.md)

---

**Orbit Analyzer** — Helping you write better Orbit code, one lint at a time.
