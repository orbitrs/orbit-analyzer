# Orbit UI Tools for Visual Studio Code

Visual Studio Code extension for Orbit UI framework development. This extension provides syntax highlighting, linting, and other helpful features for working with `.orbit` files.

## Features

- Syntax highlighting for `.orbit` files
- Real-time linting using Orbit Analyzer
- Quick fixes for common issues
- Hover information for components and props
- Go-to-definition and find-references support
- Auto-completion for components, props, and events

## Requirements

- Orbit Analyzer CLI tool must be installed and available in your PATH
- Visual Studio Code 1.85.0 or newer

## Installation

### From Visual Studio Code Marketplace

1. Open VS Code
2. Go to Extensions view (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "Orbit UI Tools"
4. Click "Install"

### Manual Installation

1. Download the `.vsix` file from the [latest release](https://github.com/orbitrs/orlint/releases)
2. Open VS Code
3. Go to Extensions view
4. Click on the "..." menu in the top-right corner
5. Select "Install from VSIX..."
6. Choose the downloaded `.vsix` file

## Configuration

This extension can be configured through VS Code settings:

```json
{
  "orbit.analyzer.enable": true,
  "orbit.analyzer.path": "orlint",
  "orbit.analyzer.rules": [], // Empty array = use all rules
  "orbit.analyzer.configPath": "", // Path to custom config file
  "orbit.analyzer.validateOnSave": true,
  "orbit.analyzer.validateOnType": false,
  "orbit.analyzer.severities": {
    "component-naming": "warning",
    "prop-type-required": "error"
  }
}
```

## Usage

Once installed, the extension automatically activates for any `.orbit` files. Linting issues will appear as squiggly underlines in your code and in the Problems panel.

### Commands

- `Orbit: Analyze Current File` - Analyze the current file
- `Orbit: Analyze Workspace` - Analyze all `.orbit` files in the workspace

## Building from Source

To build this extension from source:

1. Clone the repository:
   ```bash
   git clone https://github.com/orbitrs/orlint.git
   cd orlint/vscode
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the extension:
   ```bash
   npm run package
   ```

This will generate a `.vsix` file in the `vscode` directory that you can install in VS Code.

## Contributing

Contributions are welcome! Please see the [contributing guidelines](../DEVELOPMENT.md) for more information.

## License

This extension is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
