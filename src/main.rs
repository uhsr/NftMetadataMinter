// src/main.rs
/*
 * Main executable for NftMetadataMinter
 */

use clap::Parser;
use nftmetadataminter::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMetadataMinter - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
