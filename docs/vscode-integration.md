# VSCode Integration

This guide shows how to integrate Orbit Analyzer with Visual Studio Code for real-time linting of your .orbit files.

## Prerequisites

Before setting up the VSCode integration, make sure you have:

1. Installed the Orbit Analyzer CLI tool
2. Visual Studio Code installed on your system

## Integration Options

There are two main ways to integrate Orbit Analyzer with VSCode:

1. Using the [Orbit VSCode Extension](#orbit-vscode-extension) (Recommended)
2. [Manual Integration](#manual-integration) with VS Code tasks

## Orbit VSCode Extension

The official Orbit VSCode Extension provides first-class support for .orbit files, including:

- Syntax highlighting for .orbit files
- Real-time linting using Orbit Analyzer
- Quick fixes for common issues
- Hover information for components and props
- Go-to-definition and find-references support
- Auto-completion for components, props, and events

### Installation

1. Open VSCode
2. Go to the Extensions view (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "Orbit UI"
4. Click "Install" on the "Orbit UI Tools" extension

### Configuration

The extension reads your project's `.orlint.toml` file automatically. You can also configure the extension through VSCode settings:

1. Open VSCode settings (Ctrl+, / Cmd+,)
2. Search for "Orbit"
3. Configure settings such as:
   - `orbit.analyzer.enable`: Enable/disable the analyzer
   - `orbit.analyzer.rules`: Specify which rules to enable
   - `orbit.analyzer.severities`: Configure rule severities

### Usage

Once installed, the extension automatically activates for any .orbit files. Linting issues will appear as squiggly underlines in your code and in the Problems panel.

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

1. Make sure the `orlint` CLI is in your PATH and works from the terminal
2. Check that your .orbit files have the correct format
3. Verify your `.orlint.toml` configuration is valid
4. Look for error messages in the OUTPUT panel (View > Output) with "Tasks" selected

## Additional Resources

- [VSCode Tasks Documentation](https://code.visualstudio.com/docs/editor/tasks)
- [Orbit Analyzer Troubleshooting Guide](troubleshooting.md)
- [Creating Custom Lint Rules](custom-lint-rules.md)
