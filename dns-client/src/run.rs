use anyhow::{Context, Result};
use tokio::net::{ToSocketAddrs, UdpSocket};

use dns_common::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType};

use crate::args::Args;

pub async fn run(args: Args) -> Result<()> {
    let Args {
        server,
        port,
        name,
        qtype,
    } = args;

    // Send the request
    let socket = local_socket().await?;
    let dns_server = format!("{}:{}", server, port);
    send_request(&socket, dns_server, name, qtype).await?;

    // Get the response
    let response = get_response(&socket).await?;
    println!("{:#?}", response);
    Ok(())
}

async fn send_request(
    socket: &UdpSocket,
    dns_server: impl ToSocketAddrs,
    name: String,
    qtype: QueryType,
) -> Result<()> {
    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.question_count = 1;
    packet.header.recursion_desired = true;
    packet.questions.push(DnsQuestion::new(name, qtype));
    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    socket
        .send_to(&req_buffer.buffer[0..req_buffer.position], dns_server)
        .await
        .context("Failed to send request to DNS server")?;

    Ok(())
}

async fn get_response(socket: &UdpSocket) -> Result<DnsPacket> {
    let mut res_buffer = BytePacketBuffer::new();

    socket
        .recv_from(&mut res_buffer.buffer)
        .await
        .context("Failed to receive response from DNS server")?;

    let response = DnsPacket::from_buffer(&mut res_buffer)?;
    Ok(response)
}

async fn local_socket() -> Result<UdpSocket> {
    UdpSocket::bind(arbitrary_addr())
        .await
        .context("Failed to bind to local socket")
}

fn arbitrary_addr() -> impl ToSocketAddrs {
    "0.0.0.0:4310".to_string()
}
