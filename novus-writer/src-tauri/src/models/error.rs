//! Error types for the application

use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("File error: {0}")]
    FileError(String),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("Import error: {0}")]
    ImportError(String),
}

/// Result type alias for application errors
pub type Result<T> = std::result::Result<T, AppError>;
