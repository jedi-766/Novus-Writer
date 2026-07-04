//! Document commands - Tauri command handlers for document operations

use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::database::Database;
use crate::core::domain::document::Document;
use crate::models::error::AppError;

/// Create a new document
#[tauri::command]
pub async fn create_document(
    title: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Document, AppError> {
    let doc = Document::new(title);
    
    // Save to database
    let db_guard = db.lock().await;
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
    .execute(db_guard.pool())
    .await?;
    
    tracing::info!("Created document: {} ({})", doc.title, doc.id);
    Ok(doc)
}

/// Open an existing document
#[tauri::command]
pub async fn open_document(
    id: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Document, AppError> {
    let db_guard = db.lock().await;
    
    let row = sqlx::query_as::<_, (String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, i64, String, bool)>(
        r#"
        SELECT id, title, content, created_at, modified_at, version, tags, pinned
        FROM documents
        WHERE id = ?
        "#,
    )
    .bind(&id)
    .fetch_optional(db_guard.pool())
    .await?;
    
    match row {
        Some((id, title, content, created_at, modified_at, version, tags, pinned)) => {
            let doc = Document {
                id,
                title,
                content,
                created_at,
                modified_at,
                version: version as u32,
                tags: serde_json::from_str(&tags).unwrap_or_default(),
                pinned,
            };
            tracing::info!("Opened document: {} ({})", doc.title, doc.id);
            Ok(doc)
        }
        None => Err(AppError::DocumentNotFound(id)),
    }
}

/// Save a document
#[tauri::command]
pub async fn save_document(
    id: String,
    content: String,
    title: Option<String>,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    // Update document
    if let Some(title) = title {
        sqlx::query(
            r#"
            UPDATE documents
            SET content = ?, title = ?, modified_at = CURRENT_TIMESTAMP, version = version + 1
            WHERE id = ?
            "#,
        )
        .bind(&content)
        .bind(&title)
        .bind(&id)
        .execute(db_guard.pool())
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE documents
            SET content = ?, modified_at = CURRENT_TIMESTAMP, version = version + 1
            WHERE id = ?
            "#,
        )
        .bind(&content)
        .bind(&id)
        .execute(db_guard.pool())
        .await?;
    }
    
    tracing::info!("Saved document: {}", id);
    Ok(())
}

/// Delete a document
#[tauri::command]
pub async fn delete_document(
    id: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    let result = sqlx::query("DELETE FROM documents WHERE id = ?")
        .bind(&id)
        .execute(db_guard.pool())
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::DocumentNotFound(id));
    }
    
    tracing::info!("Deleted document: {}", id);
    Ok(())
}

/// List all documents
#[tauri::command]
pub async fn list_documents(
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<Document>, AppError> {
    let db_guard = db.lock().await;
    
    let rows = sqlx::query_as::<_, (String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, i64, String, bool)>(
        r#"
        SELECT id, title, content, created_at, modified_at, version, tags, pinned
        FROM documents
        ORDER BY pinned DESC, modified_at DESC
        "#,
    )
    .fetch_all(db_guard.pool())
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

/// Rename a document
#[tauri::command]
pub async fn rename_document(
    id: String,
    new_title: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    let result = sqlx::query(
        "UPDATE documents SET title = ?, modified_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(&new_title)
    .bind(&id)
    .execute(db_guard.pool())
    .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::DocumentNotFound(id));
    }
    
    tracing::info!("Renamed document {} to: {}", id, new_title);
    Ok(())
}
