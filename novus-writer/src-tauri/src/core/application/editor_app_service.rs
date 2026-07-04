//! Editor application service
//! 
//! Handles editor-related use cases.

use crate::core::domain::document::DocumentId;
use crate::models::error::AppError;

/// Editor operation commands
#[derive(Debug, Clone)]
pub enum EditorCommand {
    InsertText { text: String },
    DeleteText { count: usize },
    FormatText { format: String },
    InsertImage { src: String, alt: String },
    InsertTable { rows: usize, cols: usize },
    InsertLink { url: String, text: String },
    Undo,
    Redo,
}

/// Editor application service trait
pub trait EditorAppService {
    fn execute_command(
        &self,
        doc_id: &DocumentId,
        command: EditorCommand,
    ) -> Result<String, AppError>;

    fn get_content(&self, doc_id: &DocumentId) -> Result<String, AppError>;

    fn set_content(&self, doc_id: &DocumentId, content: String) -> Result<(), AppError>;
}
