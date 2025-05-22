# Renderer-Specific Analysis Guide

The Orbit UI framework supports multiple rendering backends, including Skia and WebGPU. The Orbit Analyzer includes specialized rules and checks for each renderer to help ensure your components are optimized and compatible.

## Table of Contents

- [Overview](#overview)
- [Renderer Metadata](#renderer-metadata)
- [Skia-Specific Rules](#skia-specific-rules)
- [WebGPU-Specific Rules](#webgpu-specific-rules)
- [Cross-Renderer Compatibility](#cross-renderer-compatibility)
- [Performance Considerations](#performance-considerations)
- [Custom Renderer Rules](#custom-renderer-rules)

## Overview

Orbit Analyzer can identify renderer-specific issues in your `.orbit` files, such as:

- Using features not supported by the target renderer
- Missing performance optimizations for a specific renderer
- Potential compatibility issues across renderers
- Inefficient usage patterns for a particular rendering backend

## Renderer Metadata

Orbit components can specify renderer preferences using metadata:

```orbit
<orbit>
  <metadata>
    <renderer>skia</renderer>
    <fallback-renderer>webgpu</fallback-renderer>
  </metadata>
  
  <!-- Component content -->
</orbit>
```

The Orbit Analyzer validates this metadata and performs renderer-specific checks based on it.

## Skia-Specific Rules

When a component targets the Skia renderer, the analyzer applies these specialized rules:

### skia-path-optimization

Checks that path definitions use Skia-optimized formats for better performance.

**Good example:**
```orbit
<path d="M10,10 L20,20" skia:hint="fast" />
```

**Issue example:**
```orbit
<path d="M10.0001,10.0002 L20.0001,20.0002" />
```

### skia-text-rendering

Ensures text elements use Skia's recommended text rendering options.

**Good example:**
```orbit
<text skia:text-rendering="fast">Hello World</text>
```

**Issue example:**
```orbit
<text style="font-kerning: auto;">Hello World</text>
```

### skia-image-format

Checks that images use formats well-supported by Skia.

**Good example:**
```orbit
<img src="image.png" skia:decode="hardware" />
```

**Issue example:**
```orbit
<img src="image.webp" />
```

## WebGPU-Specific Rules

When a component targets the WebGPU renderer, these specialized rules are applied:

### webgpu-shader-compatibility

Ensures any custom shaders are compatible with WebGPU.

**Good example:**
```orbit
<shader type="webgpu">
  @compute @workgroup_size(8, 8)
  fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    // WebGPU compatible shader code
  }
</shader>
```

**Issue example:**
```orbit
<shader type="glsl">
  void main() {
    // GLSL shader not compatible with WebGPU
  }
</shader>
```

### webgpu-texture-format

Checks that texture formats are compatible with WebGPU.

**Good example:**
```orbit
<texture webgpu:format="rgba8unorm" />
```

**Issue example:**
```orbit
<texture format="rgb10" />
```

## Cross-Renderer Compatibility

These rules check for cross-renderer compatibility issues:

### renderer-feature-compatibility

Identifies features that may not work across all renderers.

**Good example:**
```orbit
<filter type="blur" value="10px" />
```

**Issue example:**
```orbit
<filter type="displacement-map" values="..." />
```

### renderer-fallback-support

Ensures components with a fallback renderer don't use features incompatible with that fallback.

**Good example:**
```orbit
<!-- This component specifies WebGPU as fallback and avoids Skia-only features -->
<orbit>
  <metadata>
    <renderer>skia</renderer>
    <fallback-renderer>webgpu</fallback-renderer>
  </metadata>
  
  <rect width="100" height="100" fill="red" />
</orbit>
```

**Issue example:**
```orbit
<orbit>
  <metadata>
    <renderer>skia</renderer>
    <fallback-renderer>webgpu</fallback-renderer>
  </metadata>
  
  <!-- Using a Skia-specific feature, which won't work with the WebGPU fallback -->
  <filter skia:type="special-blur" />
</orbit>
```

## Performance Considerations

### renderer-performance-hint

Provides performance optimization suggestions for the specified renderer.

**Example output:**
```
Performance hint: Consider using 'hardware-pattern' attribute with Skia renderer for repeated background patterns
Position: line 42, column 3
Severity: Info
```

## Custom Renderer Rules

You can create custom renderer-specific rules to match your project's requirements:

```rust
use orlint::{Rule, Component, Issue, Severity, Location};

struct CustomSkiaRule;

impl Rule for CustomSkiaRule {
    fn name(&self) -> &'static str {
        "custom-skia-optimization"
    }

    fn description(&self) -> &'static str {
        "Checks for project-specific Skia optimizations"
    }

    fn check(&self, component: &Component) -> Vec<Issue> {
        // Only apply this rule to components targeting Skia
        if component.renderer != "skia" {
            return vec![];
        }
        
        // Custom logic to check the component
        // ...

        // Return any issues found
        vec![
            Issue {
                rule: self.name().to_string(),
                message: "Consider using our custom Skia optimizations".to_string(),
                severity: Severity::Info,
                location: Location {
                    file: component.path.clone(),
                    line: 42,
                    column: 10,
                },
            }
        ]
    }
}
```

## Configuring Renderer Rules

You can enable or disable specific renderer rules in your `.orlint.toml` configuration:

```toml
[analyzer.renderers]
# Enable all Skia-specific rules
skia = true

# Enable only specific WebGPU rules
webgpu = ["webgpu-shader-compatibility"]

# Disable specific cross-renderer rules
cross_renderer = { exclude = ["renderer-fallback-support"] }
```

## Command Line Usage

To run only renderer-specific checks:

```bash
orlint analyze --renderer skia src/components/
```

To analyze for compatibility with multiple renderers:

```bash
orlint analyze --renderers skia,webgpu src/components/
```
