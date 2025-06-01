//! .

use super::Control;

use integer_encoding::VarIntWriter;

pub struct InstructionWriter<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> InstructionWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }

    pub fn bytes_written(&self) -> usize {
        self.offset
    }

    pub fn write_control(&mut self, control: &Control) -> Result<(), std::io::Error> {
        self.offset += (&mut self.buf[self.offset..]).write_varint(control.add.len())?;

        let len = control.add.len();
        self.buf[self.offset..self.offset + len].copy_from_slice(control.add);
        self.offset += len;

        self.offset += (&mut self.buf[self.offset..]).write_varint(control.copy.len())?;

        let len = control.copy.len();
        self.buf[self.offset..self.offset + len].copy_from_slice(control.copy);
        self.offset += len;

        self.offset += (&mut self.buf[self.offset..]).write_varint(control.seek)?;
        Ok(())
    }
}
