//! Repository module - Data access layer
//! 
//! Provides repository implementations for data persistence.

mod document_repository;

pub use document_repository::SqliteDocumentRepository;

use crate::core::domain::document::{Document, DocumentId};
use crate::models::error::AppError;

/// Document repository trait defining the interface for document data access
pub trait DocumentRepository {
    fn save(&self, doc: &Document) -> Result<(), AppError>;
    fn find_by_id(&self, id: &DocumentId) -> Result<Option<Document>, AppError>;
    fn find_all(&self) -> Result<Vec<Document>, AppError>;
    fn delete(&self, id: &DocumentId) -> Result<(), AppError>;
    fn find_recent(&self, limit: usize) -> Result<Vec<Document>, AppError>;
}
