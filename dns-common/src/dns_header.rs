use crate::{BytePacketBuffer, ResultCode};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,

    pub recursion_desired: bool,
    pub truncated_message: bool,
    pub authoritative_answer: bool,
    pub opcode: u8,
    pub response: bool,

    pub result_code: ResultCode,
    pub checking_disabled: bool,
    pub authenticated_data: bool,
    pub z: bool,
    pub recursion_available: bool,

    pub question_count: u16,
    pub answer_count: u16,
    pub authoritative_entry_count: u16,
    pub resource_entry_count: u16,
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,
            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,
            result_code: ResultCode::NOERROR,
            checking_disabled: false,
            authenticated_data: false,
            z: false,
            recursion_available: false,
            question_count: 0,
            answer_count: 0,
            authoritative_entry_count: 0,
            resource_entry_count: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        self.id = buffer.read_u16()?;
        let flags = buffer.read_u16()?;

        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;

        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.opcode = (a >> 3) & 0x0F;
        self.response = (a & (1 << 7)) > 0;

        self.result_code = ResultCode::from_u8(b & 0x0F);
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.authenticated_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_available = (b & (1 << 7)) > 0;

        self.question_count = buffer.read_u16()?;
        self.answer_count = buffer.read_u16()?;
        self.authoritative_entry_count = buffer.read_u16()?;
        self.resource_entry_count = buffer.read_u16()?;

        Ok(())
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.write_u16(self.id)?;

        buffer.write_u8(
            (self.recursion_desired as u8)
                | ((self.truncated_message as u8) << 1)
                | ((self.authoritative_answer as u8) << 2)
                | (self.opcode << 3)
                | ((self.response as u8) << 7),
        )?;

        buffer.write_u8(
            (self.result_code as u8)
                | ((self.checking_disabled as u8) << 4)
                | ((self.authenticated_data as u8) << 5)
                | ((self.z as u8) << 6)
                | ((self.recursion_available as u8) << 7),
        )?;

        buffer.write_u16(self.question_count)?;
        buffer.write_u16(self.answer_count)?;
        buffer.write_u16(self.authoritative_entry_count)?;
        buffer.write_u16(self.resource_entry_count)?;

        Ok(())
    }
}
