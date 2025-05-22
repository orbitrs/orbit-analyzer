#!/bin/bash

# Run all CI checks locally for orlint
# Usage: ./run-local-ci.sh

set -e

echo "Running local CI checks for orlint..."

# Verify workspace setup
echo "Verifying workspace setup..."
./verify-workspace.sh

# Format code
echo "Checking code formatting..."
./verify-formatting.sh

# Run clippy
echo "Running clippy..."
cargo clippy --features ci --all-targets -- -D warnings

# Run tests with proper flags
echo "Running tests..."
RUSTFLAGS="--cfg ignore_wayland_tests" cargo test --all-features

# Build in release mode
echo "Building in release mode..."
cargo build --release

echo "Local CI checks completed successfully!"
