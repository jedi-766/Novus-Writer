//! Document application service
//! 
//! Handles document-related use cases.

use crate::core::domain::document::{Document, DocumentId};
use crate::models::error::AppError;

/// Document application service trait
pub trait DocumentAppService {
    fn create_document(&self, title: &str) -> Result<Document, AppError>;
    fn get_document(&self, id: &DocumentId) -> Result<Document, AppError>;
    fn update_document(&self, doc: Document) -> Result<Document, AppError>;
    fn delete_document(&self, id: &DocumentId) -> Result<(), AppError>;
    fn list_documents(&self) -> Result<Vec<Document>, AppError>;
}
