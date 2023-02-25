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

    pub fn position(&self) -> usize {
        self.position
    }

    fn step(&mut self, count: usize) -> Result<()> {
        self.position += count;
        Ok(())
    }

    fn read(&mut self) -> Result<u8> {
        if self.position >= 512 {
            return Err(anyhow!("End of buffer"));
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
            return Err(anyhow!("End of buffer"));
        }

        Ok(self.buffer[position])
    }

    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            return Err(anyhow!("End of buffer"));
        }
        Ok(&self.buffer[start..start + len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let mut result = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(result)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(res)
    }

    /// Read a qname
    ///
    /// The tricky part: Reading domain names, taking labels into consideration.
    /// Will take something like [3]www[6]google[3]com[0] and append
    /// www.google.com to outstr.
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        let mut pos = self.position();
        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;
        let mut delim = "";

        loop {
            if jumps_performed > max_jumps {
                return Err(anyhow!("too many jumps"));
            }

            let len = self.get(pos)?;

            // If len has the two most significant bit are set, it represents a
            // jump to some other offset in the packet:
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }

                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                jumped = true;
                jumps_performed += 1;
                continue;
            } else {
                pos += 1;
                if len == 0 {
                    break;
                }

                outstr.push_str(delim);

                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
                delim = ".";
                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }

    fn write(&mut self, val: u8) -> Result<()> {
        if self.position >= 512 {
            return Err(anyhow!("End of buffer"));
        }

        self.buffer[self.position] = val;
        self.position += 1;
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<()> {
        self.write(val)
    }

    pub fn write_u16(&mut self, val: u16) -> Result<()> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)
    }

    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<()> {
        for label in qname.split('.') {
            let len = label.len();
            if len > 0x3f {
                return Err(anyhow!("Single label exceeds 63 characters of length"));
            }

            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }

        self.write_u8(0)?;

        Ok(())
    }
}
