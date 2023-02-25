use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::Result;

use crate::{BytePacketBuffer, QueryType};

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
    NS {
        domain: String,
        name_server: String,
        ttl: u32,
    },
    CNAME {
        domain: String,
        alias: String,
        ttl: u32,
    },
    MX {
        domain: String,
        preference: u16,
        host: String,
        ttl: u32,
    },
    AAAA {
        domain: String,
        address: Ipv6Addr,
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
            QueryType::NS => {
                let mut name_server = String::new();
                buffer.read_qname(&mut name_server)?;
                Ok(DnsRecord::NS {
                    domain,
                    name_server,
                    ttl,
                })
            }
            QueryType::CNAME => {
                let mut alias = String::new();
                buffer.read_qname(&mut alias)?;
                Ok(DnsRecord::CNAME { domain, alias, ttl })
            }
            QueryType::MX => {
                let preference = buffer.read_u16()?;
                let mut host = String::new();
                buffer.read_qname(&mut host)?;
                Ok(DnsRecord::MX {
                    domain,
                    preference,
                    host,
                    ttl,
                })
            }
            QueryType::AAAA => {
                let raw_addr1 = buffer.read_u32()?;
                let raw_addr2 = buffer.read_u32()?;
                let raw_addr3 = buffer.read_u32()?;
                let raw_addr4 = buffer.read_u32()?;

                let address = Ipv6Addr::new(
                    ((raw_addr1 >> 16) & 0xFFFF) as u16,
                    ((raw_addr1 >> 0) & 0xFFFF) as u16,
                    ((raw_addr2 >> 16) & 0xFFFF) as u16,
                    ((raw_addr2 >> 0) & 0xFFFF) as u16,
                    ((raw_addr3 >> 16) & 0xFFFF) as u16,
                    ((raw_addr3 >> 0) & 0xFFFF) as u16,
                    ((raw_addr4 >> 16) & 0xFFFF) as u16,
                    ((raw_addr4 >> 0) & 0xFFFF) as u16,
                );

                Ok(DnsRecord::AAAA {
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
            DnsRecord::NS {
                ref domain,
                ref name_server,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::NS.to_u16())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                let pos = buffer.position();
                buffer.write_u16(0)?;
                buffer.write_qname(name_server)?;
                let len = buffer.position() - pos - 2;
                buffer.set_u16(pos, len as u16)?;
            }
            DnsRecord::CNAME {
                ref domain,
                ref alias,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::CNAME.to_u16())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                let pos = buffer.position();
                buffer.write_u16(0)?;
                buffer.write_qname(alias)?;
                let len = buffer.position() - pos - 2;
                buffer.set_u16(pos, len as u16)?;
            }
            DnsRecord::MX {
                ref domain,
                preference,
                ref host,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::MX.to_u16())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                let pos = buffer.position();
                buffer.write_u16(0)?;
                buffer.write_u16(preference)?;
                buffer.write_qname(host)?;
                let len = buffer.position() - pos - 2;
                buffer.set_u16(pos, len as u16)?;
            }
            DnsRecord::AAAA {
                ref domain,
                ref address,
                ttl,
            } => {
                buffer.write_qname(domain)?;
                buffer.write_u16(QueryType::AAAA.to_u16())?;
                buffer.write_u16(1)?;
                buffer.write_u32(ttl)?;
                buffer.write_u16(16)?;
                for segment in address.segments() {
                    buffer.write_u16(segment)?;
                }
            }
            DnsRecord::UNKNOWN { .. } => {
                println!("Skipping record: {:?}", self);
            }
        }

        Ok(buffer.position() - start_pos)
    }
}
