# Orbit Analyzer Development Guide

## Project Setup

This document provides information for developers working on the Orbit Analyzer project.

### Automated Setup

We provide a setup script that will configure your development environment:

```bash
# From the orbit-analyzer directory
./scripts/setup-environment.sh
```

This script will:
1. Verify the required repository structure
2. Create the necessary .cargo/config.toml file
3. Install required system dependencies
4. Test the setup with cargo check

### System Dependencies

The Orbit Analyzer depends on the OrbitRS library, which uses Skia for rendering. This requires the following system dependencies:

#### macOS
```bash
brew install fontconfig freetype
```

#### Ubuntu/Debian Linux
```bash
sudo apt-get install -y libfontconfig1-dev libfreetype6-dev
```

#### Windows
Windows builds typically bundle these dependencies or use vcpkg to manage them.

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

## Continuous Integration & Deployment

### CI Workflow

Our CI workflow is inspired by the reusable `rust-ci.yml` workflow from the `orbit-workflows` repository but includes custom modifications to handle the orbitrs dependency:

1. We create a special directory structure that mirrors our local development environment
2. We check out both the orbit-analyzer and orbitrs repositories
3. We set up necessary system dependencies for building

This customization ensures that:
- The local path override in `.cargo/config.toml` works correctly in CI
- All necessary dependencies are available for compilation
- Tests can run on multiple platforms

The CI process runs:
- Format and lint checks on Ubuntu
- Tests across multiple platforms (Ubuntu, Windows, macOS)

### Release Process

Similarly, our release workflow follows the structure of the reusable `rust-release.yml` workflow but with customizations for our dependency needs. It handles:

1. Publishing to crates.io (when triggered)
2. Building documentation with proper dependency handling
3. Creating cross-platform artifacts, including ARM architecture support
4. Creating GitHub releases with assets

This setup allows seamless work between local development and CI environments without manual configuration changes.

## Troubleshooting

### Build Failures due to Missing Libraries

If you encounter linker errors like:
```
error: linking with `cc` failed: exit status: 1
/usr/bin/ld: cannot find -lfontconfig: No such file or directory
/usr/bin/ld: cannot find -lfreetype: No such file or directory
```

Make sure you have installed the required system dependencies listed in the "System Dependencies" section above. These are needed because the orbitrs library depends on Skia, which uses fontconfig and freetype for text rendering.

In CI environments, these dependencies are installed with:
```bash
sudo apt-get update
sudo apt-get install -y libfontconfig1-dev libfreetype6-dev
```

For macOS ARM (M1/M2) machines, you may need to ensure the libraries are properly linked:
```bash
brew install fontconfig freetype
```

### Version Conflicts

If you encounter dependency conflicts, try the following steps:
1. Run `cargo update` to update all dependencies
2. Check that your local orbitrs repository is up to date with the main branch
3. If you're seeing issues specific to the CI workflow, verify that the workflow is installing all necessary dependencies

### Supported Platforms

The CI/CD pipeline supports the following platforms:
- Linux (x86_64, aarch64)
- macOS (x86_64, arm64/M1)
- Windows (x86_64)

### Required Environment Secrets

For the release workflow to function properly, the following GitHub secrets should be configured:
- `CRATES_TOKEN`: API token for publishing to crates.io

### Local CI Testing

You can run CI tests locally using the scripts from the orbit-workflows repository:

```bash
# From the orbit-analyzer directory
../orbit-workflows/scripts/run-local-ci.sh
```

### Custom vs. Reusable Workflows

While most repositories in the orbitrs ecosystem use the reusable workflows from the `orbit-workflows` repository directly, orbit-analyzer uses custom workflows for these reasons:

1. **Dependency Management**: orbit-analyzer has a direct dependency on orbitrs that requires a specific directory structure
2. **System Dependencies**: We need specific system libraries (fontconfig, freetype) for building
3. **Multi-repository Setup**: We need to check out both repositories in a specific arrangement

Our custom workflows maintain consistency with the reusable workflows by following the same structure and job organization, but with the necessary adaptations for our specific requirements.
