use crate::{BytePacketBuffer, QueryType};
use anyhow::Result;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
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

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<usize> {
        let start_pos = buffer.position();

        match *self {
            DnsRecord::A {
                ref domain,
                ref address,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::A.to_u16())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                buffer.write_u16(4)?;

                let octets = address.octets();
                buffer.write_u8(octets[0])?;
                buffer.write_u8(octets[1])?;
                buffer.write_u8(octets[2])?;
                buffer.write_u8(octets[3])?;
            }
            DnsRecord::UNKNOWN { .. } => {
                println!("Skipping record: {:?}", self);
            }
        }

        Ok(buffer.position() - start_pos)
    }
}
