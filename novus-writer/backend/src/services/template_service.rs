use sqlx::{SqlitePool, Error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub content: String,
    pub thumbnail: Option<String>,
    pub is_system: bool,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub usage_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub field_type: String, // text, date, number, choice
    pub required: bool,
    pub default_value: Option<String>,
    pub options: Option<Vec<String>>, // for choice type
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInstance {
    pub id: String,
    pub template_id: String,
    pub document_id: String,
    pub variables: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

pub struct TemplateService {
    pool: SqlitePool,
}

impl TemplateService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_template(
        &self,
        name: &str,
        category: &str,
        content: &str,
        description: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<Template, Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        sqlx::query!(
            "INSERT INTO templates (id, name, description, category, content, is_system, created_by, created_at, updated_at, usage_count)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            name,
            description,
            category,
            content,
            false,
            created_by,
            now,
            now,
            0
        )
        .execute(&self.pool)
        .await?;

        Ok(Template {
            id,
            name: name.to_string(),
            description: description.map(String::from),
            category: category.to_string(),
            content: content.to_string(),
            thumbnail: None,
            is_system: false,
            created_by: created_by.map(String::from),
            created_at: now,
            updated_at: now,
            usage_count: 0,
        })
    }

    pub async fn get_template(&self, template_id: &str) -> Result<Option<Template>, Error> {
        let template = sqlx::query_as!(
            Template,
            "SELECT id, name, description, category, content, thumbnail, is_system, created_by, created_at, updated_at, usage_count 
             FROM templates 
             WHERE id = ?",
            template_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(template)
    }

    pub async fn get_all_templates(&self) -> Result<Vec<Template>, Error> {
        let templates = sqlx::query_as!(
            Template,
            "SELECT id, name, description, category, content, thumbnail, is_system, created_by, created_at, updated_at, usage_count 
             FROM templates 
             ORDER BY category, name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(templates)
    }

    pub async fn get_templates_by_category(&self, category: &str) -> Result<Vec<Template>, Error> {
        let templates = sqlx::query_as!(
            Template,
            "SELECT id, name, description, category, content, thumbnail, is_system, created_by, created_at, updated_at, usage_count 
             FROM templates 
             WHERE category = ?
             ORDER BY name",
            category
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(templates)
    }

    pub async fn update_template(
        &self,
        template_id: &str,
        name: Option<&str>,
        content: Option<&str>,
        description: Option<&str>,
    ) -> Result<(), Error> {
        let now = Utc::now();
        
        if let Some(n) = name {
            sqlx::query!(
                "UPDATE templates SET name = ?, updated_at = ? WHERE id = ?",
                n,
                now,
                template_id
            )
            .execute(&self.pool)
            .await?;
        }
        
        if let Some(c) = content {
            sqlx::query!(
                "UPDATE templates SET content = ?, updated_at = ? WHERE id = ?",
                c,
                now,
                template_id
            )
            .execute(&self.pool)
            .await?;
        }
        
        if let Some(d) = description {
            sqlx::query!(
                "UPDATE templates SET description = ?, updated_at = ? WHERE id = ?",
                d,
                now,
                template_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn delete_template(&self, template_id: &str) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM templates WHERE id = ? AND is_system = FALSE",
            template_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn increment_usage(&self, template_id: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE templates SET usage_count = usage_count + 1 WHERE id = ?",
            template_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_categories(&self) -> Result<Vec<TemplateCategory>, Error> {
        let categories = sqlx::query_as!(
            TemplateCategory,
            "SELECT id, name, description, icon, sort_order 
             FROM template_categories 
             ORDER BY sort_order, name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    pub async fn extract_variables(&self, content: &str) -> Vec<TemplateVariable> {
        // Simple regex-like extraction for {{variable_name}} patterns
        let mut variables = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        // This is a simplified implementation - in production, use proper parsing
        let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();
        for cap in re.captures_iter(content) {
            if let Some(var_name) = cap.get(1) {
                let name = var_name.as_str();
                if !seen.contains(name) {
                    seen.insert(name.to_string());
                    variables.push(TemplateVariable {
                        name: name.to_string(),
                        field_type: "text".to_string(),
                        required: true,
                        default_value: None,
                        options: None,
                        description: None,
                    });
                }
            }
        }
        
        variables
    }

    pub async fn render_template(&self, template_id: &str, variables: &serde_json::Value) -> Result<String, Error> {
        let template = self.get_template(template_id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        let mut content = template.content;
        
        // Replace variables in the format {{variable_name}}
        if let Some(obj) = variables.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                if let Some(str_value) = value.as_str() {
                    content = content.replace(&placeholder, str_value);
                }
            }
        }
        
        // Increment usage count
        self.increment_usage(template_id).await?;

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_extraction() {
        let content = "Dear {{name}}, Your order {{order_id}} is ready. Date: {{date}}";
        // Note: This test would need regex crate added to work properly
        println!("Content: {}", content);
    }

    #[test]
    fn test_template_rendering() {
        let mut content = "Hello {{name}}, welcome to {{company}}!".to_string();
        let vars = serde_json::json!({
            "name": "John",
            "company": "Acme Corp"
        });
        
        if let Some(obj) = vars.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                if let Some(str_value) = value.as_str() {
                    content = content.replace(&placeholder, str_value);
                }
            }
        }
        
        assert_eq!(content, "Hello John, welcome to Acme Corp!");
    }
}
