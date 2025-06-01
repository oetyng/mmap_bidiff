//! .

use crate::bipatch::varint::{VarIntReader, VarIntReaderZigZag};
use std::io;

/// A zero-copy reader that interprets a bidiff patch stream and yields output fragments.
pub struct ZeroCopyReader<'a> {
    patch: &'a [u8],
    base: &'a [u8],
    patch_pos: usize,
    base_pos: usize,
    state: ReaderState,
}

pub enum PatchFragment<'a> {
    Add(&'a [u8]),
    Copy(&'a [u8]),
}

#[derive(Debug)]
enum ReaderState {
    Seek,
    Add {
        len: usize,
        copy_len: usize,
        seek_val: i64,
    },
    Copy {
        len: usize,
        seek_val: i64,
    },
    Done,
}

impl<'a> ZeroCopyReader<'a> {
    pub fn new(patch: &'a [u8], base: &'a [u8]) -> io::Result<Self> {
        Ok(Self {
            patch,
            base,
            patch_pos: 0,
            base_pos: 0,
            state: ReaderState::Seek,
        })
    }

    pub fn next(&mut self) -> io::Result<Option<PatchFragment<'a>>> {
        loop {
            match &mut self.state {
                ReaderState::Seek => {
                    let slice = &mut &self.patch[self.patch_pos..];

                    let add_len: usize = slice.read_varint()?;
                    let copy_len: usize = slice.read_varint()?;
                    let seek_val: i64 = slice.read_varint_i64()?;

                    self.patch_pos = self.patch.len() - slice.len();

                    self.state = ReaderState::Add {
                        len: add_len,
                        copy_len,
                        seek_val,
                    };
                    continue;
                }
                ReaderState::Add {
                    len,
                    copy_len,
                    seek_val,
                } => {
                    let start = self.base_pos;
                    let end = start + *len;
                    let out_slice = &self.base[start..end];
                    self.base_pos = end;
                    self.state = ReaderState::Copy {
                        len: *copy_len,
                        seek_val: *seek_val,
                    };
                    return Ok(Some(PatchFragment::Add(out_slice)));
                }
                ReaderState::Copy { len, seek_val } => {
                    let start = self.patch_pos;
                    let end = start + *len;
                    let out_slice = &self.patch[start..end];
                    self.patch_pos = end;
                    self.base_pos = (self.base_pos as i64 + *seek_val) as usize;
                    self.state = ReaderState::Seek;
                    return Ok(Some(PatchFragment::Copy(out_slice)));
                }
                ReaderState::Done => return Ok(None),
            }
        }
    }
}
