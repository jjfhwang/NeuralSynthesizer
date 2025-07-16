// src/main.rs
/*
 * Main executable for NeuralSynthesizer
 */

use clap::Parser;
use neuralsynthesizer::{Result, run};

#[derive(Parser)]
#[command(version, about = "NeuralSynthesizer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
