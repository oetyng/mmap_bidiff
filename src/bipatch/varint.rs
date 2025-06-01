//! .

use std::io::{self, ErrorKind};

/// Trait for reading unsigned LEB128-style varints from a byte slice
pub trait VarIntReader {
    fn read_varint<T: TryFrom<u64>>(&mut self) -> io::Result<T>;
}

impl<'a> VarIntReader for &'a [u8] {
    fn read_varint<T: TryFrom<u64>>(&mut self) -> io::Result<T> {
        let mut result = 0u64;
        let mut shift = 0;
        let mut bytes_read = 0;

        while bytes_read < 10 {
            if self.is_empty() {
                return Err(io::Error::new(
                    ErrorKind::UnexpectedEof,
                    "Unexpected EOF while reading varint",
                ));
            }

            let byte = self[0];
            *self = &self[1..];
            bytes_read += 1;

            result |= ((byte & 0x7F) as u64) << shift;

            if byte & 0x80 == 0 {
                return T::try_from(result).map_err(|_| {
                    io::Error::new(
                        ErrorKind::InvalidData,
                        "Varint overflow or invalid target type",
                    )
                });
            }

            shift += 7;
        }

        Err(io::Error::new(ErrorKind::InvalidData, "Varint too long"))
    }
}

/// Trait for reading signed ZigZag-decoded varints from a byte slice
pub trait VarIntReaderZigZag {
    fn read_varint_i64(&mut self) -> io::Result<i64>;
}

impl<'a> VarIntReaderZigZag for &'a [u8] {
    fn read_varint_i64(&mut self) -> io::Result<i64> {
        let raw: u64 = self.read_varint()?;
        let decoded = ((raw >> 1) as i64) ^ (-((raw & 1) as i64));
        Ok(decoded)
    }
}
