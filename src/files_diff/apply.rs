//! .

use super::{Error, Result, hash, read_patch_header};
use crate::{MmapReader, MmapWriter, bipatch};
use std::{fs::File, path::Path};

pub fn apply(before_path: &Path, patch_path: &Path, output_path: &Path) -> Result<()> {
    // Map before file
    let before_file = File::open(before_path)
        .map_err(|e| Error::IoString(format!("Failed to open before file: {e}")))?;
    let before_mmap = MmapReader::from_file(&before_file)?;

    // Map patch file
    let patch_file = File::open(patch_path)
        .map_err(|e| Error::IoString(format!("Failed to open patch file: {e}")))?;
    let patch_mmap = MmapReader::from_file(&patch_file)?;

    // Read patch metadata to determine output size
    let (meta, _) = read_patch_header(patch_mmap.as_slice())?;

    // Create/truncate output file
    let output_file = File::create(output_path)
        .map_err(|e| Error::IoString(format!("Failed to create output file: {e}")))?;
    output_file
        .set_len(meta.output_size)
        .map_err(|e| Error::IoString(format!("Failed to set output file length: {e}")))?;
    let mut output_mmap = MmapWriter::from_file(&output_file)?;

    // Apply patch
    apply_mmap(
        before_mmap.as_slice(),
        patch_mmap.as_slice(),
        output_mmap.as_mut_slice(),
    )
}

fn apply_mmap(base: &[u8], patch: &[u8], output: &mut [u8]) -> Result<()> {
    let (meta, _) = super::read_patch_header(patch)?;

    if hash(base) != meta.before_hash {
        return Err(Error::BeforeHashMismatch);
    }

    bipatch::apply_patch_stream(patch, base, output)?;

    if hash(output) != meta.after_hash {
        return Err(Error::AfterHashMismatch);
    }

    Ok(())
}
