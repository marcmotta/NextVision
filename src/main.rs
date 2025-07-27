// src/main.rs
/*
 * Main executable for NextVision
 */

use clap::Parser;
use nextvision::{Result, run};

#[derive(Parser)]
#[command(version, about = "NextVision - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
