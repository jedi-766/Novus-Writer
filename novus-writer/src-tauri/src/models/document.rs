//! Document model for DTOs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Document DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDto {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub version: u32,
    pub tags: Vec<String>,
    pub pinned: bool,
}

impl From<crate::core::domain::document::Document> for DocumentDto {
    fn from(doc: crate::core::domain::document::Document) -> Self {
        Self {
            id: doc.id,
            title: doc.title,
            created_at: doc.created_at,
            modified_at: doc.modified_at,
            version: doc.version,
            tags: doc.tags,
            pinned: doc.pinned,
        }
    }
}
