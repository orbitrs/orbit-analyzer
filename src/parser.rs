// Parser for orbit-analyzer

use crate::reporter::Issue;
use crate::AnalyzerError;
use crate::Result;
use orbit::parser::OrbitFile;

/// Parse an .orbit file
pub fn parse_orbit_file(content: &str, file_path: &str) -> Result<OrbitFile> {
    orbit::parser::OrbitParser::parse_file(content, file_path)
        .map_err(|e| AnalyzerError::Parser(e.to_string()))
}

/// Extract component name from an .orbit file
pub fn extract_component_name(file: &OrbitFile) -> Option<String> {
    // Look for a struct definition in the script section
    for line in file.script.lines() {
        let line = line.trim();
        if line.starts_with("pub struct ") {
            // Extract the struct name
            let struct_name = line.strip_prefix("pub struct ")?
                .split_whitespace()
                .next()?
                .trim_end_matches('{')
                .trim();
                
            return Some(struct_name.to_string());
        }
    }
    
    None
}

/// Parse a component's properties
pub fn parse_component_props(file: &OrbitFile) -> Result<Vec<PropInfo>> {
    let mut props = Vec::new();
    
    // Look for the props struct definition in the script section
    let mut in_props_struct = false;
    let mut braces_level = 0;
    
    for line in file.script.lines() {
        let line = line.trim();
        
        // Find the props struct
        if !in_props_struct && line.contains("pub struct ") && line.contains("Props") {
            in_props_struct = true;
            braces_level = line.chars().filter(|c| *c == '{').count() as i32;
            continue;
        }
        
        // If we're in the props struct, extract the props
        if in_props_struct {
            braces_level += line.chars().filter(|c| *c == '{').count() as i32;
            braces_level -= line.chars().filter(|c| *c == '}').count() as i32;
            
            // End of props struct
            if braces_level <= 0 {
                break;
            }
            
            // Parse property definition
            if line.starts_with("pub ") {
                if let Some(prop_info) = parse_prop_line(line) {
                    props.push(prop_info);
                }
            }
        }
    }
    
    Ok(props)
}

/// Information about a property
#[derive(Debug, Clone)]
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
