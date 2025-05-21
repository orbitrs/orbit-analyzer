// Parser for orbit-analyzer

use crate::AnalyzerError;
use crate::Result;
use orbit::parser::{OrbitAst, OrbitParser};

/// Parse an .orbit file
pub fn parse_orbit_file(content: &str, _file_path: &str) -> Result<OrbitAst> {
    OrbitParser::parse(content).map_err(|e| AnalyzerError::Parser(e.to_string()))
}

/// Extract component name from an .orbit file
#[allow(dead_code)]
pub fn extract_component_name(ast: &OrbitAst) -> Option<String> {
    // Get component name from script node
    if !ast.script.component_name.is_empty() {
        return Some(ast.script.component_name.clone());
    }

    None
}

/// Parse a component's properties
#[allow(dead_code)]
pub fn parse_component_props(ast: &OrbitAst) -> Result<Vec<PropInfo>> {
    // In the orbit crate's current implementation, we can directly access the props from the AST
    let mut props = Vec::new();

    // Convert the props from the AST to our PropInfo format
    for prop in &ast.script.props {
        props.push(PropInfo {
            name: prop.name.clone(),
            type_name: prop.ty.clone(),
            required: prop.required,
            doc: None, // AST doesn't currently store doc comments
        });
    }

    // Return the collected properties

    Ok(props)
}

/// Information about a property
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PropInfo {
    /// Property name
    pub name: String,
    /// Property type
    pub type_name: String,
    /// Whether the property is required
    pub required: bool,
    /// Property documentation
    pub doc: Option<String>,
}

/// Parse a property definition line
#[allow(dead_code)]
fn parse_prop_line(line: &str) -> Option<PropInfo> {
    // Parse lines like "pub name: String,"
    let line = line.strip_prefix("pub ")?;
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }

    let name = parts[0].trim().to_string();
    let type_part = parts[1].trim().trim_end_matches(',');

    // Check if it's an Option
    let (type_name, required) = if type_part.starts_with("Option<") {
        let inner_type = type_part.strip_prefix("Option<")?.strip_suffix(">")?;
        (inner_type.to_string(), false)
    } else {
        (type_part.to_string(), true)
    };

    Some(PropInfo {
        name,
        type_name,
        required,
        doc: None,
    })
}
