//! Export commands - Tauri command handlers for document export operations

use std::path::PathBuf;

use crate::models::error::AppError;
use crate::services::export_service::{ExportFormat, ExportService};
use crate::database::Database;

/// Export document to PDF
#[tauri::command]
pub async fn export_pdf(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Exporting document {} to PDF: {}", doc_id, output_path);
    
    // Get document from database
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    let path = PathBuf::from(&output_path);
    
    // Use export service
    match ExportService::export_to_pdf(&document, &path) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(AppError::ExportError(e)),
    }
}

/// Export document to DOCX
#[tauri::command]
pub async fn export_docx(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Exporting document {} to DOCX: {}", doc_id, output_path);
    
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    let path = PathBuf::from(&output_path);
    
    match ExportService::export_to_docx(&document, &path) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(AppError::ExportError(e)),
    }
}

/// Export document to Markdown
#[tauri::command]
pub async fn export_markdown(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Exporting document {} to Markdown: {}", doc_id, output_path);
    
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    let path = PathBuf::from(&output_path);
    
    match ExportService::export_to_markdown(&document, &path) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(AppError::ExportError(e)),
    }
}

/// Export document to HTML
#[tauri::command]
pub async fn export_html(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Exporting document {} to HTML: {}", doc_id, output_path);
    
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    let path = PathBuf::from(&output_path);
    
    match ExportService::export_to_html(&document, &path) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(AppError::ExportError(e)),
    }
}

/// Export document to RTF
#[tauri::command]
pub async fn export_rtf(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Exporting document {} to RTF: {}", doc_id, output_path);
    
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    let path = PathBuf::from(&output_path);
    
    match ExportService::export_to_rtf(&document, &path) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(AppError::ExportError(e)),
    }
}

/// Export document to plain text
#[tauri::command]
pub async fn export_txt(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Exporting document {} to TXT: {}", doc_id, output_path);
    
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    let path = PathBuf::from(&output_path);
    
    match ExportService::export_to_plain_text(&document, &path) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(AppError::ExportError(e)),
    }
}

/// Export document to ODT
#[tauri::command]
pub async fn export_odt(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Exporting document {} to ODT: {}", doc_id, output_path);
    
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    let path = PathBuf::from(&output_path);
    
    match ExportService::export_to_odt(&document, &path) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(AppError::ExportError(e)),
    }
}

/// Generic export command that auto-detects format from file extension
#[tauri::command]
pub async fn export_document(
    db: tauri::State<'_, Database>,
    doc_id: String,
    output_path: String,
) -> Result<String, AppError> {
    tracing::info!("Auto-exporting document {} to: {}", doc_id, output_path);
    
    let document = db.get_document(&doc_id)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Document {} not found", doc_id)))?;
    
    // Detect format from extension
    let path = PathBuf::from(&output_path);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("txt");
    
    let format = ExportFormat::from_extension(extension)
        .unwrap_or(ExportFormat::PlainText);
    
    match ExportService::export(&document, format, path) {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => Err(AppError::ExportError(e)),
    }
}
