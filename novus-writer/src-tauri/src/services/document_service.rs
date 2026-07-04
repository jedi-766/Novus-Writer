//! Document service - Business logic for document operations

use crate::core::domain::document::{Document, DocumentId};
use crate::models::error::AppError;
use sqlx::SqlitePool;

/// Document service for managing documents
pub struct DocumentService {
    pool: SqlitePool,
}

impl DocumentService {
    /// Create a new document service
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new document
    pub async fn create(&self, title: &str) -> Result<Document, AppError> {
        let doc = Document::new(title);
        
        sqlx::query(
            r#"
            INSERT INTO documents (id, title, content, version, tags, pinned)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&doc.id)
        .bind(&doc.title)
        .bind(&doc.content)
        .bind(doc.version as i64)
        .bind("[]")
        .bind(doc.pinned as i64)
        .execute(&self.pool)
        .await?;
        
        tracing::info!("Created document: {} ({})", doc.title, doc.id);
        Ok(doc)
    }

    /// Get a document by ID
    pub async fn get(&self, id: &DocumentId) -> Result<Document, AppError> {
        let row = sqlx::query_as::<_, (String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, i64, String, bool)>(
            r#"
            SELECT id, title, content, created_at, modified_at, version, tags, pinned
            FROM documents
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some((id, title, content, created_at, modified_at, version, tags, pinned)) => {
                Ok(Document {
                    id,
                    title,
                    content,
                    created_at,
                    modified_at,
                    version: version as u32,
                    tags: serde_json::from_str(&tags).unwrap_or_default(),
                    pinned,
                })
            }
            None => Err(AppError::DocumentNotFound(id.clone())),
        }
    }

    /// Save a document
    pub async fn save(&self, doc: &mut Document) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE documents
            SET content = ?, title = ?, modified_at = CURRENT_TIMESTAMP, version = version + 1
            WHERE id = ?
            "#,
        )
        .bind(&doc.content)
        .bind(&doc.title)
        .bind(&doc.id)
        .execute(&self.pool)
        .await?;
        
        doc.version += 1;
        doc.modified_at = chrono::Utc::now();
        
        tracing::info!("Saved document: {}", doc.id);
        Ok(())
    }

    /// Delete a document
    pub async fn delete(&self, id: &DocumentId) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::DocumentNotFound(id.clone()));
        }
        
        tracing::info!("Deleted document: {}", id);
        Ok(())
    }

    /// List all documents
    pub async fn list(&self) -> Result<Vec<Document>, AppError> {
        let rows = sqlx::query_as::<_, (String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, i64, String, bool)>(
            r#"
            SELECT id, title, content, created_at, modified_at, version, tags, pinned
            FROM documents
            ORDER BY pinned DESC, modified_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        
        let documents = rows
            .into_iter()
            .map(|(id, title, content, created_at, modified_at, version, tags, pinned)| {
                Document {
                    id,
                    title,
                    content,
                    created_at,
                    modified_at,
                    version: version as u32,
                    tags: serde_json::from_str(&tags).unwrap_or_default(),
                    pinned,
                }
            })
            .collect();
        
        Ok(documents)
    }

    /// Get recent documents
    pub async fn get_recent(&self, limit: usize) -> Result<Vec<Document>, AppError> {
        let rows = sqlx::query_as::<_, (String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, i64, String, bool)>(
            r#"
            SELECT id, title, content, created_at, modified_at, version, tags, pinned
            FROM documents
            ORDER BY modified_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;
        
        let documents = rows
            .into_iter()
            .map(|(id, title, content, created_at, modified_at, version, tags, pinned)| {
                Document {
                    id,
                    title,
                    content,
                    created_at,
                    modified_at,
                    version: version as u32,
                    tags: serde_json::from_str(&tags).unwrap_or_default(),
                    pinned,
                }
            })
            .collect();
        
        Ok(documents)
    }
}
