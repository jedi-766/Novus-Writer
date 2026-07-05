//! Document Repository - Data access layer for document operations
//! 
//! Provides concrete implementation of the DocumentRepository trait
//! using SQLx and SQLite.

use sqlx::{SqlitePool, FromRow};
use crate::core::domain::document::{Document, DocumentId};
use crate::models::error::AppError;
use crate::models::version::DocumentVersion;
use crate::models::autosave::Autosave;
use crate::models::bookmark::Bookmark;
use crate::models::backup::Backup;
use crate::utils::compression;
use chrono::{DateTime, Utc};

/// Concrete document repository implementation using SQLite
pub struct SqliteDocumentRepository {
    pool: SqlitePool,
}

impl SqliteDocumentRepository {
    /// Create a new repository instance with a connection pool
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Save or update a document
    pub async fn save(&self, doc: &Document) -> Result<(), AppError> {
        let compressed_content = compression::compress(doc.content.as_bytes())?;
        
        sqlx::query(
            r#"
            INSERT INTO documents (id, title, content, created_at, modified_at, version, tags, pinned, word_count, character_count, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                content = excluded.content,
                modified_at = CURRENT_TIMESTAMP,
                version = excluded.version,
                tags = excluded.tags,
                pinned = excluded.pinned,
                word_count = excluded.word_count,
                character_count = excluded.character_count,
                metadata = excluded.metadata
            "#,
        )
        .bind(&doc.id)
        .bind(&doc.title)
        .bind(&compressed_content)
        .bind(doc.created_at)
        .bind(doc.modified_at)
        .bind(doc.version as i64)
        .bind(serde_json::to_string(&doc.tags)?)
        .bind(doc.pinned as i64)
        .bind(0i64) // word_count - TODO: calculate
        .bind(0i64) // character_count - TODO: calculate
        .bind("{}") // metadata
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Find a document by ID
    pub async fn find_by_id(&self, id: &DocumentId) -> Result<Option<Document>, AppError> {
        let row = sqlx::query_as::<_, DocumentRow>(
            r#"
            SELECT 
                id, title, content, created_at, modified_at, version, tags, pinned,
                word_count, character_count, last_opened_at, metadata
            FROM documents
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let content = if let Some(compressed) = row.content {
                    let decompressed = compression::decompress(&compressed)?;
                    String::from_utf8(decompressed).map_err(|e| AppError::FileError(e.to_string()))?
                } else {
                    String::new()
                };

                Ok(Some(Document {
                    id: row.id,
                    title: row.title,
                    content,
                    created_at: row.created_at,
                    modified_at: row.modified_at,
                    version: row.version as u32,
                    tags: serde_json::from_str(&row.tags).unwrap_or_default(),
                    pinned: row.pinned != 0,
                }))
            }
            None => Ok(None),
        }
    }

    /// Find all documents
    pub async fn find_all(&self) -> Result<Vec<Document>, AppError> {
        let rows = sqlx::query_as::<_, DocumentRow>(
            r#"
            SELECT 
                id, title, content, created_at, modified_at, version, tags, pinned,
                word_count, character_count, last_opened_at, metadata
            FROM documents
            ORDER BY pinned DESC, modified_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut documents = Vec::with_capacity(rows.len());
        for row in rows {
            let content = if let Some(compressed) = row.content {
                let decompressed = compression::decompress(&compressed)?;
                String::from_utf8(decompressed).map_err(|e| AppError::FileError(e.to_string()))?
            } else {
                String::new()
            };

            documents.push(Document {
                id: row.id,
                title: row.title,
                content,
                created_at: row.created_at,
                modified_at: row.modified_at,
                version: row.version as u32,
                tags: serde_json::from_str(&row.tags).unwrap_or_default(),
                pinned: row.pinned != 0,
            });
        }

        Ok(documents)
    }

    /// Delete a document by ID
    pub async fn delete(&self, id: &DocumentId) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::DocumentNotFound(id.clone()));
        }

