use crate::{BytePacketBuffer, QueryType};
use anyhow::Result;
use std::net::Ipv4Addr;

pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    },
    A {
        domain: String,
        address: Ipv4Addr,
        ttl: u32,
    },
}

impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;
        let qtype_num = buffer.read_u16()?;
        let qtype = QueryType::from_u16(qtype_num);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let address = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    (raw_addr >> 0 & 0xFF) as u8,
                );
                Ok(DnsRecord::A {
                    domain,
                    address,
                    ttl,
                })
            }
            _ => Ok(DnsRecord::UNKNOWN {
                domain,
                qtype: qtype_num,
                data_len,
                ttl,
            }),
        }
    }
}
