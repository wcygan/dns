use anyhow::Result;
use clap::Parser;

use crate::args::Args;
use crate::run::run;

mod args;
mod run;

#[tokio::main]
async fn main() -> Result<()> {
    run(Args::parse()).await
}
