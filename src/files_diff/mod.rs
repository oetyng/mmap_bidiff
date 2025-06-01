//! .

mod apply;
mod compress;
mod diff;
mod error;
mod header;
mod patch;

pub(super) use header::read_patch_header;

pub use apply::apply;
pub use compress::CompressAlgorithm;
pub use diff::diff;
pub use error::{Error, Result};
pub use patch::{DiffAlgorithm, PatchMetadata};

pub fn hash(data: &[u8]) -> String {
    let hash = md5::compute(data);
    hex::encode(hash.0)
}

#[cfg(test)]
mod tests {
    use super::{CompressAlgorithm, apply, diff};
    use std::io::{Seek, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_diff() {
        let before_data = b"hello world";
        let after_data = b"hello darkness my old friend";

        // Create and write before-file
        let mut before_file = NamedTempFile::new().expect("create before tempfile");
        before_file.write_all(before_data).expect("write before");
        before_file.flush().unwrap();
        before_file.rewind().unwrap();

        // Create and write after-file
        let mut after_file = NamedTempFile::new().expect("create after tempfile");
        after_file.write_all(after_data).expect("write after");
        after_file.flush().unwrap();
        after_file.rewind().unwrap();

        // Create and close patch file (Windows-compatible)
        let patch_file = NamedTempFile::new().expect("create patch tempfile");
        let patch_path = patch_file.path().to_path_buf();
        drop(patch_file); // Fully close and release the handle

        // Create patch from before/after
        let _meta = diff(
            before_file.path(),
            after_file.path(),
            &patch_path,
            CompressAlgorithm::Zstd,
        )
        .expect("diff failed");

        // Create output file and apply patch
        let output_file = NamedTempFile::new().expect("create output tempfile");
        apply(before_file.path(), &patch_path, output_file.path()).expect("apply failed");

        // Verify result
        let output_bytes = std::fs::read(output_file.path()).expect("read output");
        assert_eq!(&output_bytes[..after_data.len()], after_data);
    }
}
