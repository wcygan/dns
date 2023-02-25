use anyhow::Result;
use dns_common::{BytePacketBuffer, DnsPacket};
use std::fs::File;
use std::io::Read;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    let mut f = File::open("response_packet.txt")?;
    let mut buf = BytePacketBuffer::new();
    f.read(&mut buf.buffer)?;

    let packet = DnsPacket::from_buffer(&mut buf)?;
    println!("{:#?}", packet);

    for q in packet.questions {
        println!("{:#?}", q);
    }

    for a in packet.answers {
        println!("{:#?}", a);
    }

    for a in packet.authorities {
        println!("{:#?}", a);
    }

    for r in packet.resources {
        println!("{:#?}", r);
    }

    Ok(())
}
