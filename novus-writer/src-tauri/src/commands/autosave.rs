//! Autosave commands - Tauri command handlers for autosave operations

use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::database::Database;
use crate::models::error::AppError;
use crate::models::autosave::Autosave;

/// Save an autosave snapshot
#[tauri::command]
pub async fn save_autosave(
    document_id: String,
    content: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<i64, AppError> {
    let db_guard = db.lock().await;
    
    // Compress content
    let compressed_content = crate::utils::compression::compress(content.as_bytes())?;
    
    // Delete existing autosave for this document
    sqlx::query("DELETE FROM autosaves WHERE document_id = ?")
        .bind(&document_id)
        .execute(db_guard.pool())
        .await?;
    
    // Insert new autosave
    let result = sqlx::query(
        "INSERT INTO autosaves (document_id, content) VALUES (?, ?)",
    )
    .bind(&document_id)
    .bind(&compressed_content)
    .execute(db_guard.pool())
    .await?;
    
    let autosave_id = result.last_insert_rowid();
    tracing::debug!("Saved autosave {} for document {}", autosave_id, document_id);
    
    Ok(autosave_id)
}

/// Get autosave for a document
#[tauri::command]
pub async fn get_autosave(
    document_id: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Option<Autosave>, AppError> {
    let db_guard = db.lock().await;
    
    let row = sqlx::query_as::<_, AutosaveRow>(
        "SELECT id, document_id, content, created_at FROM autosaves WHERE document_id = ?",
    )
    .bind(&document_id)
    .fetch_optional(db_guard.pool())
    .await?;
    
    match row {
        Some(row) => {
            let decompressed = crate::utils::compression::decompress(&row.content)?;
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
#[tauri::command]
pub async fn delete_autosave(
    document_id: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    sqlx::query("DELETE FROM autosaves WHERE document_id = ?")
        .bind(&document_id)
        .execute(db_guard.pool())
        .await?;
    
    tracing::debug!("Deleted autosave for document {}", document_id);
    
    Ok(())
}

/// Recover from autosave (get autosave and optionally delete it)
#[tauri::command]
pub async fn recover_autosave(
    document_id: String,
    delete_after_recovery: Option<bool>,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Option<String>, AppError> {
    let db_guard = db.lock().await;
    
    let row = sqlx::query_as::<_, AutosaveRow>(
        "SELECT id, document_id, content, created_at FROM autosaves WHERE document_id = ?",
    )
    .bind(&document_id)
    .fetch_optional(db_guard.pool())
    .await?;
    
    match row {
        Some(row) => {
            let decompressed = crate::utils::compression::decompress(&row.content)?;
            let content = String::from_utf8(decompressed)
                .map_err(|e| AppError::FileError(e.to_string()))?;
            
            // Optionally delete after recovery
            if delete_after_recovery.unwrap_or(false) {
                drop(db_guard); // Release lock before calling again
                let db_guard2 = db.lock().await;
                sqlx::query("DELETE FROM autosaves WHERE document_id = ?")
                    .bind(&document_id)
                    .execute(db_guard2.pool())
                    .await?;
            }
            
            tracing::info!("Recovered autosave for document {}", document_id);
            
            Ok(Some(content))
        }
        None => Ok(None),
    }
}

/// Cleanup old autosaves (keep only recent ones)
#[tauri::command]
pub async fn cleanup_old_autosaves(
    keep_count: Option<i64>,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<i64, AppError> {
    let db_guard = db.lock().await;
    let keep = keep_count.unwrap_or(5);
    
    // Delete all but the most recent autosaves per document
    let result = sqlx::query(
        r#"
        DELETE FROM autosaves
        WHERE id NOT IN (
            SELECT id FROM (
                SELECT id, document_id,
                       ROW_NUMBER() OVER (PARTITION BY document_id ORDER BY created_at DESC) as rn
                FROM autosaves
            )
            WHERE rn <= ?
        )
        "#,
    )
    .bind(keep)
    .execute(db_guard.pool())
    .await?;
    
    let deleted_count = result.rows_affected() as i64;
    tracing::info!("Cleaned up {} old autosaves", deleted_count);
    
    Ok(deleted_count)
}

/// Helper struct for querying autosaves
#[derive(sqlx::FromRow)]
struct AutosaveRow {
    id: i64,
    document_id: String,
    content: Vec<u8>,
    created_at: chrono::DateTime<chrono::Utc>,
}
