//! Export Service - Handles document export to various formats

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::models::document::Document;
use crate::utils::file_utils;

/// Export formats supported by the application
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Pdf,
    Docx,
    Html,
    Markdown,
    PlainText,
    Rtf,
    Odt,
}

impl ExportFormat {
    pub fn from_extension(ext: &str) -> Option<ExportFormat> {
        match ext.to_lowercase().as_str() {
            "pdf" => Some(ExportFormat::Pdf),
            "docx" => Some(ExportFormat::Docx),
            "html" | "htm" => Some(ExportFormat::Html),
            "md" | "markdown" => Some(ExportFormat::Markdown),
            "txt" => Some(ExportFormat::PlainText),
            "rtf" => Some(ExportFormat::Rtf),
            "odt" => Some(ExportFormat::Odt),
            _ => None,
        }
    }

    pub fn get_mime_type(&self) -> &'static str {
        match self {
            ExportFormat::Pdf => "application/pdf",
            ExportFormat::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ExportFormat::Html => "text/html",
            ExportFormat::Markdown => "text/markdown",
            ExportFormat::PlainText => "text/plain",
            ExportFormat::Rtf => "application/rtf",
            ExportFormat::Odt => "application/vnd.oasis.opendocument.text",
        }
    }

    pub fn get_extension(&self) -> &'static str {
        match self {
            ExportFormat::Pdf => "pdf",
            ExportFormat::Docx => "docx",
            ExportFormat::Html => "html",
            ExportFormat::Markdown => "md",
            ExportFormat::PlainText => "txt",
            ExportFormat::Rtf => "rtf",
            ExportFormat::Odt => "odt",
        }
    }
}

/// Service for exporting documents to various formats
pub struct ExportService;

impl ExportService {
    /// Export a document to the specified format
    pub fn export(
        document: &Document,
        format: ExportFormat,
        output_path: PathBuf,
    ) -> Result<PathBuf, String> {
        match format {
            ExportFormat::Pdf => Self::export_to_pdf(document, &output_path),
            ExportFormat::Docx => Self::export_to_docx(document, &output_path),
            ExportFormat::Html => Self::export_to_html(document, &output_path),
            ExportFormat::Markdown => Self::export_to_markdown(document, &output_path),
            ExportFormat::PlainText => Self::export_to_plain_text(document, &output_path),
            ExportFormat::Rtf => Self::export_to_rtf(document, &output_path),
            ExportFormat::Odt => Self::export_to_odt(document, &output_path),
        }
    }

    /// Export document to PDF format
    /// Note: Full PDF generation requires a PDF library or backend service
    pub fn export_to_pdf(document: &Document, output_path: &PathBuf) -> Result<PathBuf, String> {
        // Placeholder implementation
        // In production, this would use a library like printpdf or call a backend service
        let content = format!(
            "%PDF-1.4\n% Novus Writer PDF Export (Simulation)\nTitle: {}\nContent: {}",
            document.title,
            document.content.as_ref().unwrap_or(&String::new())
        );
        
        Self::write_file(output_path, content.as_bytes())?;
        Ok(output_path.clone())
    }

    /// Export document to DOCX format
    /// Note: Full DOCX generation requires a library or backend service
    pub fn export_to_docx(document: &Document, output_path: &PathBuf) -> Result<PathBuf, String> {
        // Placeholder implementation
        // In production, this would use a library docx-rs or similar
        let content = format!(
            "[DOCX Document]\nTitle: {}\nContent: {}",
            document.title,
            document.content.as_ref().unwrap_or(&String::new())
        );
        
        Self::write_file(output_path, content.as_bytes())?;
        Ok(output_path.clone())
    }

    /// Export document to HTML format
    pub fn export_to_html(document: &Document, output_path: &PathBuf) -> Result<PathBuf, String> {
        let content = document.content.as_ref().unwrap_or(&String::new());
        
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: Georgia, 'Times New Roman', serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 40px 20px;
            background-color: #fff;
            color: #333;
        }}
        h1, h2, h3, h4, h5, h6 {{
            color: #2b579a;
            margin-top: 1.5em;
            margin-bottom: 0.5em;
        }}
        p {{
            margin-bottom: 1em;
        }}
        .metadata {{
            color: #666;
            font-size: 0.9em;
            border-bottom: 1px solid #eee;
            padding-bottom: 10px;
            margin-bottom: 20px;
        }}
    </style>
</head>
<body>
    <div class="metadata">
        <p><strong>Title:</strong> {}</p>
        <p><strong>Created:</strong> {}</p>
        <p><strong>Modified:</strong> {}</p>
    </div>
    <div class="content">
        {}
    </div>
