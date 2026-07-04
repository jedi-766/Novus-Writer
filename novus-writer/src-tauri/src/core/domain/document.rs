//! Document domain entity
//! 
//! Represents a document in the system with its metadata and content.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for a document
pub type DocumentId = String;

/// A document entity with metadata and content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique document identifier
    pub id: DocumentId,
    /// Document title
    pub title: String,
    /// Document content (Lexical JSON format)
    pub content: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Version number for optimistic locking
    pub version: u32,
    /// Tags for organization
    pub tags: Vec<String>,
    /// Whether document is pinned
    pub pinned: bool,
}

impl Document {
    /// Create a new document with the given title
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            content: Self::default_content(),
            created_at: now,
            modified_at: now,
            version: 1,
            tags: Vec::new(),
            pinned: false,
        }
    }

    /// Create default empty Lexical editor content
    fn default_content() -> String {
        r#"{
            "root": {
                "children": [
                    {
                        "children": [],
                        "direction": null,
                        "format": "",
                        "indent": 0,
                        "type": "paragraph",
                        "version": 1
                    }
                ],
                "direction": null,
                "format": "",
                "indent": 0,
                "type": "root",
                "version": 1
            }
        }"#.to_string()
    }

    /// Update document content
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.modified_at = Utc::now();
        self.version += 1;
    }

    /// Update document title
    pub fn update_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        self.modified_at = Utc::now();
    }

    /// Add a tag to the document
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        let tag = tag.into();
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Remove a tag from the document
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Toggle pinned status
    pub fn toggle_pinned(&mut self) {
        self.pinned = !self.pinned;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document() {
        let doc = Document::new("Test Document");
        assert_eq!(doc.title, "Test Document");
        assert!(!doc.id.is_empty());
        assert_eq!(doc.version, 1);
        assert!(!doc.pinned);
    }

    #[test]
    fn test_update_content() {
        let mut doc = Document::new("Test");
        let initial_version = doc.version;
        doc.update_content(r#"{"root": {"children": []}}"#.to_string());
        assert_eq!(doc.version, initial_version + 1);
    }

    #[test]
    fn test_add_tag() {
        let mut doc = Document::new("Test");
        doc.add_tag("important");
        assert!(doc.tags.contains(&"important".to_string()));
        
        // Adding duplicate should not create duplicate
        doc.add_tag("important");
        assert_eq!(doc.tags.len(), 1);
    }
}
