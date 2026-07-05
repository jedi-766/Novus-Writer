//! Backup commands - Tauri command handlers for backup operations

use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::path::PathBuf;
use crate::database::Database;
use crate::models::error::AppError;
use crate::models::backup::Backup;

/// Create a backup record
#[tauri::command]
pub async fn create_backup(
    document_id: String,
    backup_path: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<i64, AppError> {
    let db_guard = db.lock().await;
    
    // Get file size if file exists
    let size = std::fs::metadata(&backup_path)
        .map(|m| m.len() as i64)
        .ok();
    
    let result = sqlx::query(
        "INSERT INTO backups (document_id, backup_path, backup_size) VALUES (?, ?, ?)",
    )
    .bind(&document_id)
    .bind(&backup_path)
    .bind(size)
    .execute(db_guard.pool())
    .await?;
    
    let backup_id = result.last_insert_rowid();
    tracing::info!("Created backup {} for document {} at {}", backup_id, document_id, backup_path);
    
    Ok(backup_id)
}

/// Get all backups for a document
#[tauri::command]
pub async fn get_backups(
    document_id: String,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<Backup>, AppError> {
    let db_guard = db.lock().await;
    
    let rows = sqlx::query_as::<_, BackupRow>(
        "SELECT id, document_id, backup_path, created_at, backup_size FROM backups WHERE document_id = ? ORDER BY created_at DESC",
    )
    .bind(&document_id)
    .fetch_all(db_guard.pool())
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

/// Delete a backup record (optionally also delete the file)
#[tauri::command]
pub async fn delete_backup(
    backup_id: i64,
    delete_file: Option<bool>,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<(), AppError> {
    let db_guard = db.lock().await;
    
    // Get backup path before deleting
    let backup_path = sqlx::query_scalar::<_, String>(
        "SELECT backup_path FROM backups WHERE id = ?",
    )
    .bind(backup_id)
    .fetch_optional(db_guard.pool())
    .await?;
    
    let result = sqlx::query("DELETE FROM backups WHERE id = ?")
        .bind(backup_id)
        .execute(db_guard.pool())
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::InvalidOperation(format!("Backup {} not found", backup_id)));
    }
    
    // Optionally delete the actual file
    if delete_file.unwrap_or(false) {
        if let Some(path) = backup_path {
            if let Err(e) = std::fs::remove_file(&path) {
                tracing::warn!("Failed to delete backup file {}: {}", path, e);
            } else {
                tracing::debug!("Deleted backup file: {}", path);
            }
        }
    }
    
    tracing::info!("Deleted backup {}", backup_id);
    
    Ok(())
}

/// Restore from a backup
#[tauri::command]
pub async fn restore_from_backup(
    backup_id: i64,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<String, AppError> {
    let db_guard = db.lock().await;
    
    let backup_path = sqlx::query_scalar::<_, String>(
        "SELECT backup_path FROM backups WHERE id = ?",
    )
    .bind(backup_id)
    .fetch_one(db_guard.pool())
    .await?;
    
    // Read backup file content
    let content = std::fs::read_to_string(&backup_path)
        .map_err(|e| AppError::FileError(format!("Failed to read backup file: {}", e)))?;
    
    tracing::info!("Restored from backup {}", backup_id);
    
    Ok(content)
}

/// Cleanup old backups (keep only recent ones per document)
#[tauri::command]
pub async fn cleanup_old_backups(
    keep_count: Option<i64>,
    delete_files: Option<bool>,
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<i64, AppError> {
    let db_guard = db.lock().await;
    let keep = keep_count.unwrap_or(10);
    let should_delete_files = delete_files.unwrap_or(false);
    
    // Get backups to delete
    let backups_to_delete = sqlx::query_as::<_, BackupRow>(
        r#"
        SELECT id, document_id, backup_path, created_at, backup_size
        FROM backups
        WHERE id NOT IN (
            SELECT id FROM (
                SELECT id, document_id,
                       ROW_NUMBER() OVER (PARTITION BY document_id ORDER BY created_at DESC) as rn
                FROM backups
            )
            WHERE rn <= ?
        )
        "#,
    )
    .bind(keep)
    .fetch_all(db_guard.pool())
    .await?;
    
    // Optionally delete files first
    if should_delete_files {
        for backup in &backups_to_delete {
            if let Err(e) = std::fs::remove_file(&backup.backup_path) {
                tracing::warn!("Failed to delete backup file {}: {}", backup.backup_path, e);
            }
        }
    }
    
    // Get IDs for deletion
    let ids: Vec<i64> = backups_to_delete.iter().map(|b| b.id).collect();
    
    if ids.is_empty() {
        return Ok(0);
    }
    
    // Build dynamic query for IN clause
    let placeholders = vec!["?"; ids.len()].join(",");
    let query = format!("DELETE FROM backups WHERE id IN ({})", placeholders);
    
    let mut sql_query = sqlx::query(&query);
    for id in &ids {
        sql_query = sql_query.bind(id);
    }
    
    let result = sql_query.execute(db_guard.pool()).await?;
    
    let deleted_count = result.rows_affected() as i64;
    tracing::info!("Cleaned up {} old backups", deleted_count);
    
    Ok(deleted_count)
}

/// Get backup statistics
#[tauri::command]
pub async fn get_backup_stats(
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<BackupStats, AppError> {
    let db_guard = db.lock().await;
    
    let (total_count, total_size): (i64, i64) = sqlx::query_as::<_, (i64, i64)>(
        "SELECT COUNT(*), COALESCE(SUM(backup_size), 0) FROM backups",
    )
    .fetch_one(db_guard.pool())
    .await?;
    
    let oldest_backup = sqlx::query_scalar::<_, Option<chrono::DateTime<chrono::Utc>>>(
        "SELECT MIN(created_at) FROM backups",
    )
    .fetch_one(db_guard.pool())
    .await?;
    
    let newest_backup = sqlx::query_scalar::<_, Option<chrono::DateTime<chrono::Utc>>>(
        "SELECT MAX(created_at) FROM backups",
    )
    .fetch_one(db_guard.pool())
    .await?;
    
    Ok(BackupStats {
        total_count: total_count as u32,
        total_size_bytes: total_size as u64,
        oldest_backup,
        newest_backup,
    })
}

/// Backup statistics
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BackupStats {
    pub total_count: u32,
    pub total_size_bytes: u64,
    pub oldest_backup: Option<chrono::DateTime<chrono::Utc>>,
    pub newest_backup: Option<chrono::DateTime<chrono::Utc>>,
}

/// Helper struct for querying backups
#[derive(sqlx::FromRow)]
struct BackupRow {
    id: i64,
    document_id: String,
    backup_path: String,
    created_at: chrono::DateTime<chrono::Utc>,
    #[allow(dead_code)]
    backup_size: Option<i64>,
}
