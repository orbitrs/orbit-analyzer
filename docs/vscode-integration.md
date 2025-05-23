# VSCode Integration

This guide shows how to integrate Orbit Analyzer with Visual Studio Code for real-time linting of your .orbit files.

> **Note:** As of v0.1.x, Orbit Analyzer and the VSCode extension are fully cross-platform (Windows, macOS, and Linux). Path handling and test detection are robust across all supported operating systems. See the [Troubleshooting Guide](troubleshooting.md) for platform-specific tips.

## Prerequisites

Before setting up the VSCode integration, make sure you have:

1. Installed the Orbit Analyzer CLI tool (now cross-platform)
2. Visual Studio Code installed on your system

## Integration Options

There are two main ways to integrate Orbit Analyzer with VSCode:

1. Using the [Orbit VSCode Extension](#orbit-vscode-extension) (Recommended)
2. [Manual Integration](#manual-integration) with VS Code tasks

## Orbit VSCode Extension

The official Orbit VSCode Extension provides first-class support for .orbit files, including:

- Syntax highlighting for .orbit files
- Real-time linting using Orbit Analyzer (cross-platform)
- Quick fixes for common issues
- Hover information for components and props
- Go-to-definition and find-references support
- Auto-completion for components, props, and events
- **Support for new lint rules** (see [Roadmap](roadmap/implementation-phases.md))

### Installation

1. Open VSCode
2. Go to the Extensions view (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "Orbit UI"
4. Click "Install" on the "Orbit UI Tools" extension

![Extension Installation](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-extension-install.png)
*Figure 6: Installing the Orbit extension from the VS Code marketplace*

### Configuration

The extension reads your project's `.orlint.toml` file automatically. You can also configure the extension through VSCode settings:

1. Open VSCode settings (Ctrl+, / Cmd+,)
2. Search for "Orbit"
3. Configure settings such as:
   - `orbit.analyzer.enable`: Enable/disable the analyzer
   - `orbit.analyzer.rules`: Specify which rules to enable (including new rules like lifecycle method checks)
   - `orbit.analyzer.severities`: Configure rule severities

![Extension Settings](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-extension-settings.png)
*Figure 7: Orbit extension settings panel*

Key settings include:

| Setting | Description | Default |
|---------|-------------|---------|
| `orbit.linter.enabled` | Enable/disable linting | `true` |
| `orbit.linter.runOnSave` | Run linter on file save | `true` |
| `orbit.linter.executablePath` | Custom path to the orlint executable | `"orlint"` |
| `orbit.preview.enabled` | Enable component preview panel | `true` |
| `orbit.preview.refreshRate` | Preview refresh delay in ms | `500` |
| `orbit.format.enabled` | Enable code formatting | `true` |

> **Tip:** As new rules are added (e.g., lifecycle method checks, event handler naming, prop validation), you can enable or customize them in your `.orlint.toml` or via the extension settings. See the [Custom Lint Rules Guide](custom-lint-rules.md) and [Implementation Phases](roadmap/implementation-phases.md) for details.

### Usage

Once installed, the extension automatically activates for any .orbit files. Linting issues will appear as squiggly underlines in your code and in the Problems panel.

Here's what you can expect when using Orbit with Visual Studio Code:

![Orbit VS Code Integration Overview](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-vscode-overview.png)
*Figure 1: Orbit extension in VS Code showing component previews, linting, and syntax highlighting*

#### Key Features

1. **Syntax Highlighting** for `.orbit` files
   ![Syntax Highlighting](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-syntax-highlighting.png)
   *Figure 2: Detailed syntax highlighting for Orbit components*

2. **Live Error Checking** with inline diagnostics
   ![Error Checking](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-error-checking.png)
   *Figure 3: Real-time error diagnostics for Orbit components*

3. **Component Preview** with live reloading
   ![Component Preview](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-component-preview.png)
   *Figure 4: Live component preview panel*

4. **Smart Completions** for Orbit APIs and markup
   ![Smart Completions](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-completions.png)
   *Figure 5: Intelligent code completion for Orbit components*

## Manual Integration

If you prefer not to use the extension, you can manually integrate Orbit Analyzer using VSCode tasks.

### Creating a Task

1. Create a `.vscode/tasks.json` file in your project with the following content:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Orbit Analyzer: Lint Current File",
      "type": "shell",
      "command": "orlint analyze ${file}",
      "problemMatcher": {
        "owner": "orbit",
        "fileLocation": ["relative", "${workspaceFolder}"],
        "pattern": {
          "regexp": "^(.+):(\\d+):(\\d+): \\[(.+)\\] (.+) \\((.+)\\)$",
          "file": 1,
          "line": 2,
          "column": 3,
          "severity": 4,
          "message": 5,
          "code": 6
        }
      },
      "presentation": {
        "reveal": "silent",
        "panel": "dedicated"
      },
      "group": {
        "kind": "build",
        "isDefault": true
      }
    },
    {
      "label": "Orbit Analyzer: Lint All Files",
      "type": "shell",
      "command": "orlint analyze src/**/*.orbit",
      "problemMatcher": {
        "owner": "orbit",
        "fileLocation": ["relative", "${workspaceFolder}"],
        "pattern": {
          "regexp": "^(.+):(\\d+):(\\d+): \\[(.+)\\] (.+) \\((.+)\\)$",
          "file": 1,
          "line": 2,
          "column": 3,
          "severity": 4,
          "message": 5,
          "code": 6
        }
      },
      "presentation": {
        "reveal": "always",
        "panel": "dedicated"
      }
    }
  ]
}
```

### Running the Tasks

- To lint the current file: Press `Ctrl+Shift+B` / `Cmd+Shift+B`
- To lint all files: Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`), type "Run Task", and select "Orbit Analyzer: Lint All Files"

