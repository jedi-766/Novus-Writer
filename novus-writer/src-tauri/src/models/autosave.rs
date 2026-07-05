//! Autosave model for crash recovery

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Autosave entry for automatic backup and crash recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Autosave {
    /// Unique autosave ID (auto-increment)
    pub id: i64,
    /// Parent document ID
    pub document_id: String,
    /// Autosaved content (compressed Lexical JSON)
    pub content: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Autosave {
    /// Create a new autosave entry
    pub fn new(document_id: String, content: String) -> Self {
        Self {
            id: 0, // Will be set by database
            document_id,
            content,
            created_at: Utc::now(),
        }
    }
}
