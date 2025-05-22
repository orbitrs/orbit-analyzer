#!/bin/bash
# Build script for the Orbit UI Tools VSCode extension

set -e

# Navigate to the vscode directory
cd "$(dirname "$0")"

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "Error: npm is required but not installed."
    exit 1
fi

# Check if node is installed
if ! command -v node &> /dev/null; then
    echo "Error: node.js is required but not installed."
    exit 1
fi

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# Build the extension
echo "Building extension..."
npm run package

# Check if vsce is installed
if command -v vsce &> /dev/null; then
    echo "Packaging VSIX file..."
    vsce package
else
    echo "Warning: vsce is not installed. Skipping VSIX packaging."
    echo "To install vsce, run: npm install -g @vscode/vsce"
fi

echo "Build complete! VSIX file is in the vscode directory."
