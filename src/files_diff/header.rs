//! .

use super::{Error, PatchMetadata, Result};
use crate::{MAGIC, VERSION, bipatch::DecodeError};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn write_patch_header(out: &mut &mut [u8], meta: &PatchMetadata) -> Result<usize> {
    let meta_bytes = meta.serialize()?;
    let mut offset = 0;

    (&mut out[offset..]).write_u32::<LittleEndian>(MAGIC)?;
    offset += 4;
    (&mut out[offset..]).write_u32::<LittleEndian>(VERSION)?;
    offset += 4;
    (&mut out[offset..]).write_u32::<LittleEndian>(meta_bytes.len() as u32)?;
    offset += 4;

    (&mut out[offset..offset + meta_bytes.len()]).copy_from_slice(&meta_bytes);
    offset += meta_bytes.len();

    Ok(offset)
}

pub fn read_patch_header<'a>(data: &'a [u8]) -> Result<(PatchMetadata, usize)> {
    let mut offset = 0;
    let magic = (&data[offset..]).read_u32::<LittleEndian>()?;
    offset += 4;
    if magic != MAGIC {
        return Err(Error::Bipatch(DecodeError::WrongMagic(magic)));
    }

    let version = (&data[offset..]).read_u32::<LittleEndian>()?;
    offset += 4;
    if version != VERSION {
        return Err(Error::Bipatch(DecodeError::WrongVersion(version)));
    }

    let meta_len = (&data[offset..]).read_u32::<LittleEndian>()? as usize;
    offset += 4;

    let meta = PatchMetadata::deserialize(&data[offset..offset + meta_len])?;
    offset += meta_len;

    Ok((meta, offset))
}
