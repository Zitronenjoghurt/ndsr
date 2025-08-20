use thiserror::Error;

pub type NDSRResult<T> = Result<T, NDSRError>;

#[derive(Debug, Error)]
pub enum NDSRError {
    #[error("Binary read/write error: {0}")]
    BinaryRW(#[from] binrw::Error),
}
