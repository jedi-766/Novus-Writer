//! Plugin trait definitions

use crate::models::error::AppError;
use trait_variant::make_trait;

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

/// Base plugin trait
#[make_trait(async)]
pub trait Plugin: Send + Sync {
    fn info(&self) -> PluginInfo;
    async fn initialize(&self) -> Result<(), AppError>;
    async fn shutdown(&self) -> Result<(), AppError>;
}

/// Editor plugin for extending editor functionality
#[make_trait(async)]
pub trait EditorPlugin: Plugin {
    async fn on_content_change(&self, content: &str) -> Result<(), AppError>;
    async fn on_selection_change(&self, selection: &str) -> Result<(), AppError>;
}

/// Export plugin for adding export formats
#[make_trait(async)]
pub trait ExportPlugin: Plugin {
    fn supported_formats(&self) -> Vec<String>;
    async fn export(&self, content: &str, format: &str, path: &str) -> Result<(), AppError>;
}

/// Import plugin for adding import formats
#[make_trait(async)]
pub trait ImportPlugin: Plugin {
    fn supported_formats(&self) -> Vec<String>;
    async fn import(&self, path: &str) -> Result<String, AppError>;
}

/// Grammar check plugin
#[make_trait(async)]
pub trait GrammarPlugin: Plugin {
    async fn check(&self, content: &str) -> Result<Vec<GrammarIssue>, AppError>;
    async fn apply_fix(&self, content: &str, issue_id: usize) -> Result<String, AppError>;
}

/// Grammar issue found by grammar plugins
#[derive(Debug, Clone)]
pub struct GrammarIssue {
    pub id: usize,
    pub message: String,
    pub start: usize,
    pub end: usize,
    pub severity: IssueSeverity,
    pub suggestions: Vec<String>,
}

/// Severity level for grammar issues
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
}
