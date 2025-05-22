#!/bin/bash

# Verify the local development workspace setup for orlint
# Usage: ./verify-workspace.sh

set -e

# Create workspace directory if it doesn't exist
mkdir -p ../orbitrs-workspace

# Check if orbit dependency is present
if [ ! -d "../orbit" ]; then
    echo "Error: orbit dependency not found"
    echo "Please clone https://github.com/orbitrs/orbit into the parent directory"
    exit 1
fi

# Verify Cargo.toml setup
if ! grep -q '\[patch."https://github.com/orbitrs/orbit.git"\]' .cargo/config.toml 2>/dev/null; then
    echo "Warning: Local patch for orbit not found in .cargo/config.toml"
    mkdir -p .cargo
    echo '[patch."https://github.com/orbitrs/orbit.git"]' >> .cargo/config.toml
    echo 'orbit = { path = "../orbit" }' >> .cargo/config.toml
fi

# Check for required system dependencies
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if ! dpkg -l | grep -q "libfontconfig1-dev\|libfreetype6-dev"; then
        echo "Warning: Required system dependencies not found. Please install:"
        echo "sudo apt-get install -y libfontconfig1-dev libfreetype6-dev"
    fi
fi

echo "Workspace verification completed!"
