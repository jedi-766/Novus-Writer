//! Novus Writer - A modern, offline-first desktop notes application
//! 
//! This is the main library crate containing all business logic,
//! database operations, editor commands, and services.

pub mod core;
pub mod database;
pub mod editor;
pub mod commands;
pub mod services;
pub mod models;
pub mod utils;
pub mod plugins;

// Re-export commonly used types
pub use models::document::Document;
pub use models::error::AppError;
pub use services::document_service::DocumentService;
pub use database::Database;
