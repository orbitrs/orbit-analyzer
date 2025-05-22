# Orbit Analyzer Development Guide

## Project Setup

This document provides information for developers working on the Orbit Analyzer project.

## Dependency Management

### OrbitRS Dependency

The Orbit Analyzer project depends on the OrbitRS library. Due to the active development of both projects, we've set up a specific dependency configuration to handle local development and CI workflows:

#### Main Dependency Configuration

In `Cargo.toml`, we define the OrbitRS dependency with the git repository:

```toml
orbitrs = { git = "https://github.com/orbitrs/orbitrs.git", branch = "main" }
```

This configuration:
- Uses the `main` branch of the orbitrs repository
- Does not specify a version, allowing Cargo to use whatever version is in the git repository
- Will be used when publishing the crate or when no local patch is available

#### Local Development Override

For local development, we use a path override in `.cargo/config.toml`:

```toml
[patch."https://github.com/orbitrs/orbitrs.git"]
orbitrs = { path = "../orbitrs" }
```

This patch configuration:
- Overrides the git dependency with a local path
- Allows you to work with local changes in the orbitrs repository
- Only applies to your local development environment
- Maintains the relative path structure that matches our monorepo structure

#### Version Management

When the orbitrs library undergoes changes:

1. For local development, you don't need to worry about versions as the local path override will use your current local copy.

2. For git dependencies, by not specifying a version, Cargo will automatically use the actual version from the specified branch.

3. This approach eliminates version conflicts between the git branch and manually specified versions.

#### Potential Version Conflicts

By removing the version specification from the git dependency, we eliminate the most common source of version conflicts. If you still experience dependency issues:

1. Ensure your local copy of orbitrs is up to date with the main branch
2. Run `cargo update -p orbitrs` to force Cargo to refresh the dependency
3. If needed, run `cargo clean` followed by `cargo build` to completely rebuild the dependency tree

#### CI Workflow Configuration

Our CI workflow creates the appropriate directory structure by:
1. Checking out both repositories in the right relative locations
2. Ensuring the local path override works correctly in CI

This setup allows seamless work between local development and CI environments without manual configuration changes.
