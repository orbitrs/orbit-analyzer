#!/bin/bash
# Setup script for orlint development environment

set -e

# Display banner
echo "======================================================"
echo "  Setting up orlint development environment"
echo "======================================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || ! grep -q "name = \"orlint\"" Cargo.toml; then
  echo "Error: This script must be run from the orlint directory"
  exit 1
fi

# Check if orbitrs exists in the parent directory
if [ ! -d "../orbitrs" ]; then
  echo "Error: orbitrs repository not found in parent directory"
  echo "Please clone the orbitrs repository to the same parent directory as orlint"
  exit 1
fi

# Ensure .cargo directory exists
mkdir -p .cargo

# Create or update the config.toml file
cat > .cargo/config.toml << EOL
[patch."https://github.com/orbitrs/orbitrs.git"]
orbitrs = { path = "../orbitrs" }
EOL

echo "Created .cargo/config.toml with local orbitrs patch"

# Check and install system dependencies
install_system_dependencies() {
  if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS, checking for required dependencies..."
    
    # Check for Homebrew
    if ! command -v brew &> /dev/null; then
      echo "Homebrew not found. Please install it from https://brew.sh/"
      exit 1
    fi
    
    # Install dependencies
    echo "Installing fontconfig and freetype via Homebrew..."
    brew install fontconfig freetype
    
  elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Detected Linux, checking for required dependencies..."
    
    # Check for apt
    if ! command -v apt-get &> /dev/null; then
      echo "apt-get not found. Please install libfontconfig1-dev and libfreetype6-dev manually."
      exit 1
    fi
    
    # Install dependencies
    echo "Installing libfontconfig1-dev and libfreetype6-dev..."
    sudo apt-get update
    sudo apt-get install -y libfontconfig1-dev libfreetype6-dev
    
  else
    echo "Unsupported OS. Please install the required dependencies manually."
    echo "  - For Windows: Use vcpkg to install fontconfig and freetype"
    echo "  - For other systems: Install the equivalent of fontconfig-dev and freetype-dev"
  fi
}

# Prompt for system dependency installation
read -p "Would you like to install required system dependencies? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  install_system_dependencies
fi

# Test the setup
echo "Testing the setup with cargo check..."
cargo check

if [ $? -eq 0 ]; then
  echo "======================================================"
  echo "  🎉 Setup completed successfully!"
  echo "======================================================"
  echo "You can now build and run orlint with:"
  echo "  cargo build"
  echo "  cargo run"
else
  echo "======================================================"
  echo "  ⚠️ Setup completed with warnings"
  echo "======================================================"
  echo "Cargo check encountered issues. Please check the output above."
fi
