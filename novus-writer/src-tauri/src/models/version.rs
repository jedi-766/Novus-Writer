//! Document Version model for version history tracking

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Document version for tracking changes over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    /// Unique version ID (auto-increment)
    pub id: i64,
    /// Parent document ID
    pub document_id: String,
    /// Version content (compressed Lexical JSON)
    pub content: String,
    /// Version number
    pub version_number: u32,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Optional change summary/description
    pub change_summary: Option<String>,
}

impl DocumentVersion {
    /// Create a new document version
    pub fn new(
        document_id: String,
        content: String,
        version_number: u32,
        change_summary: Option<String>,
    ) -> Self {
        Self {
            id: 0, // Will be set by database
            document_id,
            content,
            version_number,
            created_at: Utc::now(),
            change_summary,
        }
    }
}
