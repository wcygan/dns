use anyhow::{Context, Result};
use tokio::net::{ToSocketAddrs, UdpSocket};

use dns_common::{lookup, BytePacketBuffer, DnsPacket, DnsQuestion, QueryType};

use crate::args::Args;

pub async fn run(args: Args) -> Result<()> {
    let Args {
        server,
        port,
        name,
        qtype,
    } = args;

    let socket = local_socket().await?;
    let dns_server = format!("{}:{}", server, port);
    let response = lookup(&socket, dns_server, name, qtype)
        .await
        .context("Failed to lookup")?;

    println!("{:#?}", response);
    Ok(())
}

async fn local_socket() -> Result<UdpSocket> {
    UdpSocket::bind(arbitrary_addr())
        .await
        .context("Failed to bind to local socket")
}

fn arbitrary_addr() -> impl ToSocketAddrs {
    "0.0.0.0:4310".to_string()
}
