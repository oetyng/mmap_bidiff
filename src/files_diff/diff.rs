//! .

use std::path::Path;

use super::{
    CompressAlgorithm, DiffAlgorithm, Error, PatchMetadata, Result, hash,
    header::write_patch_header,
};

use crate::{
    MmapReader, MmapWriter,
    bidiff::{self, DiffParams},
};

// 512KiB. Choosing a chunk size that's too large will result in suboptimal core
// utilization, whereas choosing a chunk size that's too small will result in
// increased memory usage for diminishing returns
const SCAN_CHUNK_SIZE: usize = 1024 * 512;

pub fn diff(
    before: &Path,
    after: &Path,
    patch_path: &Path,
    compress_algorithm: CompressAlgorithm,
) -> Result<PatchMetadata> {
    use std::fs::OpenOptions;

    let before_mmap = MmapReader::open(before)?;
    let after_mmap = MmapReader::open(after)?;

    // Estimate worst-case size (after.len() + overhead)
    let upper_bound = after_mmap.len() + 512;

    // Create and preallocate patch file
    let patch_file = OpenOptions::new()
        .read(true) // REQUIRED for mmap on Windows
        .write(true)
        .create(true)
        .truncate(true)
        .open(patch_path)
        .map_err(|e| Error::IoString(format!("failed to create patch file: {e}")))?;

    patch_file
        .set_len(upper_bound as u64)
        .map_err(|e| Error::IoString(format!("failed to set patch file len: {e}")))?;

    // Memory-map it for writing
    let mut patch_mmap = MmapWriter::from_file(&patch_file)
        .map_err(|e| Error::IoString(format!("failed to mmap patch file: {e}")))?;

    // Perform diff
    let meta = diff_mmap(
        &before_mmap,
        &after_mmap,
        &mut patch_mmap,
        compress_algorithm,
    )?;

    // Truncate file to final patch size
    // ensure flush before truncating
    drop(patch_mmap);
    patch_file
        .set_len(meta.output_size)
        .map_err(|e| Error::IoString(format!("failed to truncate patch file: {e}")))?;

    Ok(meta)
}

fn diff_mmap(
    before: &MmapReader,
    after: &MmapReader,
    output: &mut MmapWriter,
    compress_algorithm: CompressAlgorithm,
) -> Result<PatchMetadata> {
    let diff_params = DiffParams::new(num_cpus::get() - 1, Some(SCAN_CHUNK_SIZE))
        .map_err(|e| Error::Bidiff(format!("failed to create diff params: {e}")))?;

    let metadata = PatchMetadata {
        compress_algorithm,
        diff_algorithm: DiffAlgorithm::Bidiff1,
        before_hash: hash(before.as_slice()),
        after_hash: hash(after.as_slice()),
        output_size: after.len() as u64,
    };

    let header_size = write_patch_header(&mut output.as_mut_slice(), &metadata)?;
    if header_size > output.len() {
        return Err(Error::IoString("Output buffer too small for header".into()));
    }

    let written = bidiff::simple_diff_with_params(
        before.as_slice(),
        after.as_slice(),
        &mut output.as_mut_slice()[header_size..],
        &diff_params,
    )?;

    let expected_total = header_size + written;
    if expected_total > output.len() {
        return Err(Error::IoString(format!(
            "Output patch overflow: wrote {expected_total}, buffer size {}",
            output.len()
        )));
    }

    Ok(metadata)
}
