use sqlx::{SqlitePool, Error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailMergeField {
    pub id: String,
    pub name: String,
    pub field_type: String, // text, date, number, currency, address
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailMergeRecipient {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub custom_fields: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailMergeJob {
    pub id: String,
    pub document_id: String,
    pub template_content: String,
    pub status: String, // draft, processing, completed, failed
    pub total_recipients: u32,
    pub processed_count: u32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailMergeResult {
    pub job_id: String,
    pub recipient_id: String,
    pub output_document_id: String,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct MailMergeService {
    pool: SqlitePool,
}

impl MailMergeService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_job(
        &self,
        document_id: &str,
        template_content: &str,
        recipient_ids: &[String],
        created_by: &str,
    ) -> Result<MailMergeJob, Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let total = recipient_ids.len() as u32;
        
        sqlx::query!(
            "INSERT INTO mail_merge_jobs (id, document_id, template_content, status, total_recipients, processed_count, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            document_id,
            template_content,
            "draft",
            total,
            0,
            created_by,
            now
        )
        .execute(&self.pool)
        .await?;

        // Store recipient associations
        for recipient_id in recipient_ids {
            sqlx::query!(
                "INSERT INTO mail_merge_recipients (job_id, recipient_id) VALUES (?, ?)",
                id,
                recipient_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(MailMergeJob {
            id,
            document_id: document_id.to_string(),
            template_content: template_content.to_string(),
            status: "draft".to_string(),
            total_recipients: total,
            processed_count: 0,
            created_by: created_by.to_string(),
            created_at: now,
            completed_at: None,
        })
    }

    pub async fn get_job(&self, job_id: &str) -> Result<Option<MailMergeJob>, Error> {
        let job = sqlx::query_as!(
            MailMergeJob,
            "SELECT id, document_id, template_content, status, total_recipients, processed_count, created_by, created_at, completed_at 
             FROM mail_merge_jobs 
             WHERE id = ?",
            job_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn add_recipient(
        &self,
        first_name: &str,
        last_name: &str,
        email: Option<&str>,
        custom_fields: &serde_json::Value,
    ) -> Result<MailMergeRecipient, Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let fields_json = serde_json::to_string(custom_fields)?;
        
        sqlx::query!(
            "INSERT INTO mail_merge_recipients_list (id, first_name, last_name, email, custom_fields, created_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            id,
            first_name,
            last_name,
            email,
            fields_json,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(MailMergeRecipient {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.map(String::from),
            address_line1: None,
            address_line2: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            custom_fields: custom_fields.clone(),
            created_at: now,
        })
    }

    pub async fn update_recipient_address(
        &self,
        recipient_id: &str,
        address_line1: Option<&str>,
        address_line2: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        postal_code: Option<&str>,
        country: Option<&str>,
    ) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE mail_merge_recipients_list 
             SET address_line1 = ?, address_line2 = ?, city = ?, state = ?, postal_code = ?, country = ?
             WHERE id = ?",
            address_line1,
            address_line2,
            city,
            state,
            postal_code,
            country,
            recipient_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recipients(&self, job_id: &str) -> Result<Vec<MailMergeRecipient>, Error> {
        let recipients = sqlx::query_as!(
            RecipientRaw,
            "SELECT r.id, r.first_name, r.last_name, r.email, r.address_line1, r.address_line2, 
                    r.city, r.state, r.postal_code, r.country, 
                    r.custom_fields as \"custom_fields: String\", r.created_at
             FROM mail_merge_recipients_list r
             JOIN mail_merge_recipients mr ON r.id = mr.recipient_id
             WHERE mr.job_id = ?",
            job_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recipients.into_iter().map(|r| r.into()).collect())
    }

    pub async fn process_job(&self, job_id: &str) -> Result<Vec<MailMergeResult>, Error> {
        let job = self.get_job(job_id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        // Update status to processing
        sqlx::query!(
            "UPDATE mail_merge_jobs SET status = ? WHERE id = ?",
            "processing",
            job_id
        )
        .execute(&self.pool)
        .await?;

        let recipients = self.get_recipients(job_id).await?;
        let mut results = Vec::new();
        let mut processed = 0u32;

        for recipient in recipients {
            match self.merge_document(&job.template_content, &recipient) {
                Ok(output_content) => {
                    // In a real implementation, this would create a new document
                    let output_doc_id = format!("merged_{}_{}", job_id, recipient.id);
                    
                    let result = MailMergeResult {
                        job_id: job_id.to_string(),
                        recipient_id: recipient.id,
                        output_document_id: output_doc_id,
                        status: "completed".to_string(),
                        error_message: None,
                        created_at: Utc::now(),
                    };
                    
                    results.push(result);
                    processed += 1;
                }
                Err(e) => {
                    let result = MailMergeResult {
                        job_id: job_id.to_string(),
                        recipient_id: recipient.id,
                        output_document_id: String::new(),
                        status: "failed".to_string(),
                        error_message: Some(e.to_string()),
                        created_at: Utc::now(),
                    };
                    
                    results.push(result);
                }
            }
        }

        // Update job status
        let now = Utc::now();
        sqlx::query!(
            "UPDATE mail_merge_jobs 
             SET status = ?, processed_count = ?, completed_at = ?
             WHERE id = ?",
            "completed",
            processed,
            now,
            job_id
        )
        .execute(&self.pool)
        .await?;

        Ok(results)
    }

    fn merge_document(
        &self,
        template: &str,
        recipient: &MailMergeRecipient,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut content = template.to_string();

        // Replace standard fields
        content = content.replace("{{first_name}}", &recipient.first_name);
        content = content.replace("{{last_name}}", &recipient.last_name);
        
        if let Some(email) = &recipient.email {
            content = content.replace("{{email}}", email);
        }
        
        if let Some(address) = &recipient.address_line1 {
            content = content.replace("{{address_line1}}", address);
        }
        
        if let Some(city) = &recipient.city {
            content = content.replace("{{city}}", city);
        }
        
        if let Some(state) = &recipient.state {
            content = content.replace("{{state}}", state);
        }
        
        if let Some(postal) = &recipient.postal_code {
            content = content.replace("{{postal_code}}", postal);
        }

        // Replace custom fields
        if let Some(obj) = recipient.custom_fields.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                if let Some(str_value) = value.as_str() {
                    content = content.replace(&placeholder, str_value);
                }
            }
        }

        Ok(content)
    }

    pub async fn cancel_job(&self, job_id: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE mail_merge_jobs SET status = ? WHERE id = ? AND status IN ('draft', 'processing')",
            "cancelled",
            job_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_job_history(&self, document_id: &str) -> Result<Vec<MailMergeJob>, Error> {
        let jobs = sqlx::query_as!(
            MailMergeJob,
            "SELECT id, document_id, template_content, status, total_recipients, processed_count, created_by, created_at, completed_at 
             FROM mail_merge_jobs 
             WHERE document_id = ?
             ORDER BY created_at DESC",
            document_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
    }
}

#[derive(Debug)]
struct RecipientRaw {
    id: String,
    first_name: String,
    last_name: String,
    email: Option<String>,
    address_line1: Option<String>,
    address_line2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    custom_fields: String,
    created_at: DateTime<Utc>,
}

impl From<RecipientRaw> for MailMergeRecipient {
    fn from(raw: RecipientRaw) -> Self {
        let custom_fields = serde_json::from_str(&raw.custom_fields).unwrap_or(serde_json::Value::Null);
        MailMergeRecipient {
            id: raw.id,
            first_name: raw.first_name,
            last_name: raw.last_name,
            email: raw.email,
            address_line1: raw.address_line1,
            address_line2: raw.address_line2,
            city: raw.city,
            state: raw.state,
            postal_code: raw.postal_code,
            country: raw.country,
            custom_fields,
            created_at: raw.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_document() {
        let service = MailMergeService::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());
        let template = "Dear {{first_name}} {{last_name}},\n\nYour email {{email}} is confirmed.\n\nSincerely,\nThe Team";
        
        let recipient = MailMergeRecipient {
            id: "test1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: Some("john.doe@example.com".to_string()),
            address_line1: None,
            address_line2: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            custom_fields: serde_json::Value::Null,
            created_at: Utc::now(),
        };

        let result = service.merge_document(template, &recipient).unwrap();
        
        assert!(result.contains("Dear John Doe"));
        assert!(result.contains("john.doe@example.com"));
    }

    #[test]
    fn test_merge_with_custom_fields() {
        let service = MailMergeService::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());
        let template = "Order {{order_id}} for {{product}} totaling {{amount}}";
        
        let custom_fields = serde_json::json!({
            "order_id": "ORD-12345",
            "product": "Premium Widget",
            "amount": "$99.99"
        });
        
        let recipient = MailMergeRecipient {
            id: "test2".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            email: None,
            address_line1: None,
            address_line2: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            custom_fields,
            created_at: Utc::now(),
        };

        let result = service.merge_document(template, &recipient).unwrap();
        
        assert!(result.contains("Order ORD-12345"));
        assert!(result.contains("Premium Widget"));
        assert!(result.contains("$99.99"));
    }
}
