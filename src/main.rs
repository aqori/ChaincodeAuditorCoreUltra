// src/main.rs
/*
 * Main executable for ChaincodeAuditorCoreUltra
 */

use clap::Parser;
use chaincodeauditorcoreultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChaincodeAuditorCoreUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