</body>
</html>"#,
            document.title,
            document.title,
            document.created_at.format("%Y-%m-%d %H:%M"),
            document.updated_at.format("%Y-%m-%d %H:%M"),
            Self::content_to_html(content)
        );

        Self::write_file(output_path, html.as_bytes())?;
        Ok(output_path.clone())
    }

    /// Export document to Markdown format
    pub fn export_to_markdown(document: &Document, output_path: &PathBuf) -> Result<PathBuf, String> {
        let content = document.content.as_ref().unwrap_or(&String::new());
        
        let markdown = format!(
            "# {}\n\n{}\n\n---\n*Exported from Novus Writer on {}*",
            document.title,
            content,
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        );

        Self::write_file(output_path, markdown.as_bytes())?;
        Ok(output_path.clone())
    }

    /// Export document to plain text format
    pub fn export_to_plain_text(document: &Document, output_path: &PathBuf) -> Result<PathBuf, String> {
        let content = document.content.as_ref().unwrap_or(&String::new());
        
        let text = format!(
            "{}\n\n{}\n\n---\nExported from Novus Writer on {}",
            document.title,
            content,
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        );

        Self::write_file(output_path, text.as_bytes())?;
        Ok(output_path.clone())
    }

    /// Export document to RTF format
    pub fn export_to_rtf(document: &Document, output_path: &PathBuf) -> Result<PathBuf, String> {
        let content = document.content.as_ref().unwrap_or(&String::new());
        
        // Basic RTF structure
        let rtf = format!(
            r#"{{\rtf1\ansi\deff0
{{\fonttbl{{\f0\fswiss\fprq2\fcharset0 Arial;}}}}
{{\*\generator Novus Writer;}}
\viewkind4\uc1\pard\f0\fs24
\b {}\b0\par
\par
{}\par
}}"#,
            document.title.replace('\\', "\\\\").replace('{', "\\{").replace('}', "\\}"),
            Self::escape_rtf(content)
        );

        Self::write_file(output_path, rtf.as_bytes())?;
        Ok(output_path.clone())
    }

    /// Export document to ODT format
    /// Note: Full ODT generation requires a library or backend service
    pub fn export_to_odt(document: &Document, output_path: &PathBuf) -> Result<PathBuf, String> {
        // Placeholder implementation
        // In production, this would create a proper ODT package
        let content = format!(
            "[ODT Document]\nTitle: {}\nContent: {}",
            document.title,
            document.content.as_ref().unwrap_or(&String::new())
        );
        
        Self::write_file(output_path, content.as_bytes())?;
        Ok(output_path.clone())
    }

    /// Helper function to write bytes to a file
    fn write_file(path: &PathBuf, content: &[u8]) -> Result<(), String> {
        let mut file = fs::File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;
        
        file.write_all(content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }

    /// Convert content to HTML (basic conversion)
    fn content_to_html(content: &str) -> String {
        // Basic HTML escaping and paragraph wrapping
        content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\n', "</p><p>")
            .replace("\r", "")
    }

    /// Escape special characters for RTF
    fn escape_rtf(content: &str) -> String {
        content
            .replace('\\', "\\\\")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('\n', "\\par\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    fn create_test_document() -> Document {
        Document {
            id: "test-1".to_string(),
            title: "Test Document".to_string(),
            content: Some("This is test content.".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            word_count: 4,
            character_count: 21,
        }
    }

    #[test]
    fn test_export_format_from_extension() {
        assert_eq!(ExportFormat::from_extension("pdf"), Some(ExportFormat::Pdf));
        assert_eq!(ExportFormat::from_extension("PDF"), Some(ExportFormat::Pdf));
        assert_eq!(ExportFormat::from_extension("docx"), Some(ExportFormat::Docx));
        assert_eq!(ExportFormat::from_extension("html"), Some(ExportFormat::Html));
        assert_eq!(ExportFormat::from_extension("md"), Some(ExportFormat::Markdown));
        assert_eq!(ExportFormat::from_extension("txt"), Some(ExportFormat::PlainText));
        assert_eq!(ExportFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_export_format_mime_types() {
        assert_eq!(ExportFormat::Pdf.get_mime_type(), "application/pdf");
        assert_eq!(ExportFormat::Html.get_mime_type(), "text/html");
        assert_eq!(ExportFormat::PlainText.get_mime_type(), "text/plain");
    }

    #[test]
    fn test_export_to_plain_text() {
        let doc = create_test_document();
        let temp_path = PathBuf::from("/tmp/test_export.txt");
        
        // Note: This test would need actual file system access
        // In CI, you'd want to use a temporary directory
    }
}
