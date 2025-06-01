//! .

mod error;
mod reader;
mod varint;

use crate::files_diff::{Error, Result, hash, read_patch_header};

use reader::{PatchFragment, ZeroCopyReader};
use std::io::{self, ErrorKind};

pub use error::DecodeError;

pub(super) use super::{MAGIC, VERSION};

pub fn apply_patch_stream(patch: &[u8], base: &[u8], output: &mut [u8]) -> Result<()> {
    let (meta, header_len) = read_patch_header(patch)?;
    if hash(base) != meta.before_hash {
        return Err(Error::BeforeHashMismatch);
    }
    if output.len() != meta.output_size as usize {
        return Err(Error::AfterHashMismatch);
    }

    let mut reader = ZeroCopyReader::new(&patch[header_len..], base)?;
    let mut write_pos = 0;

    while let Some(fragment) = reader.next()? {
        match fragment {
            PatchFragment::Add(add_slice) => {
                for (i, &b) in add_slice.iter().enumerate() {
                    output[write_pos + i] = b;
                }
                write_pos += add_slice.len();
            }
            PatchFragment::Copy(copy_slice) => {
                output[write_pos..write_pos + copy_slice.len()].copy_from_slice(copy_slice);
                write_pos += copy_slice.len();
            }
        }
    }

    if write_pos != output.len() {
        return Err(Error::Bipatch(
            io::Error::new(
                ErrorKind::UnexpectedEof,
                format!("Wrote {write_pos} bytes, expected {}", output.len()),
            )
            .into(),
        ));
    }

    if hash(output) != meta.after_hash {
        return Err(Error::AfterHashMismatch);
    }

    Ok(())
}
