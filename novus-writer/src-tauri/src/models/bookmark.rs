//! Bookmark model for document navigation

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Bookmark for quick navigation within documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// Unique bookmark ID (auto-increment)
    pub id: i64,
    /// Parent document ID
    pub document_id: String,
    /// Bookmark name/label
    pub name: String,
    /// Position in document (character offset or node reference)
    pub position: i64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Bookmark {
    /// Create a new bookmark
    pub fn new(document_id: String, name: String, position: i64) -> Self {
        Self {
            id: 0, // Will be set by database
            document_id,
            name,
            position,
            created_at: Utc::now(),
        }
    }
}
