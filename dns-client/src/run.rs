use anyhow::{Context, Result};
use tokio::net::{ToSocketAddrs, UdpSocket};

use dns_common::{BytePacketBuffer, DnsPacket, DnsQuestion};

use crate::args::Args;

pub async fn run(args: Args) -> Result<()> {
    let socket = local_socket().await?;

    let dns_server = format!("{}:{}", args.server, args.port);

    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.question_count = 1;
    packet.header.recursion_desired = true;

    packet
        .questions
        .push(DnsQuestion::new(args.name, args.qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    socket
        .send_to(&req_buffer.buffer[0..req_buffer.position], dns_server)
        .await
        .context("Failed to send request to DNS server")?;

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
