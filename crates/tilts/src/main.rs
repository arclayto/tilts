//! `tilts` — factor-exposure analysis for real portfolios.
//!
//! ```text
//! tilts analyze holdings.csv --model ff5mom --region us --since 2015-01 --rolling 36
//! tilts factors fetch --model ff5 --region us
//! tilts cache path | clear
//! ```
//!
//! This binary is a thin shell over `tilts-core` (math) and `tilts-data` (I/O):
//! argument parsing, orchestration, and terminal rendering only.

mod cli;
mod render;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    cli::run(args)
}
