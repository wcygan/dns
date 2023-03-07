use std::net::Ipv4Addr;

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use tokio::net::{ToSocketAddrs, UdpSocket};

use crate::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType, ResultCode};

pub async fn socket(addr: impl ToSocketAddrs) -> Result<UdpSocket> {
    UdpSocket::bind(addr)
        .await
        .context("Failed to bind to local socket")
}

#[async_recursion]
pub async fn recursive_lookup(
    socket: &UdpSocket,
    dns_server: Ipv4Addr,
    qname: &str,
    qtype: QueryType,
) -> Result<DnsPacket> {
    let mut ns = dns_server;

    loop {
        println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);

        // The next step is to send the query to the active server.
        let ns_copy = ns;

        let server = (ns_copy, 53);
        let response = lookup(socket, server, &qname.to_string(), &qtype).await?;

        // If there are entries in the answer section, and no errors, we are done!
        if !response.answers.is_empty() && response.header.result_code == ResultCode::NOERROR {
            return Ok(response);
        }

        if response.header.result_code == ResultCode::NXDOMAIN {
            return Ok(response);
        }

        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;
            continue;
        }

        let new_ns_name = match response.get_unresolved_ns(qname) {
            Some(name) => name,
            None => {
                return Ok(response);
            }
        };

        let recursive_response = recursive_lookup(socket, ns, new_ns_name, QueryType::A).await?;

        if let Some(new_ns) = recursive_response.get_random_a() {
            ns = new_ns;
        } else {
            return Ok(response);
        }
    }
}

pub async fn lookup(
    socket: &UdpSocket,
    dns_server: impl ToSocketAddrs,
    name: &String,
    qtype: &QueryType,
) -> Result<DnsPacket> {
    send_request(socket, dns_server, name, qtype).await?;
    get_response(socket).await
}

async fn send_request(
    socket: &UdpSocket,
    dns_server: impl ToSocketAddrs,
    name: &String,
    qtype: &QueryType,
) -> Result<()> {
    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.question_count = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(name.clone(), *qtype));
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

    DnsPacket::from_buffer(&mut res_buffer)
}
