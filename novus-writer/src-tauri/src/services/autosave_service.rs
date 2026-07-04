//! Autosave service - Automatic document saving

use crate::core::domain::document::DocumentId;
use crate::models::error::AppError;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::interval;
use tokio::sync::watch;

/// Autosave configuration
#[derive(Debug, Clone)]
pub struct AutosaveConfig {
    /// Interval between autosaves in seconds
    pub interval_seconds: u64,
    /// Maximum number of autosaves to keep per document
    pub max_autosaves: usize,
}

impl Default for AutosaveConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            max_autosaves: 10,
        }
    }
}

/// Autosave service for automatic document saving
pub struct AutosaveService {
    pool: SqlitePool,
    config: AutosaveConfig,
    shutdown_tx: watch::Sender<bool>,
}

impl AutosaveService {
    /// Create a new autosave service
    pub fn new(pool: SqlitePool, config: AutosaveConfig) -> Self {
        let (shutdown_tx, _) = watch::channel(false);
        Self {
            pool,
            config,
            shutdown_tx,
        }
    }

    /// Start the autosave background task
    pub fn start(&self) -> tokio::task::JoinHandle<()> {
        let pool = self.pool.clone();
        let config = self.config.clone();
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.interval_seconds));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Perform autosave cleanup
                        if let Err(e) = Self::cleanup_old_autosaves(&pool, config.max_autosaves).await {
                            tracing::error!("Autosave cleanup failed: {}", e);
                        }
                    }
                    _ = shutdown_rx.changed() => {
                        if *shutdown_rx.borrow() {
                            break;
                        }
                    }
                }
            }
            
            tracing::info!("Autosave service stopped");
        })
    }

    /// Save document content as autosave
    pub async fn save_autosave(
        &self,
        doc_id: &DocumentId,
        content: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO autosaves (document_id, content)
            VALUES (?, ?)
            "#,
        )
        .bind(doc_id)
        .bind(content)
        .execute(&self.pool)
        .await?;

        // Cleanup old autosaves
        Self::cleanup_old_autosaves(&self.pool, self.config.max_autosaves).await?;

        tracing::debug!("Autosaved document: {}", doc_id);
        Ok(())
    }

    /// Get the latest autosave for a document
    pub async fn get_latest_autosave(
        &self,
        doc_id: &DocumentId,
    ) -> Result<Option<String>, AppError> {
        let row = sqlx::query_scalar::<_, String>(
            r#"
            SELECT content
            FROM autosaves
            WHERE document_id = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(doc_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    /// Cleanup old autosaves keeping only the most recent ones
    async fn cleanup_old_autosaves(
        pool: &SqlitePool,
        max_count: usize,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM autosaves
            WHERE id NOT IN (
                SELECT id FROM (
                    SELECT id
                    FROM autosaves
                    GROUP BY document_id
                    ORDER BY created_at DESC
                    LIMIT ?
                )
            )
            "#,
        )
        .bind(max_count as i64)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Stop the autosave service
    pub fn stop(&self) {
        let _ = self.shutdown_tx.send(true);
    }

    /// Clear all autosaves for a document
    pub async fn clear_autosaves(&self, doc_id: &DocumentId) -> Result<(), AppError> {
        sqlx::query("DELETE FROM autosaves WHERE document_id = ?")
            .bind(doc_id)
            .execute(&self.pool)
            .await?;

        tracing::debug!("Cleared autosaves for document: {}", doc_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_autosave_config_default() {
        let config = AutosaveConfig::default();
        assert_eq!(config.interval_seconds, 30);
        assert_eq!(config.max_autosaves, 10);
    }
}
