use std::net::Ipv4Addr;

use anyhow::Result;
use tokio::net::UdpSocket;

use dns_common::{recursive_lookup, BytePacketBuffer, DnsPacket, ResultCode};

// Default to 8.8.8.8:53 which is Google's DNS server
// Alternatively we could use 1.1.1.1:53 which is Cloudflare's DNS server
// Or some user-specific DNS server
static GOOGLE_DNS: &str = "8.8.8.8";

#[tokio::main]
async fn main() -> Result<()> {
    let mut buf = [0; 512];
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    loop {
        let (_size, _src) = socket.recv_from(&mut buf).await?;
        handle_query(&socket).await?;
    }
}

async fn handle_query(socket: &UdpSocket) -> Result<()> {
    let mut buf = BytePacketBuffer::new();
    let (_, src) = socket.recv_from(&mut buf.buffer).await?;
    let mut request = DnsPacket::from_buffer(&mut buf)?;

    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        let dns = GOOGLE_DNS.parse::<Ipv4Addr>()?;
        if let Ok(result) = recursive_lookup(socket, dns, &question.qname, question.qtype).await {
            packet.questions.push(question);
            packet.header.result_code = result.header.result_code;

            for rec in result.answers {
                println!("Answer: {:?}", rec);
                packet.answers.push(rec);
            }

            for rec in result.authorities {
                println!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }

            for rec in result.resources {
                println!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        } else {
            packet.header.result_code = ResultCode::SERVFAIL;
        }
    } else {
        packet.header.result_code = ResultCode::FORMERR;
    }

    let mut res_buffer = BytePacketBuffer::new();
    packet.write(&mut res_buffer)?;

    let len = res_buffer.position();
    let data = res_buffer.get_range(0, len)?;

    socket.send_to(data, src).await?;

    Ok(())
}
