use anyhow::{Context, Result};
use tokio::net::{ToSocketAddrs, UdpSocket};

use crate::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType};

pub async fn lookup(
    socket: &UdpSocket,
    dns_server: impl ToSocketAddrs,
    name: String,
    qtype: QueryType,
) -> Result<DnsPacket> {
    send_request(socket, dns_server, name, qtype).await?;
    let response = get_response(socket).await?;
    Ok(response)
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
