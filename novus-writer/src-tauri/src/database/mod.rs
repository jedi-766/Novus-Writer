//! Database module - SQLite database operations with connection pooling

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;
use crate::models::error::AppError;
use std::time::Duration;

/// Database configuration
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

/// Database connection wrapper with connection pool
pub struct Database {
    pool: SqlitePool,
    config: DatabaseConfig,
}

impl Database {
    /// Create a new in-memory database instance (for testing)
    pub fn new() -> Self {
        let config = DatabaseConfig::default();
        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .connect_lazy("sqlite::memory:")
            .expect("Failed to create database pool");
        
        Self { pool, config }
    }

    /// Create database with file path
    pub async fn with_path(path: &Path) -> Result<Self, AppError> {
        Self::with_path_and_config(path, DatabaseConfig::default()).await
    }

    /// Create database with custom configuration
    pub async fn with_path_and_config(path: &Path, config: DatabaseConfig) -> Result<Self, AppError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .connect(path.to_str().ok_or_else(|| AppError::FileError("Invalid path".to_string()))?)
            .await?;
        
        Ok(Self { pool, config })
    }

    /// Initialize database schema from embedded SQL
    pub async fn init(&self) -> Result<(), AppError> {
        // Enable WAL mode for better concurrency
        sqlx::query("PRAGMA journal_mode = WAL")
            .execute(&self.pool)
            .await?;

        // Enable foreign keys
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await?;

        // Create documents table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content BLOB,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                version INTEGER DEFAULT 1,
                tags TEXT DEFAULT '[]',
                pinned INTEGER DEFAULT 0,
                word_count INTEGER DEFAULT 0,
                character_count INTEGER DEFAULT 0,
                thumbnail BLOB,
                last_opened_at DATETIME,
                metadata TEXT DEFAULT '{}'
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create document_versions table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_versions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id TEXT NOT NULL,
                version_number INTEGER NOT NULL,
                content BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                change_summary TEXT,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
                UNIQUE(document_id, version_number)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create autosaves table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS autosaves (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id TEXT NOT NULL,
                content BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create bookmarks table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id TEXT NOT NULL,
                name TEXT NOT NULL,
                position INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create backups table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS backups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id TEXT NOT NULL,
                backup_path TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                backup_size INTEGER,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for performance
        self.create_indexes().await?;

        // Create trigger for updating modified_at
        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS update_document_modified_at 
            AFTER UPDATE ON documents
            BEGIN
                UPDATE documents SET modified_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
            END
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create schema_migrations table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                description TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Record initial migration
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO schema_migrations (version, description) 
            VALUES ('1.0.0', 'Initial schema creation')
            "#,
        )
        .execute(&self.pool)
        .await?;

        tracing::info!("Database initialized successfully with WAL mode");
        Ok(())
    }

    /// Create performance indexes
    async fn create_indexes(&self) -> Result<(), AppError> {
        let indexes = [
            "CREATE INDEX IF NOT EXISTS idx_documents_modified_at ON documents(modified_at DESC)",
            "CREATE INDEX IF NOT EXISTS idx_documents_pinned ON documents(pinned DESC)",
            "CREATE INDEX IF NOT EXISTS idx_documents_last_opened ON documents(last_opened_at DESC)",
            "CREATE INDEX IF NOT EXISTS idx_documents_title ON documents(title)",
            "CREATE INDEX IF NOT EXISTS idx_versions_document_id ON document_versions(document_id)",
            "CREATE INDEX IF NOT EXISTS idx_versions_created_at ON document_versions(created_at DESC)",
            "CREATE INDEX IF NOT EXISTS idx_autosaves_document_id ON autosaves(document_id)",
            "CREATE INDEX IF NOT EXISTS idx_autosaves_created_at ON autosaves(created_at DESC)",
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_document_id ON bookmarks(document_id)",
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_name ON bookmarks(name)",
            "CREATE INDEX IF NOT EXISTS idx_backups_document_id ON backups(document_id)",
            "CREATE INDEX IF NOT EXISTS idx_backups_created_at ON backups(created_at DESC)",
        ];

        for index_sql in indexes.iter() {
            sqlx::query(index_sql)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Get the database pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Get database configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Close the database connection pool
    pub async fn close(&self) {
        self.pool.close().await;
        tracing::info!("Database connection pool closed");
    }

    /// Run database health check
    pub async fn health_check(&self) -> Result<(), AppError> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_initialization() {
        let db = Database::new();
        let result = db.init().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_database_health_check() {
        let db = Database::new();
        db.init().await.unwrap();
        let result = db.health_check().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_database_indexes_created() {
        let db = Database::new();
        db.init().await.unwrap();
        
        // Verify indexes exist by querying sqlite_master
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'"
        )
        .fetch_one(db.pool())
        .await;
        
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }
}
