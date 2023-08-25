use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Blockprint Collective", version, long_about = None)]
#[command(about = "Measure the accuracy of a blockprint instance using synthetic blocks")]
struct Cli {
    /// Lighthouse node to fetch block reward data from.
    #[arg(long, value_name = "FILE")]
    lighthouse_url: Option<PathBuf>,

    /// Blockprint instance to use for classifying blocks.
    #[arg(, long, action = clap::ArgAction::Count)]
    debug: u8,
}
