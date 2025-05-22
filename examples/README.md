# Orbit Analyzer Examples

This directory contains example `.orbit` component files that demonstrate the capabilities of the Orbit Analyzer.

## Files

- `Button.orbit`: A well-structured component following best practices
- `BadComponent.orbit`: A component with various issues that the analyzer will detect
- `RendererSpecific.orbit`: A component showcasing renderer-specific features
- `.orlint.toml`: Sample configuration file for the analyzer

## Running the Analyzer

To analyze these examples with the default configuration:

```bash
# From the orlint directory
cargo run -- analyze examples/**/*.orbit
```

To use the sample configuration:

```bash
# From the orlint directory
cargo run -- analyze --config examples/.orlint.toml examples/**/*.orbit
```

## Expected Results

### Button.orbit

The `Button.orbit` component should pass all linting rules, as it follows best practices:
- Uses PascalCase for component name
- All props have type annotations
- All state variables have type annotations and initial values
- Has public methods
- Has non-empty template content

### BadComponent.orbit

The `BadComponent.orbit` component should trigger multiple linting errors:
- Component name uses camelCase instead of PascalCase
- Props lack type annotations
- State variable lacks type annotation and initial value
- No public methods defined
- Contains renderer-specific styles that may not be compatible

### RendererSpecific.orbit

The `RendererSpecific.orbit` component demonstrates renderer-specific features:
- Uses WebGPU-specific attributes
- Includes renderer metadata
- Properly structured component that should only have renderer compatibility issues when analyzed with Skia as the default renderer
