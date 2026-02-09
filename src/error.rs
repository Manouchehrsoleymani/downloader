use std::fmt;

/// Custom error types for the downloader
#[derive(Debug)]
pub enum DownloadError {
    /// HTTP request error
    HttpError(reqwest::Error),
    /// IO error
    IoError(std::io::Error),
    /// Invalid header value
    InvalidHeader(String),
    /// Server doesn't support range requests
    RangeNotSupported,
    /// File size parsing error
    InvalidFileSize(String),
    /// Chunk download error
    ChunkDownloadError { chunk_id: usize, error: String },
    /// File merge error
    MergeError(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::HttpError(e) => write!(f, "HTTP error: {}", e),
            DownloadError::IoError(e) => write!(f, "IO error: {}", e),
            DownloadError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            DownloadError::RangeNotSupported => write!(f, "Server does not support range requests"),
            DownloadError::InvalidFileSize(msg) => write!(f, "Invalid file size: {}", msg),
            DownloadError::ChunkDownloadError { chunk_id, error } => {
                write!(f, "Chunk {} download error: {}", chunk_id, error)
            }
            DownloadError::MergeError(msg) => write!(f, "Merge error: {}", msg),
        }
    }
}

impl std::error::Error for DownloadError {}

impl From<reqwest::Error> for DownloadError {
    fn from(err: reqwest::Error) -> Self {
        DownloadError::HttpError(err)
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(err: std::io::Error) -> Self {
        DownloadError::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, DownloadError>;
