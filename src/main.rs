// Main entry point for the orbit-analyzer CLI

use clap::{Args, Parser, Subcommand};
use orbit_analyzer::{analyze_file, Reporter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "orbit-analyzer")]
#[command(about = "Static analysis tool for Orbit UI framework files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint .orbit files for issues
    Lint(LintArgs),
}

#[derive(Args)]
struct LintArgs {
    /// Files to analyze
    #[arg(required = true)]
    files: Vec<PathBuf>,
    
    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Lint(args) => {
            let reporter = match args.format.as_str() {
                "json" => Reporter::new_json(),
                _ => Reporter::new_text(),
            };
            
            for file in args.files {
                let file_path = file.to_string_lossy();
                match analyze_file(&file_path) {
                    Ok(issues) => {
                        reporter.report_issues(&file_path, &issues);
                    }
                    Err(e) => {
                        eprintln!("Error analyzing {}: {}", file_path, e);
                    }
                }
            }
        }
    }
    
    Ok(())
}
