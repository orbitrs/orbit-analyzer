// Main entry point for the orlint CLI

use clap::{Args, Parser, Subcommand};
use orlint::{analyze_files_with_config, Config, Reporter, Severity, VERSION};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(author, version = VERSION, about, long_about = None)]
#[command(name = "orlint")]
#[command(about = "Static analysis tool for Orbit UI framework files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze .orbit files for issues
    Analyze(AnalyzeArgs),

    /// Validate .orbit files syntax only
    Validate(ValidateArgs),

    /// List available rules
    ListRules,
}

#[derive(Args)]
struct AnalyzeArgs {
    /// Files or directories to analyze (supports glob patterns)
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Output format (text, json, html)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Output file path (if not specified, output to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Custom configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Rules to enable (comma-separated list)
    #[arg(long)]
    rules: Option<String>,

    /// Rules to disable (comma-separated list)
    #[arg(long)]
    disable_rules: Option<String>,

    /// Minimum severity level to report (error, warning, info)
    #[arg(long, default_value = "warning")]
    min_severity: String,

    /// Run analysis in parallel
    #[arg(short, long)]
    parallel: bool,
}

#[derive(Args)]
struct ValidateArgs {
    /// Files or directories to validate (supports glob patterns)
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze(args) => analyze_command(args),
        Commands::Validate(args) => validate_command(args),
        Commands::ListRules => list_rules_command(),
    }
}

/// Execute the analyze command
fn analyze_command(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let mut config = match &args.config {
        Some(path) => Config::from_file(path)?,
        None => Config::find_and_load().unwrap_or_default(),
    };

    // Override configuration with command line arguments
    if let Some(rules) = args.rules {
        config.analyzer.enabled_rules = rules.split(',').map(|s| s.trim().to_string()).collect();
    }

    if let Some(disable_rules) = args.disable_rules {
        config.analyzer.disabled_rules = disable_rules
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }

    config.reporter.format = args.format;
    config.reporter.output_path = args.output.map(|p| p.to_string_lossy().to_string());
    config.analyzer.parallel = args.parallel;

    // Parse min severity
    config.reporter.min_severity = match args.min_severity.to_lowercase().as_str() {
        "error" => Severity::Error,
        "warning" => Severity::Warning,
        "info" => Severity::Info,
        _ => {
            eprintln!("Invalid severity level: {}", args.min_severity);
            eprintln!("Using default: warning");
            Severity::Warning
        }
    };

    // Collect all files to analyze
    let mut all_files = Vec::new();
    for path in args.paths {
        collect_orbit_files(&path, &mut all_files)?;
    }

    if all_files.is_empty() {
        eprintln!("No .orbit files found");
        return Ok(());
    }

    // Convert paths to strings
    let file_paths: Vec<&str> = all_files
        .iter()
        .map(|p| p.to_str().unwrap_or_default())
        .collect();

    // Analyze files
    match analyze_files_with_config(&file_paths, config.clone()) {
        Ok(issues) => {
            // Create reporter based on configuration
            let reporter = match config.reporter.format.as_str() {
                "json" => Reporter::new_json(),
                "html" => Reporter::new_html(),
                _ => Reporter::new_text(),
            };

            // Report issues
            if issues.is_empty() {
                println!("No issues found in {} files", file_paths.len());
            } else {
                reporter.report_all_issues(&issues);

                // Exit with error code if there are errors
                if issues.iter().any(|i| i.severity == Severity::Error) {
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error analyzing files: {e}");
            process::exit(1);
        }
    }

    Ok(())
}

/// Execute the validate command
fn validate_command(args: ValidateArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Collect all files to validate
    let mut all_files = Vec::new();
    for path in args.paths {
        collect_orbit_files(&path, &mut all_files)?;
    }

    if all_files.is_empty() {
        eprintln!("No .orbit files found");
        return Ok(());
    }

    println!("Validating {} files...", all_files.len());

    let mut error_count = 0;
    for file in all_files {
        let file_path = file.to_string_lossy();
        match std::fs::read_to_string(&file) {
            Ok(content) => match orlint::parser::parse_orbit_file(&content, &file_path) {
                Ok(_) => {
                    println!("✅ {file_path}: Valid");
                }
                Err(e) => {
                    eprintln!("❌ {file_path}: {e}");
                    error_count += 1;
                }
            },
            Err(e) => {
                eprintln!("Error reading {file_path}: {e}");
                error_count += 1;
            }
        }
    }

    if error_count > 0 {
        eprintln!("{error_count} files had validation errors");
        process::exit(1);
    } else {
        println!("All files are valid");
    }

    Ok(())
}

/// Execute the list-rules command
fn list_rules_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("Available rules in orlint:");

    let rules = [
        (
            "non-empty-template",
            "Template section should not be empty",
            "warning",
        ),
        (
            "public-function",
            "Component should have at least one public function",
            "info",
        ),
        (
            "component-naming",
            "Component names should follow naming conventions (default: PascalCase)",
            "warning",
        ),
        (
            "prop-type-required",
            "All component properties should have type annotations",
            "error",
        ),
        (
            "renderer-compatibility",
            "Check component compatibility with specific renderers",
            "error",
        ),
        (
            "state-variable-usage",
            "Check for proper state variable usage patterns",
            "warning",
        ),
    ];

    for (name, desc, severity) in rules {
        println!("  - {name} ({severity})");
        println!("      {desc}");
    }

    Ok(())
}

/// Collect all .orbit files in a directory or get a single file
fn collect_orbit_files(
    path: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            collect_orbit_files(&path, files)?;
        }
    } else if path.is_file() && path.extension().is_some_and(|ext| ext == "orbit") {
        files.push(path.to_path_buf());
    }

    Ok(())
}
