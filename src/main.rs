// src/main.rs
/*
 * Main executable for MarketBeacon
 */

use clap::Parser;
use marketbeacon::{Result, run};

#[derive(Parser)]
#[command(version, about = "MarketBeacon - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
