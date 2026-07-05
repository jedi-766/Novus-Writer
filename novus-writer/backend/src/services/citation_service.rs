use sqlx::{SqlitePool, Error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub id: String,
    pub document_id: String,
    pub citation_type: String, // apa, mla, chicago, ieee, harvard
    pub entry_type: String, // book, journal_article, website, conference, thesis, etc.
    pub fields: serde_json::Value,
    pub formatted_citation: String,
    pub bibliography_entry: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationStyle {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bibliography {
    pub id: String,
    pub document_id: String,
    pub style: String,
    pub entries: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

pub struct CitationService {
    pool: SqlitePool,
}

impl CitationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_citation(
        &self,
        document_id: &str,
        citation_type: &str,
        entry_type: &str,
        fields: &serde_json::Value,
    ) -> Result<Citation, Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        // Format the citation based on type and entry
        let (formatted_citation, bibliography_entry) = self.format_citation(citation_type, entry_type, fields);
        
        let fields_json = serde_json::to_string(fields)?;
        
        sqlx::query!(
            "INSERT INTO citations (id, document_id, citation_type, entry_type, fields, formatted_citation, bibliography_entry, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            document_id,
            citation_type,
            entry_type,
            fields_json,
            formatted_citation,
            bibliography_entry,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(Citation {
            id,
            document_id: document_id.to_string(),
            citation_type: citation_type.to_string(),
            entry_type: entry_type.to_string(),
            fields: fields.clone(),
            formatted_citation,
            bibliography_entry,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_citations(&self, document_id: &str) -> Result<Vec<Citation>, Error> {
        let citations = sqlx::query_as!(
            CitationRaw,
            "SELECT id, document_id, citation_type, entry_type, fields as \"fields: String\", formatted_citation, bibliography_entry, created_at, updated_at 
             FROM citations 
             WHERE document_id = ?
             ORDER BY created_at",
            document_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(citations.into_iter().map(|c| c.into()).collect())
    }

    pub async fn update_citation(
        &self,
        citation_id: &str,
        fields: &serde_json::Value,
    ) -> Result<Citation, Error> {
        let now = Utc::now();
        let fields_json = serde_json::to_string(fields)?;
        
        // Get existing citation to determine type
        let existing = sqlx::query_as!(
            CitationRaw,
            "SELECT id, document_id, citation_type, entry_type, fields as \"fields: String\", formatted_citation, bibliography_entry, created_at, updated_at 
             FROM citations 
             WHERE id = ?",
            citation_id
        )
        .fetch_one(&self.pool)
        .await?;

        let fields_value: serde_json::Value = serde_json::from_str(&existing.fields)?;
        let (formatted_citation, bibliography_entry) = 
            self.format_citation(&existing.citation_type, &existing.entry_type, fields);

        sqlx::query!(
            "UPDATE citations 
             SET fields = ?, formatted_citation = ?, bibliography_entry = ?, updated_at = ?
             WHERE id = ?",
            fields_json,
            formatted_citation,
            bibliography_entry,
            now,
            citation_id
        )
        .execute(&self.pool)
        .await?;

        Ok(Citation {
            id: existing.id,
            document_id: existing.document_id,
            citation_type: existing.citation_type,
            entry_type: existing.entry_type,
            fields: fields.clone(),
            formatted_citation,
            bibliography_entry,
            created_at: existing.created_at,
            updated_at: now,
        })
    }

    pub async fn delete_citation(&self, citation_id: &str) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM citations WHERE id = ?",
            citation_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn generate_bibliography(&self, document_id: &str, style: &str) -> Result<Bibliography, Error> {
        let citations = self.get_citations(document_id).await?;
        let now = Utc::now();
        
        let mut entries = Vec::new();
        for citation in citations {
            entries.push(citation.bibliography_entry);
        }

        // Sort entries alphabetically by first author
        entries.sort();

        let id = Uuid::new_v4().to_string();
        
        Ok(Bibliography {
            id,
            document_id: document_id.to_string(),
            style: style.to_string(),
            entries,
            generated_at: now,
        })
    }

    pub async fn get_citation_styles(&self) -> Result<Vec<CitationStyle>, Error> {
        let styles = vec![
            CitationStyle {
                id: "apa".to_string(),
                name: "APA 7th Edition".to_string(),
                description: Some("American Psychological Association".to_string()),
                is_default: true,
            },
            CitationStyle {
                id: "mla".to_string(),
                name: "MLA 9th Edition".to_string(),
                description: Some("Modern Language Association".to_string()),
                is_default: false,
            },
            CitationStyle {
                id: "chicago".to_string(),
                name: "Chicago Manual of Style".to_string(),
                description: Some("University of Chicago Press".to_string()),
                is_default: false,
            },
            CitationStyle {
                id: "ieee".to_string(),
                name: "IEEE Style".to_string(),
                description: Some("Institute of Electrical and Electronics Engineers".to_string()),
                is_default: false,
            },
            CitationStyle {
                id: "harvard".to_string(),
                name: "Harvard Style".to_string(),
                description: Some("Author-Date System".to_string()),
                is_default: false,
            },
        ];

        Ok(styles)
    }

    fn format_citation(
        &self,
        citation_type: &str,
        entry_type: &str,
        fields: &serde_json::Value,
    ) -> (String, String) {
        // Simplified citation formatting - in production, use a proper CSL processor
        let obj = fields.as_object().unwrap_or(&serde_json::Map::new());
        
        let author = obj.get("author")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown Author");
        
        let title = obj.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled");
        
        let year = obj.get("year")
            .and_then(|v| v.as_str())
            .unwrap_or("n.d.");

        match citation_type {
            "apa" => self.format_apa(author, title, year, entry_type, obj),
            "mla" => self.format_mla(author, title, year, entry_type, obj),
            "chicago" => self.format_chicago(author, title, year, entry_type, obj),
            "ieee" => self.format_ieee(author, title, year, entry_type, obj),
            "harvard" => self.format_harvard(author, title, year, entry_type, obj),
            _ => (format!("({}) {}", author, year), format!("{}. ({})", author, year)),
        }
    }

    fn format_apa(
        &self,
        author: &str,
        title: &str,
        year: &str,
        entry_type: &str,
        _fields: &serde_json::Map<String, serde_json::Value>,
    ) -> (String, String) {
        let in_text = format!("({}, {})", author, year);
        let bibliography = format!("{}. ({}). <i>{}</i>.", author, year, title);
        (in_text, bibliography)
    }

    fn format_mla(
        &self,
        author: &str,
        title: &str,
        year: &str,
        entry_type: &str,
        _fields: &serde_json::Map<String, serde_json::Value>,
    ) -> (String, String) {
        let in_text = format!("({} {})", author, year);
        let bibliography = format!("{}. <i>{}</i>. {}", author, title, year);
        (in_text, bibliography)
    }

    fn format_chicago(
        &self,
        author: &str,
        title: &str,
        year: &str,
        entry_type: &str,
        _fields: &serde_json::Map<String, serde_json::Value>,
    ) -> (String, String) {
        let in_text = format!("({} {}, {})", author, year);
        let bibliography = format!("{}. <i>{}</i> ({}).", author, title, year);
        (in_text, bibliography)
    }

    fn format_ieee(
        &self,
        author: &str,
        title: &str,
        year: &str,
        entry_type: &str,
        _fields: &serde_json::Map<String, serde_json::Value>,
    ) -> (String, String) {
        let in_text = format!("[{}] ", author.chars().next().unwrap_or('A'));
        let bibliography = format!("[1] {}. \"{}.\", {}", author, title, year);
        (in_text, bibliography)
    }

    fn format_harvard(
        &self,
        author: &str,
        title: &str,
        year: &str,
        entry_type: &str,
        _fields: &serde_json::Map<String, serde_json::Value>,
    ) -> (String, String) {
        let in_text = format!("({}, {})", author, year);
        let bibliography = format!("{}. ({}) <i>{}</i>.", author, year, title);
        (in_text, bibliography)
    }
}

#[derive(Debug)]
struct CitationRaw {
    id: String,
    document_id: String,
    citation_type: String,
    entry_type: String,
    fields: String,
    formatted_citation: String,
    bibliography_entry: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<CitationRaw> for Citation {
    fn from(raw: CitationRaw) -> Self {
        let fields = serde_json::from_str(&raw.fields).unwrap_or(serde_json::Value::Null);
        Citation {
            id: raw.id,
            document_id: raw.document_id,
            citation_type: raw.citation_type,
            entry_type: raw.entry_type,
            fields,
            formatted_citation: raw.formatted_citation,
            bibliography_entry: raw.bibliography_entry,
            created_at: raw.created_at,
            updated_at: raw.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apa_formatting() {
        let service = CitationService::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());
        let fields = serde_json::json!({
            "author": "Smith, J.",
            "title": "The Art of Programming",
            "year": "2023"
        });
        
        let (in_text, biblio) = service.format_apa(
            "Smith, J.",
            "The Art of Programming",
            "2023",
            "book",
            fields.as_object().unwrap()
        );
        
        assert_eq!(in_text, "(Smith, J., 2023)");
        assert!(biblio.contains("Smith, J."));
        assert!(biblio.contains("2023"));
        assert!(biblio.contains("The Art of Programming"));
    }

    #[test]
    fn test_mla_formatting() {
        let service = CitationService::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());
        let (in_text, biblio) = service.format_mla(
            "Johnson, M.",
            "Digital Humanities",
            "2022",
            "journal_article",
            &serde_json::Map::new()
        );
        
        assert_eq!(in_text, "(Johnson, M. 2022)");
        assert!(biblio.contains("Johnson, M."));
        assert!(biblio.contains("Digital Humanities"));
    }
}
