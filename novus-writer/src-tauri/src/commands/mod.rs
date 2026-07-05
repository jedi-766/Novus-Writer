//! Commands module - Tauri command handlers

pub mod document;
pub mod editor;
pub mod search;
pub mod export;
pub mod version;
pub mod autosave;
pub mod bookmark;
pub mod backup;

// Re-export all commands for easy access
pub use document::*;
pub use editor::*;
pub use search::*;
pub use export::*;
pub use version::*;
pub use autosave::*;
pub use bookmark::*;
pub use backup::*;
