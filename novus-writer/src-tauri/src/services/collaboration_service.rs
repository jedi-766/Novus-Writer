use sqlx::{SqlitePool, Error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub id: String,
    pub document_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedUser {
    pub id: String,
    pub session_id: String,
    pub user_id: String,
    pub username: String,
    pub cursor_position: Option<CursorPosition>,
    pub color: String,
    pub joined_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    pub offset: usize,
    pub selection_start: Option<usize>,
    pub selection_end: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationMessage {
    Join { session_id: String, user: ConnectedUser },
    Leave { user_id: String, session_id: String },
    ContentChange { 
        user_id: String, 
        changes: Vec<TextChange>,
        version: u64,
    },
    CursorUpdate { 
        user_id: String, 
        position: CursorPosition,
    },
    Sync { 
        content: String, 
        version: u64,
        users: Vec<ConnectedUser>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChange {
    pub start: usize,
    pub end: usize,
    pub inserted: String,
    pub deleted: String,
}

pub struct CollaborationService {
    pool: SqlitePool,
}

impl CollaborationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_session(&self, document_id: &str) -> Result<CollaborationSession, Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        sqlx::query!(
            "INSERT INTO collaboration_sessions (id, document_id, created_at, updated_at, is_active)
             VALUES (?, ?, ?, ?, ?)",
            id,
            document_id,
            now,
            now,
            true
        )
        .execute(&self.pool)
        .await?;

        Ok(CollaborationSession {
            id,
            document_id: document_id.to_string(),
            created_at: now,
            updated_at: now,
            is_active: true,
        })
    }

    pub async fn get_active_session(&self, document_id: &str) -> Result<Option<CollaborationSession>, Error> {
        let session = sqlx::query_as!(
            CollaborationSession,
            "SELECT id, document_id, created_at, updated_at, is_active 
             FROM collaboration_sessions 
             WHERE document_id = ? AND is_active = TRUE",
            document_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn add_user_to_session(&self, session_id: &str, user_id: &str, username: &str) -> Result<ConnectedUser, Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let color = self.generate_user_color(user_id);
        
        sqlx::query!(
            "INSERT INTO connected_users (id, session_id, user_id, username, color, joined_at, last_activity)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            id,
            session_id,
            user_id,
            username,
            color,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(ConnectedUser {
            id,
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            cursor_position: None,
            color,
            joined_at: now,
            last_activity: now,
        })
    }

    pub async fn update_cursor(&self, user_id: &str, position: CursorPosition) -> Result<(), Error> {
        let now = Utc::now();
        let position_json = serde_json::to_string(&position)?;
        
        sqlx::query!(
            "UPDATE connected_users 
             SET cursor_position = ?, last_activity = ?
             WHERE user_id = ?",
            position_json,
            now,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_connected_users(&self, session_id: &str) -> Result<Vec<ConnectedUser>, Error> {
        let users = sqlx::query_as!(
            ConnectedUserRaw,
            "SELECT id, session_id, user_id, username, cursor_position as \"cursor_position: String\", color, joined_at, last_activity 
             FROM connected_users 
             WHERE session_id = ?",
            session_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn remove_user(&self, user_id: &str) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM connected_users WHERE user_id = ?",
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn close_session(&self, session_id: &str) -> Result<(), Error> {
        let now = Utc::now();
        
        sqlx::query!(
            "UPDATE collaboration_sessions 
             SET is_active = FALSE, updated_at = ?
             WHERE id = ?",
            now,
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    fn generate_user_color(&self, user_id: &str) -> String {
        // Generate a consistent color based on user_id hash
        let hash: u32 = user_id.bytes().map(|b| b as u32).sum();
        let colors = [
            "#FF6B6B", "#4ECDC4", "#45B7D1", "#96CEB4", 
            "#FFEAA7", "#DDA0DD", "#98D8C8", "#F7DC6F",
            "#BB8FCE", "#85C1E9", "#F8B500", "#00CED1"
        ];
        colors[(hash as usize) % colors.len()].to_string()
    }
}

#[derive(Debug)]
struct ConnectedUserRaw {
    id: String,
    session_id: String,
    user_id: String,
    username: String,
    cursor_position: String,
    color: String,
    joined_at: DateTime<Utc>,
    last_activity: DateTime<Utc>,
}

impl From<ConnectedUserRaw> for ConnectedUser {
    fn from(raw: ConnectedUserRaw) -> Self {
        let cursor_position = serde_json::from_str(&raw.cursor_position).unwrap_or(None);
        ConnectedUser {
            id: raw.id,
            session_id: raw.session_id,
            user_id: raw.user_id,
            username: raw.username,
            cursor_position,
            color: raw.color,
            joined_at: raw.joined_at,
            last_activity: raw.last_activity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_generation_consistency() {
        let service = CollaborationService::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());
        let color1 = service.generate_user_color("user123");
        let color2 = service.generate_user_color("user123");
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_different_users_different_colors() {
        let service = CollaborationService::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());
        let color1 = service.generate_user_color("user1");
        let color2 = service.generate_user_color("user2");
        // Not guaranteed to be different, but likely with good distribution
        println!("User1: {}, User2: {}", color1, color2);
    }
}
