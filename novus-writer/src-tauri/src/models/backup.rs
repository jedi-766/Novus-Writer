//! Backup model for automatic backup tracking

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Backup entry for tracking automatic backups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    /// Unique backup ID (auto-increment)
    pub id: i64,
    /// Parent document ID
    pub document_id: String,
    /// Path to the backup file
    pub backup_path: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Backup {
    /// Create a new backup entry
    pub fn new(document_id: String, backup_path: String) -> Self {
        Self {
            id: 0, // Will be set by database
            document_id,
            backup_path,
            created_at: Utc::now(),
        }
    }
}
