use anyhow::Result;
use clap::Parser;

use crate::args::Args;

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    Ok(())
}
