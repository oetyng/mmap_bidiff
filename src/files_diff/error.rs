//! .

pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during differing and patching operations.
///
/// This enum represents all possible errors that can occur when generating or
/// applying patches, including algorithm-specific errors, hash validation
/// failures, and I/O operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("An error occurred in the bidiff algorithm: {0}")]
    Bidiff(String),
    #[error("An error occurred in the bipatch decoding: {0}")]
    Bipatch(#[from] crate::bipatch::DecodeError),
    #[error("The hash of the source file doesn't match the expected hash")]
    BeforeHashMismatch,
    #[error("The hash of the generated file doesn't match the expected hash")]
    AfterHashMismatch,
    #[error("The hash of the operations doesn't match the expected hash")]
    OperationsHashMismatch,
    #[error("An I/O error occurred during file operations: {0}")]
    IoString(String),
    #[error("An error occurred while processing a zip archive: {0}")]
    Zstd(String),
    #[error("An error occurred while serializing a patch or patch set: {0}")]
    Serialize(#[from] rmp_serde::encode::Error),
    #[error("An error occurred while deserializing a patch or patch set: {0}")]
    Deserialize(#[from] rmp_serde::decode::Error),
}
