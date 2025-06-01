//! .
//! Final binary format on disk:
//! +----------------+----------------+----------------+----------------+
//! |   MAGIC (u32)  | VERSION (u32)  | METADATA LEN   | METADATA (rmp)|
//! +----------------+----------------+----------------+----------------+
//! |                  INSTRUCTION STREAM (encoded)                    |
//! +------------------------------------------------------------------+
//! MAGIC = 0xB1DF
//! VERSION = 0x1000
//! METADATA LEN = u32
//! METADATA = rmp_serde-encoded PatchMetadata
//! Rest is bidiff instruction stream (uncompressed)
//!

use super::{Error, Result, compress::CompressAlgorithm};
use serde::{Deserialize, Serialize};

/// Algorithms available for generating binary diffs.
///
/// Each algorithm offers different tradeoffs between patch size, generation
/// speed, and application speed.
///
/// # Example
/// ```rust
/// use mmap_bidiff::{diff, DiffAlgorithm, CompressAlgorithm};
///
/// // Use rsync for fast differing of similar files
/// let rsync_patch = diff(
///     b"original",
///     b"modified",
///     DiffAlgorithm::Rsync020,
///     CompressAlgorithm::None
/// )?;
///
/// // Use bidiff for potentially smaller patches
/// let bidiff_patch = diff(
///     b"original",
///     b"modified",
///     DiffAlgorithm::Bidiff1,
///     CompressAlgorithm::Zstd
/// )?;
/// # Ok::<(), mmap_bidiff::Error>(())
/// ```
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum DiffAlgorithm {
    /// Bidirectional diff algorithm version 1.
    /// May produce smaller patches for very different files.
    Bidiff1,
}

impl std::fmt::Display for DiffAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A patch that can transform one file into another.
///
/// Contains all the information needed to verify and apply a patch,
/// including source and target file hashes for integrity validation.
///
/// # Example
/// ```rust
/// use mmap_bidiff::{diff, apply, DiffAlgorithm, CompressAlgorithm};
///
/// let source = b"original content";
/// let target = b"modified content";
///
/// // Generate a patch
/// let patch = diff(
///     source,
///     target,
///     DiffAlgorithm::Bidiff1,
///     CompressAlgorithm::Zstd
/// )?;
///
/// // Verify source hash matches
/// assert_eq!(mmap_bidiff::hash(source), patch.before_hash);
///
/// // Apply patch and verify result
/// let result = apply(source, &patch)?;
/// assert_eq!(mmap_bidiff::hash(&result), patch.after_hash);
/// # Ok::<(), mmap_bidiff::Error>(())
/// ```
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PatchMetadata {
    /// Algorithm used to generate this patch
    pub diff_algorithm: DiffAlgorithm,
    /// Compression method used for the patch data
    pub compress_algorithm: CompressAlgorithm,
    /// MD5 hash of the source file
    pub before_hash: String,
    /// MD5 hash of the target file
    pub after_hash: String,
    // Exact length of resulting file after patch apply
    pub output_size: u64,
}

impl PatchMetadata {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        rmp_serde::to_vec(self).map_err(Error::Serialize)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        rmp_serde::from_slice(data).map_err(Error::Deserialize)
    }
}
