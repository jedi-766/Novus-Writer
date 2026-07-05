//! Bookmark commands - Tauri command handlers for bookmark operations

use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::database::Database;
use crate::models::error::AppError;
use crate::models::bookmark::Bookmark;

/// Add a bookmark to a document
#[tauri::command]
pub async fn add_bookmark(
    document_id: String,
    name: String,
    position: i64,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<i64, AppError> {
    let db_guard = db.lock().await;
    
    let result = sqlx::query(
        "INSERT INTO bookmarks (document_id, name, position) VALUES (?, ?, ?)",
    )
    .bind(&document_id)
    .bind(&name)
    .bind(position)
    .execute(db_guard.pool())
    .await?;
    
    let bookmark_id = result.last_insert_rowid();
    tracing::info!("Added bookmark '{}' to document {}", name, document_id);
    
    Ok(bookmark_id)
}

/// Get all bookmarks for a document
#[tauri::command]
pub async fn get_bookmarks(
    document_id: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<Bookmark>, AppError> {
    let db_guard = db.lock().await;
    
    let rows = sqlx::query_as::<_, BookmarkRow>(
        "SELECT id, document_id, name, position, created_at FROM bookmarks WHERE document_id = ? ORDER BY position",
    )
    .bind(&document_id)
    .fetch_all(db_guard.pool())
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

/// Remove a bookmark
#[tauri::command]
pub async fn remove_bookmark(
    bookmark_id: i64,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    let result = sqlx::query("DELETE FROM bookmarks WHERE id = ?")
        .bind(bookmark_id)
        .execute(db_guard.pool())
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::InvalidOperation(format!("Bookmark {} not found", bookmark_id)));
    }
    
    tracing::info!("Removed bookmark {}", bookmark_id);
    
    Ok(())
}

/// Update bookmark position
#[tauri::command]
pub async fn update_bookmark_position(
    bookmark_id: i64,
    new_position: i64,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    let result = sqlx::query("UPDATE bookmarks SET position = ? WHERE id = ?")
        .bind(new_position)
        .bind(bookmark_id)
        .execute(db_guard.pool())
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::InvalidOperation(format!("Bookmark {} not found", bookmark_id)));
    }
    
    tracing::debug!("Updated bookmark {} position to {}", bookmark_id, new_position);
    
    Ok(())
}

/// Rename a bookmark
#[tauri::command]
pub async fn rename_bookmark(
    bookmark_id: i64,
    new_name: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    let result = sqlx::query("UPDATE bookmarks SET name = ? WHERE id = ?")
        .bind(&new_name)
        .bind(bookmark_id)
        .execute(db_guard.pool())
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::InvalidOperation(format!("Bookmark {} not found", bookmark_id)));
    }
    
    tracing::debug!("Renamed bookmark {} to '{}'", bookmark_id, new_name);
    
    Ok(())
}

/// Helper struct for querying bookmarks
#[derive(sqlx::FromRow)]
struct BookmarkRow {
    id: i64,
    document_id: String,
    name: String,
    position: i64,
    created_at: chrono::DateTime<chrono::Utc>,
}
