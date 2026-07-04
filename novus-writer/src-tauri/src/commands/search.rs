//! Search commands - Tauri command handlers for find and replace operations

use crate::models::error::AppError;
use regex::Regex;

/// Search result match
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub position: usize,
    pub length: usize,
    pub context: String,
}

/// Find text in content
#[tauri::command]
pub fn find_text(
    content: String,
    query: String,
    case_sensitive: bool,
    use_regex: bool,
) -> Result<Vec<SearchResult>, AppError> {
    let mut results = Vec::new();
    
    let search_text = if case_sensitive {
        content.clone()
    } else {
        content.to_lowercase()
    };
    
    let search_query = if case_sensitive {
        query.clone()
    } else {
        query.to_lowercase()
    };
    
    if use_regex {
        let re = Regex::new(&search_query).map_err(|e| {
            AppError::InvalidOperation(format!("Invalid regex: {}", e))
        })?;
        
        for mat in re.find_iter(&search_text) {
            let start = mat.start();
            let end = mat.end();
            let context_start = start.saturating_sub(50);
            let context_end = (end + 50).min(search_text.len());
            
            results.push(SearchResult {
                position: start,
                length: end - start,
                context: search_text[context_start..context_end].to_string(),
            });
        }
    } else {
        let mut start = 0;
        while let Some(pos) = search_text[start..].find(&search_query) {
            let absolute_pos = start + pos;
            let end = absolute_pos + search_query.len();
            let context_start = absolute_pos.saturating_sub(50);
            let context_end = (end + 50).min(search_text.len());
            
            results.push(SearchResult {
                position: absolute_pos,
                length: search_query.len(),
                context: search_text[context_start..context_end].to_string(),
            });
            
            start = end;
        }
    }
    
    tracing::debug!("Found {} matches for query", results.len());
    Ok(results)
}

/// Replace text in content
#[tauri::command]
pub fn replace_text(
    content: String,
    search: String,
    replace: String,
    replace_all: bool,
    case_sensitive: bool,
    use_regex: bool,
) -> Result<String, AppError> {
    if use_regex {
        let pattern = if case_sensitive {
            search.clone()
        } else {
            format!("(?i){}", search)
        };
        
        let re = Regex::new(&pattern).map_err(|e| {
            AppError::InvalidOperation(format!("Invalid regex: {}", e))
        })?;
        
        if replace_all {
            Ok(re.replace_all(&content, replace.as_str()).to_string())
        } else {
            Ok(re.replace(&content, replace.as_str()).to_string())
        }
    } else {
        let (search, replace) = if case_sensitive {
            (search, replace)
        } else {
            // For case-insensitive without regex, we need custom logic
            return Err(AppError::InvalidOperation(
                "Case-insensitive search requires regex".to_string(),
            ));
        };
        
        if replace_all {
            Ok(content.replace(&search, &replace))
        } else {
            let mut result = content.clone();
            if let Some(pos) = content.find(&search) {
                result.replace_range(pos..pos + search.len(), &replace);
            }
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_text_simple() {
        let content = "Hello world! Hello Rust!".to_string();
        let results = find_text(content, "Hello".to_string(), false, false).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].position, 0);
        assert_eq!(results[1].position, 13);
    }

    #[test]
    fn test_replace_text() {
        let content = "Hello world!".to_string();
        let result = replace_text(
            content,
            "world".to_string(),
            "Rust".to_string(),
            false,
            true,
            false,
        )
        .unwrap();
        assert_eq!(result, "Hello Rust!");
    }
}
