use anyhow::{anyhow, Result};

pub struct BytePacketBuffer {
    pub buffer: [u8; 512],
    pub position: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buffer: [0; 512],
            position: 0,
        }
    }

    fn position(&self) -> usize {
        self.position
    }

    fn step(&mut self, count: usize) -> Result<()> {
        self.position += count;
        Ok(())
    }

    fn read(&mut self) -> Result<u8> {
        if self.position >= 512 {
            return Err(anyhow!("out of bounds"));
        }

        let result = self.buffer[self.position];
        self.position += 1;

        Ok(result)
    }

    fn seek(&mut self, position: usize) -> Result<()> {
        self.position = position;
        Ok(())
    }

    fn get(&self, position: usize) -> Result<u8> {
        if position >= 512 {
            return Err(anyhow!("out of bounds"));
        }

        Ok(self.buffer[position])
    }

    fn get_range(&self, start: usize, end: usize) -> Result<&[u8]> {
        if end > 512 {
            return Err(anyhow!("out of bounds"));
        }

        Ok(&self.buffer[start..end])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut result = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(result)
    }
}
