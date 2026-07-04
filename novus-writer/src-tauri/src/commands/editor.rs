//! Editor commands - Tauri command handlers for editor operations

use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::database::Database;
use crate::models::error::AppError;

/// Insert text at current cursor position
#[tauri::command]
pub async fn insert_text(
    _doc_id: String,
    _text: String,
) -> Result<String, AppError> {
    // Text insertion is handled on the frontend by Lexical
    // This command is for special cases requiring backend processing
    tracing::debug!("Insert text command received");
    Ok(_text)
}

/// Format selected text
#[tauri::command]
pub async fn format_text(
    _doc_id: String,
    _format: String,
) -> Result<(), AppError> {
    // Formatting is handled on the frontend by Lexical
    tracing::debug!("Format text command received: {}", _format);
    Ok(())
}

/// Insert an image into the document
#[tauri::command]
pub async fn insert_image(
    _doc_id: String,
    _src: String,
    _alt: String,
) -> Result<String, AppError> {
    // Image insertion is primarily handled on frontend
    // Backend can process and store embedded images here if needed
    tracing::debug!("Insert image command received: {}", _src);
    Ok(_src)
}

/// Insert a table into the document
#[tauri::command]
pub async fn insert_table(
    _doc_id: String,
    _rows: usize,
    _cols: usize,
) -> Result<String, AppError> {
    // Table insertion is handled on frontend by Lexical Table plugin
    tracing::debug!("Insert table command received: {}x{}", _rows, _cols);
    
    // Return basic table structure as JSON
    let table_json = serde_json::json!({
        "type": "table",
        "rows": _rows,
        "cols": _cols
    });
    
    Ok(table_json.to_string())
}
