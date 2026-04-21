use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditorError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, EditorError>;