use std::net::Ipv4Addr;

use anyhow::Result;

use crate::{BytePacketBuffer, DnsHeader, DnsQuestion, DnsRecord, QueryType};

#[derive(Debug, Clone)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> Self {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket> {
        let mut result = DnsPacket::new();

        result.header.read(buffer)?;

        for _ in 0..result.header.question_count {
            let mut q = DnsQuestion::new("".to_string(), QueryType::Unknown(0));
            q.read(buffer)?;
            result.questions.push(q);
        }

        for _ in 0..result.header.answer_count {
            let r = DnsRecord::read(buffer)?;
            result.answers.push(r);
        }

        for _ in 0..result.header.authoritative_entry_count {
            let r = DnsRecord::read(buffer)?;
            result.authorities.push(r);
        }

        for _ in 0..result.header.resource_entry_count {
            let r = DnsRecord::read(buffer)?;
            result.resources.push(r);
        }

        Ok(result)
    }

    pub fn write(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        self.header.question_count = self.questions.len() as u16;
        self.header.answer_count = self.answers.len() as u16;
        self.header.authoritative_entry_count = self.authorities.len() as u16;
        self.header.resource_entry_count = self.resources.len() as u16;

        self.header.write(buffer)?;

        for question in &self.questions {
            question.write(buffer)?;
        }
        for rec in &self.answers {
            rec.write(buffer)?;
        }
        for rec in &self.authorities {
            rec.write(buffer)?;
        }
        for rec in &self.resources {
            rec.write(buffer)?;
        }

        Ok(())
    }

    pub fn get_random_a(&self) -> Option<Ipv4Addr> {
        self.answers
            .iter()
            .filter_map(|r| match r {
                DnsRecord::A { address, .. } => Some(*address),
                _ => None,
            })
            .next()
    }

    fn get_ns<'a>(&'a self, qname: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.authorities
            .iter()
            .filter_map(|r| match r {
                DnsRecord::NS {
                    domain,
                    name_server,
                    ..
                } => Some((domain.as_str(), name_server.as_str())),
                _ => None,
            })
            .filter(move |(domain, _)| qname.ends_with(*domain))
    }

    pub fn get_resolved_ns(&self, qname: &str) -> Option<Ipv4Addr> {
        self.get_ns(qname)
            .flat_map(|(_, host)| {
                self.resources.iter().filter_map(move |r| match r {
                    DnsRecord::A {
                        domain, address, ..
                    } if domain == host => Some(address),
                    _ => None,
                })
            })
            .copied()
            .next()
    }

    pub fn get_unresolved_ns<'a>(&'a self, qname: &'a str) -> Option<&'a str> {
        self.get_ns(qname).map(|(_, host)| host).next()
    }
}
