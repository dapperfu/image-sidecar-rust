/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: CLI interface for sportball-sidecar-rust
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: clap, tokio, anyhow
 */

use clap::{Parser, Subcommand};
use sportball_sidecar_rust::{SportballSidecar, SidecarFormat};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "sportball-sidecar-rust")]
#[command(about = "High-performance Rust implementation for sportball JSON sidecar operations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate JSON sidecar files in parallel
    Validate {
        /// Input directory containing sidecar files
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file (use '-' for stdout)
        #[arg(short, long, default_value = "-")]
        output: String,
        
        /// Number of parallel workers
        #[arg(short, long, default_value = "16")]
        workers: usize,
        
        /// Operation type filter
        #[arg(long)]
        operation_type: Option<String>,
    },
    
    /// Get comprehensive statistics about sidecar files
    Stats {
        /// Input directory containing sidecar files
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file (use '-' for stdout)
        #[arg(short, long, default_value = "-")]
        output: String,
        
        /// Operation type filter
        #[arg(long)]
        operation_type: Option<String>,
    },
    
    /// Clean up orphaned sidecar files
    Cleanup {
        /// Input directory containing sidecar files
        #[arg(short, long)]
        input: PathBuf,
        
        /// Dry run - show what would be cleaned without actually cleaning
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Export sidecar data to various formats
    Export {
        /// Input directory containing sidecar files
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file
        #[arg(short, long)]
        output: PathBuf,
        
        /// Operation type filter
        #[arg(long)]
        operation_type: Option<String>,
        
        /// Export format (json, csv)
        #[arg(long, default_value = "json")]
        format: String,
    },
    
    /// Convert sidecar files between formats
    Convert {
        /// Input directory containing sidecar files
        #[arg(short, long)]
        input: PathBuf,
        
        /// Target format (json, bin, rkyv)
        #[arg(short, long)]
        format: String,
        
        /// Dry run - show what would be converted without actually converting
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Show format statistics for sidecar files
    FormatStats {
        /// Input directory containing sidecar files
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file (use '-' for stdout)
        #[arg(short, long, default_value = "-")]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Validate { input, output, workers, operation_type: _ } => {
            let sidecar = SportballSidecar::new(Some(workers));
            let results = sidecar.validate_sidecars(&input).await?;
            
            let output_data = serde_json::json!({
                "total_files": results.len(),
                "valid_files": results.iter().filter(|r| r.is_valid).count(),
                "invalid_files": results.iter().filter(|r| !r.is_valid).count(),
                "results": results
            });
            
            if output == "-" {
                println!("{}", serde_json::to_string_pretty(&output_data)?);
            } else {
                std::fs::write(&output, serde_json::to_string_pretty(&output_data)?)?;
                println!("Validation results written to: {}", output);
            }
        }
        
        Commands::Stats { input, output, operation_type: _ } => {
            let sidecar = SportballSidecar::new(None);
            let stats = sidecar.get_statistics(&input).await?;
            
            if output == "-" {
                println!("{}", serde_json::to_string_pretty(&stats)?);
            } else {
                std::fs::write(&output, serde_json::to_string_pretty(&stats)?)?;
                println!("Statistics written to: {}", output);
            }
        }
        
        Commands::Cleanup { input, dry_run } => {
            let sidecar = SportballSidecar::new(None);
            
            if dry_run {
                println!("Dry run mode - scanning for orphaned sidecar files in: {:?}", input);
                // TODO: Implement dry run functionality
                println!("Dry run not yet implemented");
            } else {
                let removed_count = sidecar.cleanup_orphaned(&input).await?;
                println!("Removed {} orphaned sidecar files", removed_count);
            }
        }
        
        Commands::Export { input, output, operation_type: _, format } => {
            let sidecar = SportballSidecar::new(None);
            let sidecars = sidecar.find_sidecars(&input).await?;
            
            match format.as_str() {
                "json" => {
                    let export_data = serde_json::json!({
                        "exported_at": chrono::Utc::now().to_rfc3339(),
                        "source_directory": input,
                        "total_sidecars": sidecars.len(),
                        "sidecars": sidecars
                    });
                    std::fs::write(&output, serde_json::to_string_pretty(&export_data)?)?;
                }
                "csv" => {
                    // TODO: Implement CSV export
                    println!("CSV export not yet implemented");
                    return Ok(());
                }
                _ => {
                    eprintln!("Unsupported export format: {}", format);
                    return Ok(());
                }
            }
            
            println!("Exported {} sidecar files to: {:?}", sidecars.len(), output);
        }
        
        Commands::Convert { input, format, dry_run } => {
            let sidecar = SportballSidecar::new(None);
            
            // Parse target format
            let target_format = match format.to_lowercase().as_str() {
                "json" => SidecarFormat::Json,
                "bin" | "binary" => SidecarFormat::Binary,
                "rkyv" => SidecarFormat::Rkyv,
                _ => {
                    eprintln!("Unsupported format: {}. Supported formats: json, bin, rkyv", format);
                    return Ok(());
                }
            };
            
            if dry_run {
                println!("Dry run mode - would convert sidecar files in {:?} to {:?}", input, target_format);
                let format_stats = sidecar.get_format_statistics(&input).await?;
                println!("Current format distribution:");
                for (format, count) in format_stats {
                    println!("  {:?}: {} files", format, count);
                }
            } else {
                let converted_count = sidecar.convert_directory_format(&input, target_format).await?;
                println!("Converted {} sidecar files to {:?}", converted_count, target_format);
            }
        }
        
        Commands::FormatStats { input, output } => {
            let sidecar = SportballSidecar::new(None);
            let format_stats = sidecar.get_format_statistics(&input).await?;
            
            let output_data = serde_json::json!({
                "directory": input,
                "format_distribution": format_stats,
                "total_files": format_stats.values().sum::<u32>(),
                "generated_at": chrono::Utc::now().to_rfc3339()
            });
            
            if output == "-" {
                println!("{}", serde_json::to_string_pretty(&output_data)?);
            } else {
                std::fs::write(&output, serde_json::to_string_pretty(&output_data)?)?;
                println!("Format statistics written to: {}", output);
            }
        }
    }
    
    Ok(())
}
