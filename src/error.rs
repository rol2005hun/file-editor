use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonitorError {
    #[error("Failed to read system data: {0}")]
    ReadError(String),
    #[error("Invalid data format")]
    InvalidFormat,
}

pub type Result<T> = std::result::Result<T, MonitorError>;