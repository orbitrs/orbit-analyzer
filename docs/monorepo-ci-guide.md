# Monorepo CI Setup Guide

This document explains how the CI system is configured to handle the dependency between `orlint` and other parts of the Orbit Framework in the monorepo structure.

## Overview

The `orlint` CI workflow is designed to work in the Orbit Framework monorepo structure, where several related projects (orbitrs, orbiton, orbitkit, etc.) are developed together. This setup presents unique challenges for CI, particularly because of local dependency paths.

## Dependency Structure

In local development, the `orlint` depends on the `orbitrs` crate, which is typically referenced with a local path:

```toml
# In orlint/Cargo.toml
[dependencies]
orbitrs = { path = "../orbitrs" }
```

In CI environments, we need to replicate this structure to ensure tests run consistently.

## CI Workflow Strategy

Our CI workflow uses the following approach:

1. **Monorepo Checkout**: We check out the entire monorepo into a consistent directory structure.

2. **Cargo Path Configuration**: We set up `.cargo/config.toml` with patch definitions to ensure the local dependencies can be resolved:

   ```toml
   [patch.crates-io]
   orbitrs = { path = "../orbitrs" }
   ```

3. **Workspace-aware Caching**: We use `Swatinem/rust-cache@v2` with workspace awareness to speed up builds while respecting the monorepo structure.

## Job Structure

The CI workflow includes the following jobs:

- **Format & Lint**: Checks code formatting and runs clippy
- **Test**: Runs unit and integration tests
- **Security Audit**: Checks for security vulnerabilities
- **Dependency Check**: Identifies outdated dependencies
- **Benchmarks**: Runs performance benchmarks (only on the main branch)

## Handling Wayland Dependency Issues

The CI workflow includes special handling for the wayland-backend dependency, which sometimes attempts to use unstable Rust features. We use a conditional compilation flag to skip these problematic tests in CI:

```rust
// Example of conditionally skipping tests
#[cfg(not(ignore_wayland_tests))]
#[test]
fn test_that_uses_wayland() {
    // Test implementation
}
```

In the CI, we set the `RUSTFLAGS` environment variable to activate this flag:

```yaml
- name: Run tests
  run: |
    RUSTFLAGS="--cfg ignore_wayland_tests" cargo test --all-features
```

## Troubleshooting CI Issues

If CI builds are failing, here are common issues and solutions:

1. **Missing Dependencies**: Ensure system dependencies are installed in the workflow.

2. **Path Resolution**: Check that the path structure in CI matches what's expected in `.cargo/config.toml`.

3. **Feature Flag Conflicts**: Some dependencies might have conflicting feature requirements. Use conditional compilation to work around these.

4. **Benchmark Failures**: Benchmarks can sometimes fail due to environmental factors. We use `|| true` to prevent these from failing the build.

## Adding New Dependencies

When adding new dependencies to `orlint`:

1. Update both the `Cargo.toml` and CI workflow if necessary
2. Consider if any new system dependencies need to be installed in CI
3. Test locally with the same structure as CI to ensure compatibility

## Further Reading

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust Monorepo Best Practices](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)
- [Cargo Workspace Documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html)
