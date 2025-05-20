# GitHub Copilot Instructions for orbit-analyzer

## About This Project

This project, `orbit-analyzer`, is part of the Orbit UI framework ecosystem.
- **Primary Language:** Rust
- **Core Focus:** A static analysis tool for `.orbit` component files, ensuring code quality and adherence to best practices within the Orbit framework.

Refer to the main `README.md` in the project root for detailed information on its architecture, goals, and specific conventions.

## Key Technologies & Concepts

- **Orbit Framework:** Understand the structure of `.orbit` files (markup, Rust logic, optional CSS/JS/HTML) as this tool analyzes them.
- **Static Analysis:** Familiarize yourself with concepts like AST (Abstract Syntax Tree) traversal, syntax validation, semantic analysis, and linting rules.
- **Rust Best Practices:** Adhere to Rust idioms, error handling patterns, and module organization.
- **Project-Specific Conventions:** Pay attention to the architecture (Parser, Analyzer, Rule Engine, Reporter, CLI) and configuration options (`.orbit-analyzer.toml`).
- **Renderer-Aware Analysis:** The analyzer can validate component metadata for renderer preferences (Skia/WGPU) and check for optimal usage of renderer-specific features.

## When Assisting:

- **Consult READMEs:** Always check the `README.md` in the `orbit-analyzer` project root before providing solutions.
- **Code Generation:**
    - For Rust code related to parsing, analysis, or linting rules, ensure it aligns with the project's architecture.
    - When suggesting new lint rules, consider their impact and configurability.
- **Configuration:** Be aware of the `.orbit-analyzer.toml` configuration file and its settings for syntax checks, semantic analysis, metrics, and renderer-specific analysis.
- **Integration:** Understand that `orbit-analyzer` is designed to integrate with the `orbiton` CLI and potentially IDEs.

By following these guidelines, you can provide more accurate and helpful assistance for the `orbit-analyzer` project.
