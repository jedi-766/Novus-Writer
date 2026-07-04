//! File utilities

use std::path::{Path, PathBuf};
use crate::models::error::AppError;

/// Get the application data directory
pub fn get_app_data_dir() -> Result<PathBuf, AppError> {
    let app_name = "novus-writer";
    
    #[cfg(target_os = "linux")]
    {
        if let Some(config_dir) = dirs::config_dir() {
            return Ok(config_dir.join(app_name));
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Some(config_dir) = dirs::home_dir() {
            return Ok(config_dir.join("Library/Application Support").join(app_name));
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        if let Some(config_dir) = dirs::data_local_dir() {
            return Ok(config_dir.join(app_name));
        }
    }
    
    Err(AppError::FileError("Could not determine app data directory".to_string()))
}

/// Get the documents directory
pub fn get_documents_dir() -> Result<PathBuf, AppError> {
    dirs::document_dir()
        .ok_or_else(|| AppError::FileError("Could not determine documents directory".to_string()))
}

/// Get the database path
pub fn get_database_path() -> Result<PathBuf, AppError> {
    let app_dir = get_app_data_dir()?;
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| AppError::FileError(e.to_string()))?;
    Ok(app_dir.join("novus_writer.db"))
}

/// Get the backups directory
pub fn get_backups_dir() -> Result<PathBuf, AppError> {
    let app_dir = get_app_data_dir()?;
    let backups_dir = app_dir.join("backups");
    std::fs::create_dir_all(&backups_dir)
        .map_err(|e| AppError::FileError(e.to_string()))?;
    Ok(backups_dir)
}

/// Generate a safe filename from a string
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' => c,
            _ => '_',
        })
        .collect()
}

/// Get file extension
pub fn get_extension(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}

/// Check if file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

/// Create directory if it doesn't exist
pub fn ensure_dir_exists(path: &Path) -> Result<(), AppError> {
    std::fs::create_dir_all(path)
        .map_err(|e| AppError::FileError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Hello World!"), "Hello_World_");
        assert_eq!(sanitize_filename("test-file.txt"), "test-file.txt");
        assert_eq!(sanitize_filename("doc<>name"), "doc__name");
    }

    #[test]
    fn test_get_extension() {
        let path = Path::new("/home/user/document.txt");
        assert_eq!(get_extension(path), Some("txt"));
        
        let path_no_ext = Path::new("/home/user/document");
        assert_eq!(get_extension(path_no_ext), None);
    }
}