        Ok(())
    }

    /// Find recent documents with limit
    pub async fn find_recent(&self, limit: usize) -> Result<Vec<Document>, AppError> {
        let rows = sqlx::query_as::<_, DocumentRow>(
            r#"
            SELECT 
                id, title, content, created_at, modified_at, version, tags, pinned,
                word_count, character_count, last_opened_at, metadata
            FROM documents
            ORDER BY 
                CASE WHEN last_opened_at IS NOT NULL THEN last_opened_at ELSE modified_at END DESC
            LIMIT ?
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut documents = Vec::with_capacity(rows.len());
        for row in rows {
            let content = if let Some(compressed) = row.content {
                let decompressed = compression::decompress(&compressed)?;
                String::from_utf8(decompressed).map_err(|e| AppError::FileError(e.to_string()))?
            } else {
                String::new()
            };

            documents.push(Document {
                id: row.id,
                title: row.title,
                content,
                created_at: row.created_at,
                modified_at: row.modified_at,
                version: row.version as u32,
                tags: serde_json::from_str(&row.tags).unwrap_or_default(),
                pinned: row.pinned != 0,
            });
        }

        Ok(documents)
    }

    /// Create a new document version
    pub async fn create_version(
        &self,
        document_id: &str,
        content: &str,
        version_number: u32,
        change_summary: Option<&str>,
    ) -> Result<i64, AppError> {
        let compressed_content = compression::compress(content.as_bytes())?;

        let result = sqlx::query(
            r#"
            INSERT INTO document_versions (document_id, version_number, content, change_summary)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(document_id)
        .bind(version_number as i64)
        .bind(&compressed_content)
        .bind(change_summary)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get version history for a document
    pub async fn get_versions(&self, document_id: &str) -> Result<Vec<DocumentVersion>, AppError> {
        let rows = sqlx::query_as::<_, VersionRow>(
            r#"
            SELECT id, document_id, version_number, content, created_at, change_summary
            FROM document_versions
            WHERE document_id = ?
            ORDER BY version_number DESC
            "#,
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await?;

        let mut versions = Vec::with_capacity(rows.len());
        for row in rows {
            let decompressed = compression::decompress(&row.content)?;
            let content = String::from_utf8(decompressed)
                .map_err(|e| AppError::FileError(e.to_string()))?;

            versions.push(DocumentVersion {
                id: row.id,
                document_id: row.document_id,
                content,
                version_number: row.version_number as u32,
                created_at: row.created_at,
                change_summary: row.change_summary,
            });
        }

        Ok(versions)
    }

    /// Restore a specific version
    pub async fn restore_version(
        &self,
        version_id: i64,
    ) -> Result<(String, String), AppError> {
        let row = sqlx::query_as::<_, VersionRow>(
            "SELECT id, document_id, version_number, content, created_at, change_summary FROM document_versions WHERE id = ?",
        )
        .bind(version_id)
        .fetch_one(&self.pool)
        .await?;

        let decompressed = compression::decompress(&row.content)?;
        let content = String::from_utf8(decompressed)
            .map_err(|e| AppError::FileError(e.to_string()))?;

        Ok((row.document_id, content))
    }

    /// Create or update autosave
    pub async fn save_autosave(&self, document_id: &str, content: &str) -> Result<i64, AppError> {
        let compressed_content = compression::compress(content.as_bytes())?;

        // Delete existing autosave for this document
        sqlx::query("DELETE FROM autosaves WHERE document_id = ?")
            .bind(document_id)
            .execute(&self.pool)
            .await?;

        // Insert new autosave
        let result = sqlx::query(
            "INSERT INTO autosaves (document_id, content) VALUES (?, ?)",
        )
        .bind(document_id)
        .bind(&compressed_content)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get autosave for a document
    pub async fn get_autosave(&self, document_id: &str) -> Result<Option<Autosave>, AppError> {
        let row = sqlx::query_as::<_, AutosaveRow>(
            "SELECT id, document_id, content, created_at FROM autosaves WHERE document_id = ?",
        )
        .bind(document_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let decompressed = compression::decompress(&row.content)?;
                let content = String::from_utf8(decompressed)
                    .map_err(|e| AppError::FileError(e.to_string()))?;

                Ok(Some(Autosave {
                    id: row.id,
                    document_id: row.document_id,
                    content,
                    created_at: row.created_at,
                }))
            }
            None => Ok(None),
        }
    }

    /// Delete autosave for a document
    pub async fn delete_autosave(&self, document_id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM autosaves WHERE document_id = ?")
            .bind(document_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Add bookmark
    pub async fn add_bookmark(
        &self,
        document_id: &str,
        name: &str,
        position: i64,
    ) -> Result<i64, AppError> {
        let result = sqlx::query(
            "INSERT INTO bookmarks (document_id, name, position) VALUES (?, ?, ?)",
        )
        .bind(document_id)
        .bind(name)
        .bind(position)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get bookmarks for a document
    pub async fn get_bookmarks(&self, document_id: &str) -> Result<Vec<Bookmark>, AppError> {
        let rows = sqlx::query_as::<_, BookmarkRow>(
            "SELECT id, document_id, name, position, created_at FROM bookmarks WHERE document_id = ? ORDER BY position",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await?;

        let bookmarks = rows
            .into_iter()
            .map(|row| Bookmark {
                id: row.id,
                document_id: row.document_id,
                name: row.name,
                position: row.position,
                created_at: row.created_at,
            })
            .collect();

        Ok(bookmarks)
    }

    /// Remove bookmark
    pub async fn remove_bookmark(&self, bookmark_id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM bookmarks WHERE id = ?")
            .bind(bookmark_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Create backup record
    pub async fn create_backup(
        &self,
        document_id: &str,
        backup_path: &str,
        size: Option<i64>,
    ) -> Result<i64, AppError> {
        let result = sqlx::query(
            "INSERT INTO backups (document_id, backup_path, backup_size) VALUES (?, ?, ?)",
        )
        .bind(document_id)
        .bind(backup_path)
        .bind(size)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get backups for a document
    pub async fn get_backups(&self, document_id: &str) -> Result<Vec<Backup>, AppError> {
        let rows = sqlx::query_as::<_, BackupRow>(
            "SELECT id, document_id, backup_path, created_at, backup_size FROM backups WHERE document_id = ? ORDER BY created_at DESC",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await?;

        let backups = rows
            .into_iter()
            .map(|row| Backup {
                id: row.id,
                document_id: row.document_id,
                backup_path: row.backup_path,
                created_at: row.created_at,
            })
            .collect();

        Ok(backups)
    }

    /// Search documents by title or content
    pub async fn search(&self, query: &str) -> Result<Vec<Document>, AppError> {
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query_as::<_, DocumentRow>(
            r#"
            SELECT 
                id, title, content, created_at, modified_at, version, tags, pinned,
                word_count, character_count, last_opened_at, metadata
            FROM documents
            WHERE title LIKE ? OR tags LIKE ?
            ORDER BY modified_at DESC
            "#,
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        let mut documents = Vec::with_capacity(rows.len());
        for row in rows {
            let content = if let Some(compressed) = row.content {
                let decompressed = compression::decompress(&compressed)?;
                String::from_utf8(decompressed).map_err(|e| AppError::FileError(e.to_string()))?
            } else {
                String::new()
            };

            documents.push(Document {
                id: row.id,
                title: row.title,
                content,
                created_at: row.created_at,
                modified_at: row.modified_at,
                version: row.version as u32,
                tags: serde_json::from_str(&row.tags).unwrap_or_default(),
                pinned: row.pinned != 0,
            });
        }

        Ok(documents)
    }
}

/// Helper struct for querying documents
#[derive(FromRow)]
struct DocumentRow {
    id: String,
    title: String,
    content: Option<Vec<u8>>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
    version: i64,
    tags: String,
    pinned: i64,
    #[allow(dead_code)]
    word_count: i64,
    #[allow(dead_code)]
    character_count: i64,
    #[allow(dead_code)]
    last_opened_at: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    metadata: String,
}

/// Helper struct for querying versions
#[derive(FromRow)]
struct VersionRow {
    id: i64,
    document_id: String,
    version_number: i64,
    content: Vec<u8>,
    created_at: DateTime<Utc>,
    change_summary: Option<String>,
}

/// Helper struct for querying autosaves
#[derive(FromRow)]
struct AutosaveRow {
    id: i64,
    document_id: String,
    content: Vec<u8>,
    created_at: DateTime<Utc>,
}

/// Helper struct for querying bookmarks
#[derive(FromRow)]
struct BookmarkRow {
    id: i64,
    document_id: String,
    name: String,
    position: i64,
    created_at: DateTime<Utc>,
}

/// Helper struct for querying backups
#[derive(FromRow)]
struct BackupRow {
    id: i64,
    document_id: String,
    backup_path: String,
    created_at: DateTime<Utc>,
    #[allow(dead_code)]
    backup_size: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[tokio::test]
    async fn test_save_and_find_document() {
        let db = Database::new();
        db.init().await.unwrap();
        let repo = SqliteDocumentRepository::new(db.pool().clone());

        let doc = Document::new("Test Document");
        repo.save(&doc).await.unwrap();

        let found = repo.find_by_id(&doc.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Test Document");
    }

    #[tokio::test]
    async fn test_delete_document() {
        let db = Database::new();
        db.init().await.unwrap();
        let repo = SqliteDocumentRepository::new(db.pool().clone());

        let doc = Document::new("Test Document");
        repo.save(&doc).await.unwrap();
        repo.delete(&doc.id).await.unwrap();

        let found = repo.find_by_id(&doc.id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_create_version() {
        let db = Database::new();
        db.init().await.unwrap();
        let repo = SqliteDocumentRepository::new(db.pool().clone());

        let doc = Document::new("Test Document");
        repo.save(&doc).await.unwrap();

        let version_id = repo.create_version(&doc.id, &doc.content, 1, Some("Initial version"))
            .await
            .unwrap();

        assert!(version_id > 0);

        let versions = repo.get_versions(&doc.id).await.unwrap();
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].version_number, 1);
    }

    #[tokio::test]
    async fn test_autosave() {
        let db = Database::new();
        db.init().await.unwrap();
        let repo = SqliteDocumentRepository::new(db.pool().clone());

        let doc = Document::new("Test Document");
        repo.save(&doc).await.unwrap();

        repo.save_autosave(&doc.id, "Autosaved content").await.unwrap();

        let autosave = repo.get_autosave(&doc.id).await.unwrap();
        assert!(autosave.is_some());
        assert_eq!(autosave.unwrap().content, "Autosaved content");
    }

    #[tokio::test]
    async fn test_bookmarks() {
        let db = Database::new();
        db.init().await.unwrap();
        let repo = SqliteDocumentRepository::new(db.pool().clone());

        let doc = Document::new("Test Document");
        repo.save(&doc).await.unwrap();

        repo.add_bookmark(&doc.id, "Chapter 1", 100).await.unwrap();
        repo.add_bookmark(&doc.id, "Chapter 2", 500).await.unwrap();

        let bookmarks = repo.get_bookmarks(&doc.id).await.unwrap();
        assert_eq!(bookmarks.len(), 2);
        assert_eq!(bookmarks[0].name, "Chapter 1");
        assert_eq!(bookmarks[1].name, "Chapter 2");

        repo.remove_bookmark(bookmarks[0].id).await.unwrap();
        let remaining = repo.get_bookmarks(&doc.id).await.unwrap();
        assert_eq!(remaining.len(), 1);
    }
}
