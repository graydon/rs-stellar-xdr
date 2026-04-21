//! CLI entry point for the XDR code generator.

mod generator;
mod naming;
mod options;
mod output;
mod types;

#[cfg(test)]
mod tests;

use clap::Parser;
use generator::RustGenerator;
use options::RustOptions;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// XDR code generator.
#[derive(Parser, Debug)]
#[command(name = "xdr-generator")]
#[command(about = "Generate code from XDR definitions")]
struct Args {
    /// Input XDR files
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,

    /// Output file
    #[arg(short, long)]
    output: PathBuf,

    /// Types with custom Default implementation (skip derive(Default))
    #[arg(long, value_delimiter = ',')]
    custom_default: Vec<String>,

    /// Types with custom FromStr/Display implementation (use SerializeDisplay)
    #[arg(long, value_delimiter = ',')]
    custom_str: Vec<String>,

    /// Types that should NOT have Display/FromStr/schemars generated
    #[arg(long, value_delimiter = ',')]
    no_display_fromstr: Vec<String>,

    /// Output file for a standalone CXX bridge module. Requires --bridge-types.
    #[arg(long)]
    cxx_bridge_output: Option<PathBuf>,

    /// Comma-separated list of XDR type names to include in the CXX bridge.
    #[arg(long, value_delimiter = ',')]
    bridge_types: Vec<String>,
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

    let generator = RustGenerator::new(&spec, options);
    generator.generate_to_file(&spec, &args.output)?;
    eprintln!("Generated: {}", args.output.display());

    // Generate standalone CXX bridge module if requested
    if let Some(cxx_output) = &args.cxx_bridge_output {
        if args.bridge_types.is_empty() {
            return Err("--cxx-bridge-output requires --bridge-types".into());
        }
        let bridge_types: HashSet<String> = args.bridge_types.into_iter().collect();
        generator.generate_cxx_bridge_to_file(&spec, cxx_output, &bridge_types)?;
        eprintln!("Generated CXX bridge: {}", cxx_output.display());
    }

    Ok(())
}
