use crate::{BytePacketBuffer, QueryType};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    pub fn new(qname: String, qtype: QueryType) -> Self {
        DnsQuestion { qname, qtype }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.read_qname(&mut self.qname)?;
        self.qtype = QueryType::from_u16(buffer.read_u16()?);
        buffer.read_u16()?;
        Ok(())
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.write_qname(&self.qname)?;
        buffer.write_u16(self.qtype.to_u16())?;
        buffer.write_u16(1)
    }
}
