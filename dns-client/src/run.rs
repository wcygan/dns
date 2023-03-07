use anyhow::{Context, Result};
use tokio::net::ToSocketAddrs;

use dns_common::{lookup, socket};

use crate::args::Args;

pub async fn run(args: Args) -> Result<()> {
    let Args {
        server,
        port,
        name,
        qtype,
    } = args;

    let socket = socket(addr()).await?;
    let dns_server = format!("{}:{}", server, port);
    let response = lookup(&socket, dns_server, &name, &qtype)
        .await
        .context("Failed to lookup")?;

    println!("{:#?}", response);
    Ok(())
}

fn addr() -> impl ToSocketAddrs {
    "0.0.0.0:4310".to_string()
}
