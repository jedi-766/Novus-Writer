//! Version commands - Tauri command handlers for document version management

use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::database::Database;
use crate::models::error::AppError;
use crate::models::version::DocumentVersion;

/// Create a new document version
#[tauri::command]
pub async fn create_version(
    document_id: String,
    content: String,
    change_summary: Option<String>,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<i64, AppError> {
    let db_guard = db.lock().await;
    
    // Get current document to determine version number
    let row = sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(MAX(version), 0) + 1 FROM documents WHERE id = ?"
    )
    .bind(&document_id)
    .fetch_one(db_guard.pool())
    .await?;
    
    let version_number = row as u32;
    
    // Compress content
    let compressed_content = crate::utils::compression::compress(content.as_bytes())?;
    
    // Insert version
    let result = sqlx::query(
        r#"
        INSERT INTO document_versions (document_id, version_number, content, change_summary)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&document_id)
    .bind(version_number as i64)
    .bind(&compressed_content)
    .bind(&change_summary)
    .execute(db_guard.pool())
    .await?;
    
    let version_id = result.last_insert_rowid();
    tracing::info!("Created version {} for document {}", version_id, document_id);
    
    Ok(version_id)
}

/// Get version history for a document
#[tauri::command]
pub async fn get_version_history(
    document_id: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<DocumentVersion>, AppError> {
    let db_guard = db.lock().await;
    
    let rows = sqlx::query_as::<_, VersionRow>(
        r#"
        SELECT id, document_id, version_number, content, created_at, change_summary
        FROM document_versions
        WHERE document_id = ?
        ORDER BY version_number DESC
        "#,
    )
    .bind(&document_id)
    .fetch_all(db_guard.pool())
    .await?;
    
    let mut versions = Vec::with_capacity(rows.len());
    for row in rows {
        let decompressed = crate::utils::compression::decompress(&row.content)?;
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
#[tauri::command]
pub async fn restore_version(
    version_id: i64,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(String, String), AppError> {
    let db_guard = db.lock().await;
    
    let row = sqlx::query_as::<_, VersionRow>(
        "SELECT id, document_id, version_number, content, created_at, change_summary FROM document_versions WHERE id = ?",
    )
    .bind(version_id)
    .fetch_one(db_guard.pool())
    .await?;
    
    let decompressed = crate::utils::compression::decompress(&row.content)?;
    let content = String::from_utf8(decompressed)
        .map_err(|e| AppError::FileError(e.to_string()))?;
    
    // Update document with restored content
    sqlx::query(
        r#"
        UPDATE documents 
        SET content = ?, modified_at = CURRENT_TIMESTAMP, version = version + 1
        WHERE id = ?
        "#,
    )
    .bind(&content)
    .bind(&row.document_id)
    .execute(db_guard.pool())
    .await?;
    
    tracing::info!("Restored version {} for document {}", version_id, row.document_id);
    
    Ok((row.document_id, content))
}

/// Delete a specific version
#[tauri::command]
pub async fn delete_version(
    version_id: i64,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    let result = sqlx::query("DELETE FROM document_versions WHERE id = ?")
        .bind(version_id)
        .execute(db_guard.pool())
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::InvalidOperation(format!("Version {} not found", version_id)));
    }
    
    tracing::info!("Deleted version {}", version_id);
    
    Ok(())
}

/// Helper struct for querying versions
#[derive(sqlx::FromRow)]
struct VersionRow {
    id: i64,
    document_id: String,
    version_number: i64,
    content: Vec<u8>,
    created_at: chrono::DateTime<chrono::Utc>,
    change_summary: Option<String>,
}
