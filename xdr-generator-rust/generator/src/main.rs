//! CLI entry point for the XDR code generator.

mod generator;
mod lazy_codegen;
mod naming;
mod options;
mod output;
mod types;

#[cfg(test)]
mod tests;

use clap::{Parser, ValueEnum};
use generator::RustGenerator;
use options::RustOptions;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// XDR code generator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputMode {
    Rust,
    CxxBridge,
    Both,
}

#[derive(Parser, Debug)]
#[command(name = "xdr-generator")]
#[command(about = "Generate code from XDR definitions")]
struct Args {
    /// Input XDR files
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,

    /// Generation mode
    #[arg(long, value_enum, default_value_t = OutputMode::Rust)]
    mode: OutputMode,

    /// Output file
    #[arg(short, long)]
    output: PathBuf,

    /// Output file for the cxx bridge module when mode is `both`
    #[arg(long)]
    bridge_output: Option<PathBuf>,

    /// Comma-separated list of type names to expose in cxx bridge output
    #[arg(long, value_delimiter = ',')]
    bridge_types: Vec<String>,

    /// Types with custom Default implementation (skip derive(Default))
    #[arg(long, value_delimiter = ',')]
    custom_default: Vec<String>,

    /// Types with custom FromStr/Display implementation (use SerializeDisplay)
    #[arg(long, value_delimiter = ',')]
    custom_str: Vec<String>,

    /// Types that should NOT have Display/FromStr/schemars generated
    #[arg(long, value_delimiter = ',')]
    no_display_fromstr: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read all input files and sort by filename
    let mut files: Vec<(PathBuf, String)> = Vec::new();
    for path in &args.input {
        let content = fs::read_to_string(path)?;
        files.push((path.clone(), content));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    // Build file list for the parser
    let file_refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(path, content)| (path.to_str().unwrap_or(""), content.as_str()))
        .collect();

    // Parse the XDR files
    let spec = xdr_parser::parser::parse_files(&file_refs)?;

    let options = RustOptions {
        custom_default_impl: args.custom_default.into_iter().collect::<HashSet<_>>(),
        custom_str_impl: args.custom_str.into_iter().collect::<HashSet<_>>(),
        no_display_fromstr: args.no_display_fromstr.into_iter().collect::<HashSet<_>>(),
    };
    let bridge_types = (!args.bridge_types.is_empty())
        .then(|| args.bridge_types.into_iter().collect::<HashSet<_>>());

    let generator = RustGenerator::new(&spec, options);
    match args.mode {
        OutputMode::Rust => {
            generator.generate_to_file(&spec, &args.output)?;
            eprintln!("Generated: {}", args.output.display());
        }
        OutputMode::CxxBridge => {
            generator.generate_cxx_bridge_to_file(&spec, &args.output, bridge_types.as_ref())?;
            eprintln!("Generated: {}", args.output.display());
        }
        OutputMode::Both => {
            let bridge_output = args.bridge_output.as_ref().ok_or(
                "--bridge-output is required when --mode both",
            )?;
            generator.generate_to_file(&spec, &args.output)?;
            generator.generate_cxx_bridge_to_file(&spec, bridge_output, bridge_types.as_ref())?;
            eprintln!("Generated: {}", args.output.display());
            eprintln!("Generated: {}", bridge_output.display());
        }
    }

    Ok(())
}
