//! .

pub type BidiffResult<T> = std::result::Result<T, BidiffError>;

#[derive(thiserror::Error, Debug)]
pub enum BidiffError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