## Setting Up File Associations

To ensure VSCode recognizes .orbit files correctly:

1. Open VSCode settings (Ctrl+, / Cmd+,)
2. Search for "Files: Associations"
3. Add an entry: `"*.orbit": "html"`

This will provide basic syntax highlighting even without the extension.

## Troubleshooting

If you encounter issues with VSCode integration:

1. Make sure the `orlint` CLI is in your PATH and works from the terminal (cross-platform support is now robust)
2. Check that your .orbit files have the correct format
3. Verify your `.orlint.toml` configuration is valid (especially for new rules)
4. Look for error messages in the OUTPUT panel (View > Output) with "Tasks" selected
5. See the [Troubleshooting Guide](troubleshooting.md) for platform-specific issues and solutions

### Troubleshooting Extension Issues

If you encounter issues with the extension, try these steps:

1. **Check Extension Output**
   - Open the Output panel (`Ctrl+Shift+U` or `Cmd+Shift+U`)
   - Select "Orbit" from the dropdown menu
   - Check for error messages

   ![Extension Output](https://raw.githubusercontent.com/orbitrs/docs-assets/main/images/vscode/orbit-extension-output.png)
   *Figure 8: Checking extension output for diagnostics*

2. **Verify Orlint Installation**
   - Open a terminal in VS Code
   - Run `orlint --version` to verify the tool is installed and accessible

3. **Reload the Extension**
   - Open the Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`)
   - Search for and select "Developer: Reload Window"

## Roadmap and New Features

- See the [Implementation Phases](roadmap/implementation-phases.md) for details on new and upcoming lint rules (e.g., lifecycle method checks, event handler naming, prop validation, state mutation, etc.).
- The extension and CLI will continue to improve with enhanced reporting, rule customization, and performance optimizations.

## Additional Resources

- [VSCode Tasks Documentation](https://code.visualstudio.com/docs/editor/tasks)
- [Orbit Analyzer Troubleshooting Guide](troubleshooting.md)
- [Creating Custom Lint Rules](custom-lint-rules.md)
