# Novus Writer - Phase 5: Export & Import

## Overview

Phase 5 implements comprehensive document export and import capabilities, enabling users to save documents in multiple formats (PDF, DOCX, HTML, plain text) and import from external sources. This phase builds upon the completed editor integration (Phase 4) and database layer (Phase 3) to provide full document interoperability.

## Objectives

1. **Export Functionality**
   - PDF export with formatting preservation
   - DOCX (Microsoft Word) export
   - HTML export for web publishing
   - Plain text (.txt) export
   - Markdown (.md) export
   - Custom .notes format (native format)

2. **Import Functionality**
   - Import from DOCX files
   - Import from HTML files
   - Import from plain text files
   - Import from Markdown files
   - Import from .notes format

3. **Export Settings UI**
   - Page size selection (A4, Letter, Legal, etc.)
   - Orientation (Portrait/Landscape)
   - Margins configuration
   - Quality settings for images
   - Include/exclude metadata

4. **Batch Operations**
   - Bulk export multiple documents
   - Batch import from folder
   - Progress tracking for large operations

5. **Format Conversion Pipeline**
   - Extensible plugin architecture
   - Format validation
   - Error handling and recovery
   - Preview before export

## Architecture

### Export Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Initiates Export                     │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│                   Export Dialog UI                           │
│  - Format selection                                          │
│  - Settings configuration                                    │
│  - Preview                                                   │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│                 Tauri Command Layer                          │
│            (invoke('export_document', ...))                  │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│                Export Service (Rust)                         │
│  - Validates request                                         │
│  - Selects appropriate exporter                              │
│  - Manages conversion pipeline                               │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│              Format-Specific Exporter                        │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────┐  │
│  │   PDF    │   DOCX   │   HTML   │   TXT    │   MD     │  │
│  │ Exporter │ Exporter │ Exporter │ Exporter │ Exporter │  │
│  └──────────┴──────────┴──────────┴──────────┴──────────┘  │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│                 File System Write                            │
│              (via tauri-plugin-fs)                           │
└─────────────────────────────────────────────────────────────┘
```

### Import Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   User Initiates Import                      │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│                  File Picker Dialog                          │
│           (via tauri-plugin-dialog)                          │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│               Format Detection                               │
│  - Check file extension                                      │
│  - Validate file signature (magic bytes)                     │
│  - Determine appropriate importer                            │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│              Format-Specific Importer                        │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────┐  │
│  │   PDF    │   DOCX   │   HTML   │   TXT    │   MD     │  │
│  │ Importer │ Importer │ Importer │ Importer │ Importer │  │
│  └──────────┴──────────┴──────────┴──────────┴──────────┘  │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│             Content Normalization                            │
│  - Convert to Lexical editor state                           │
│  - Extract and embed assets                                  │
│  - Preserve formatting where possible                        │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│              Preview & Confirmation                          │
└─────────────────────┬───────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────────────────┐
│               Save to Database                               │
│            (via Phase 3 commands)                            │
└─────────────────────────────────────────────────────────────┘
```

### Plugin Architecture for Exporters/Importers

```rust
/// Trait for export plugins
pub trait ExportPlugin: Send + Sync {
    /// Returns the unique identifier for this exporter
    fn id(&self) -> &str;
    
    /// Returns human-readable name
    fn name(&self) -> &str;
    
    /// Returns supported file extensions
    fn supported_extensions(&self) -> Vec<&str>;
    
    /// Returns MIME type
    fn mime_type(&self) -> &str;
    
    /// Export the document to the specified format
    fn export(&self, document: &Document, options: ExportOptions) -> Result<Vec<u8>>;
    
    /// Returns whether this exporter supports batch operations
    fn supports_batch(&self) -> bool { true }
}

/// Trait for import plugins
pub trait ImportPlugin: Send + Sync {
    /// Returns the unique identifier for this importer
    fn id(&self) -> &str;
    
    /// Returns human-readable name
    fn name(&self) -> &str;
    
    /// Returns supported file extensions
    fn supported_extensions(&self) -> Vec<&str>;
    
    /// Detect if this importer can handle the given file
    fn can_import(&self, data: &[u8], extension: &str) -> bool;
    
    /// Import the file and convert to Document
    fn import(&self, data: &[u8], options: ImportOptions) -> Result<Document>;
}
```

## Implementation Details

### 1. Export Service Implementation

```rust
// src-tauri/src/services/export_service.rs

use crate::models::document::Document;
use crate::core::result::Result;
use std::sync::Arc;

pub struct ExportService {
    exporters: HashMap<String, Arc<dyn ExportPlugin>>,
}

impl ExportService {
    pub fn new() -> Self {
        let mut service = Self {
            exporters: HashMap::new(),
        };
        
        // Register built-in exporters
        service.register(Box::new(PdfExporter::new()));
        service.register(Box::new(DocxExporter::new()));
        service.register(Box::new(HtmlExporter::new()));
        service.register(Box::new(TextExporter::new()));
        service.register(Box::new(MarkdownExporter::new()));
        service.register(Box::new(NotesExporter::new()));
        
        service
    }
    
    pub fn register(&mut self, exporter: Box<dyn ExportPlugin>) {
        self.exporters.insert(exporter.id().to_string(), exporter);
    }
    
    pub async fn export(
        &self,
        document: &Document,
        format: &str,
        options: ExportOptions,
        output_path: &str,
    ) -> Result<()> {
        let exporter = self.exporters
            .get(format)
            .ok_or_else(|| ExportError::UnsupportedFormat(format.to_string()))?;
        
        let data = exporter.export(document, options)?;
        
        // Write to file using Tauri FS API
        use tauri_plugin_fs::FsExt;
        // Implementation...
        
        Ok(())
    }
    
    pub fn get_supported_formats(&self) -> Vec<ExportFormat> {
        self.exporters
            .values()
            .map(|e| ExportFormat {
                id: e.id().to_string(),
                name: e.name().to_string(),
                extensions: e.supported_extensions(),
                mime_type: e.mime_type(),
            })
            .collect()
    }
}
```

### 2. PDF Exporter Implementation

```rust
// src-tauri/src/plugins/exporters/pdf_exporter.rs

use printpdf::{PdfDocument, PdfLayerIndex, Point, Mm};
use crate::models::document::Document;

pub struct PdfExporter {
    // Configuration
    default_page_size: PageSize,
    image_quality: u8,
}

impl PdfExporter {
    pub fn new() -> Self {
        Self {
            default_page_size: PageSize::A4,
            image_quality: 90,
        }
    }
}

impl ExportPlugin for PdfExporter {
    fn id(&self) -> &str { "pdf" }
    
    fn name(&self) -> &str { "PDF Document" }
    
    fn supported_extensions(&self) -> Vec<&str> {
        vec!["pdf"]
    }
    
    fn mime_type(&self) -> &str { "application/pdf" }
    
    fn export(&self, document: &Document, options: ExportOptions) -> Result<Vec<u8>> {
        // Create PDF document
        let (doc, page1, layer1) = PdfDocument::new(
            document.metadata.title.clone(),
            Mm::from(options.page_width),
            Mm::from(options.page_height),
            "Layer 1",
        );
        
        // Convert Lexical content to PDF elements
        self.render_content(&layer1, &document.content)?;
        
        // Add metadata
        doc.document_info(title = document.metadata.title.clone());
        doc.document_info(author = document.metadata.author.clone());
        
        // Serialize to bytes
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer)?;
        
        Ok(buffer)
    }
}

impl PdfExporter {
    fn render_content(&self, layer: &PdfLayerIndex, content: &LexicalContent) -> Result<()> {
        // Iterate through Lexical nodes and render to PDF
        for node in &content.nodes {
            match node {
                LexicalNode::Paragraph(p) => self.render_paragraph(layer, p)?,
                LexicalNode::Heading(h) => self.render_heading(layer, h)?,
                LexicalNode::Image(img) => self.render_image(layer, img)?,
                LexicalNode::Table(t) => self.render_table(layer, t)?,
                LexicalNode::List(l) => self.render_list(layer, l)?,
                // ... other node types
            }
        }
        Ok(())
    }
}
```

### 3. DOCX Exporter Implementation

```rust
// src-tauri/src/plugins/exporters/docx_exporter.rs

use docx_rs::{Docx, Paragraph, Run, Text};
use crate::models::document::Document;

pub struct DocxExporter;

impl DocxExporter {
    pub fn new() -> Self { Self }
}

impl ExportPlugin for DocxExporter {
    fn id(&self) -> &str { "docx" }
    
    fn name(&self) -> &str { "Microsoft Word Document" }
    
    fn supported_extensions(&self) -> Vec<&str> {
        vec!["docx"]
    }
    
    fn mime_type(&self) -> &str { 
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" 
    }
    
    fn export(&self, document: &Document, options: ExportOptions) -> Result<Vec<u8>> {
        let mut docx = Docx::new();
        
        // Add document properties
        docx = docx.document_property(
            document.metadata.title.clone(),
            document.metadata.author.clone(),
        );
        
        // Convert Lexical content to DOCX paragraphs
        let paragraphs = self.convert_to_paragraphs(&document.content)?;
        
        for para in paragraphs {
            docx = docx.add_paragraph(para);
        }
        
        // Build and return bytes
        let mut buffer = Vec::new();
        docx.build().write(&mut buffer, None)?;
        
        Ok(buffer)
    }
}
```

### 4. DOCX Importer Implementation

```rust
// src-tauri/src/plugins/importers/docx_importer.rs

use docx_rs::read_docx;
use crate::models::document::Document;

pub struct DocxImporter;

impl ImportPlugin for DocxImporter {
    fn id(&self) -> &str { "docx_import" }
    
    fn name(&self) -> &str { "Microsoft Word Document Importer" }
    
    fn supported_extensions(&self) -> Vec<&str> {
        vec!["docx"]
    }
    
    fn can_import(&self, data: &[u8], extension: &str) -> bool {
        extension == "docx" && data.starts_with(&[0x50, 0x4B]) // PK zip signature
    }
    
    fn import(&self, data: &[u8], options: ImportOptions) -> Result<Document> {
        // Parse DOCX file
        let docx = read_docx(data)?;
        
        // Extract content and convert to Lexical format
        let content = self.parse_docx_content(&docx)?;
        
        // Extract embedded images
        let assets = self.extract_assets(&docx)?;
        
        // Create new document
        let document = Document {
            metadata: DocumentMetadata {
                title: docx.document_property.title.unwrap_or("Imported Document".to_string()),
                author: docx.document_property.creator.unwrap_or_default(),
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
            content,
            assets,
            settings: Default::default(),
        };
        
        Ok(document)
    }
}
```

### 5. HTML Exporter/Importer

```rust
// src-tauri/src/plugins/exporters/html_exporter.rs

pub struct HtmlExporter;

impl ExportPlugin for HtmlExporter {
    fn id(&self) -> &str { "html" }
    
    fn name(&self) -> &str { "HTML Document" }
    
    fn supported_extensions(&self) -> Vec<&str> {
        vec!["html", "htm"]
    }
    
    fn mime_type(&self) -> &str { "text/html" }
    
    fn export(&self, document: &Document, options: ExportOptions) -> Result<Vec<u8>> {
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>{}</style>
</head>
<body>
    {}
</body>
</html>"#,
            document.metadata.title,
            self.generate_styles(),
            self.convert_to_html(&document.content)?
        );
        
        Ok(html.into_bytes())
    }
}
```

### 6. Export Options Structure

```rust
// src-tauri/src/models/export.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// Output format (pdf, docx, html, txt, md)
    pub format: String,
    
    /// Page size (A4, Letter, Legal, etc.)
    pub page_size: PageSize,
    
    /// Page orientation
    pub orientation: Orientation,
    
    /// Margins in millimeters
    pub margins: Margins,
    
    /// Image quality (0-100)
    pub image_quality: u8,
    
    /// Include document metadata
    pub include_metadata: bool,
    
    /// Embed fonts
    pub embed_fonts: bool,
    
    /// Password protection (for PDF)
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    A3,
    Letter,
    Legal,
    Tabloid,
    Custom { width: f64, height: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: "pdf".to_string(),
            page_size: PageSize::A4,
            orientation: Orientation::Portrait,
            margins: Margins {
                top: 25.4,
                bottom: 25.4,
                left: 25.4,
                right: 25.4,
            },
            image_quality: 90,
            include_metadata: true,
            embed_fonts: false,
            password: None,
        }
    }
}
```

### 7. Tauri Commands

```rust
// src-tauri/src/commands/export.rs

use tauri::State;
use crate::services::export_service::ExportService;
use crate::models::export::ExportOptions;

#[tauri::command]
pub async fn export_document(
    document_id: String,
    format: String,
    options: ExportOptions,
    output_path: String,
    state: State<'_, ExportService>,
) -> Result<(), String> {
    // Get document from database
    let document = get_document_by_id(&document_id)
        .await
        .map_err(|e| e.to_string())?;
    
    // Perform export
    state
        .export(&document, &format, options, &output_path)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_export_formats(
    state: State<'_, ExportService>,
) -> Result<Vec<ExportFormat>, String> {
    Ok(state.get_supported_formats())
}

#[tauri::command]
pub async fn preview_export(
    document_id: String,
    format: String,
    options: ExportOptions,
    state: State<'_, ExportService>,
) -> Result<Vec<u8>, String> {
    let document = get_document_by_id(&document_id)
        .await
        .map_err(|e| e.to_string())?;
    
    // Generate preview (first page or thumbnail)
    state
        .preview(&document, &format, options)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_export(
    document_ids: Vec<String>,
    format: String,
    options: ExportOptions,
    output_directory: String,
    state: State<'_, ExportService>,
) -> Result<BatchExportResult, String> {
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for doc_id in document_ids {
        match state.export_by_id(&doc_id, &format, options.clone(), &output_directory).await {
            Ok(path) => results.push(ExportResult { document_id: doc_id, success: true, path: Some(path) }),
            Err(e) => errors.push(ExportResult { document_id: doc_id, success: false, error: Some(e.to_string()), ..Default::default() }),
        }
    }
    
    Ok(BatchExportResult {
        total: document_ids.len(),
        successful: results.len(),
        failed: errors.len(),
        results,
    })
}
```

```rust
// src-tauri/src/commands/import.rs

#[tauri::command]
pub async fn import_document(
    file_path: String,
    options: ImportOptions,
    state: State<'_, ImportService>,
) -> Result<Document, String> {
    // Read file
    let data = tokio::fs::read(&file_path)
        .await
        .map_err(|e| e.to_string())?;
    
    // Detect format and import
    let document = state
        .import(&data, &file_path, options)
        .await
        .map_err(|e| e.to_string())?;
    
    // Save to database
    save_document(&document)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(document)
}

#[tauri::command]
pub async fn detect_file_format(
    file_path: String,
    state: State<'_, ImportService>,
) -> Result<Option<ImportFormat>, String> {
    let data = tokio::fs::read(&file_path)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(state.detect_format(&data, &file_path))
}

#[tauri::command]
pub async fn get_import_formats(
    state: State<'_, ImportService>,
) -> Result<Vec<ImportFormat>, String> {
    Ok(state.get_supported_formats())
}

#[tauri::command]
pub async fn batch_import(
    file_paths: Vec<String>,
    options: ImportOptions,
    state: State<'_, ImportService>,
) -> Result<BatchImportResult, String> {
    let mut results = Vec::new();
    
    for path in file_paths {
        match state.import_from_path(&path, options.clone()).await {
            Ok(doc) => results.push(ImportResult { path, success: true, document_id: Some(doc.id.clone()) }),
            Err(e) => results.push(ImportResult { path, success: false, error: Some(e.to_string()), ..Default::default() }),
        }
    }
    
    Ok(BatchImportResult {
        total: file_paths.len(),
        successful: results.iter().filter(|r| r.success).count(),
        failed: results.iter().filter(|r| !r.success).count(),
        results,
    })
}
```

### 8. Frontend Export Dialog

```typescript
// frontend/src/components/dialogs/ExportDialog.tsx

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

interface ExportDialogProps {
  documentId: string;
  onClose: () => void;
}

export function ExportDialog({ documentId, onClose }: ExportDialogProps) {
  const [format, setFormat] = useState('pdf');
  const [options, setOptions] = useState<ExportOptions>({
    pageSize: 'A4',
    orientation: 'portrait',
    margins: { top: 25.4, bottom: 25.4, left: 25.4, right: 25.4 },
    imageQuality: 90,
    includeMetadata: true,
  });
  const [isExporting, setIsExporting] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleExport = async () => {
    setIsExporting(true);
    
    try {
      // Open save dialog
      const outputPath = await open({
        directory: false,
        multiple: false,
        filters: [{
          name: format.toUpperCase(),
          extensions: [format],
        }],
      });
      
      if (!outputPath) return;
      
      // Perform export
      await invoke('export_document', {
        documentId,
        format,
        options,
        outputPath,
      });
      
      onClose();
    } catch (error) {
      console.error('Export failed:', error);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="export-dialog">
      <h2>Export Document</h2>
      
      <div className="format-selection">
        <label>Format:</label>
        <select value={format} onChange={(e) => setFormat(e.target.value)}>
          <option value="pdf">PDF Document</option>
          <option value="docx">Microsoft Word (.docx)</option>
          <option value="html">HTML Document</option>
          <option value="txt">Plain Text</option>
          <option value="md">Markdown</option>
          <option value="notes">Novus Notes (.notes)</option>
        </select>
      </div>
      
      <div className="export-options">
        {format === 'pdf' && (
          <>
            <PageSizeSelector value={options.pageSize} onChange={(v) => setOptions({...options, pageSize: v})} />
            <OrientationSelector value={options.orientation} onChange={(v) => setOptions({...options, orientation: v})} />
            <MarginsConfig value={options.margins} onChange={(v) => setOptions({...options, margins: v})} />
            <ImageQualitySlider value={options.imageQuality} onChange={(v) => setOptions({...options, imageQuality: v})} />
          </>
        )}
      </div>
      
      <div className="dialog-actions">
        <button onClick={onClose} disabled={isExporting}>Cancel</button>
        <button onClick={handleExport} disabled={isExporting}>
          {isExporting ? 'Exporting...' : 'Export'}
        </button>
      </div>
      
      {isExporting && (
        <div className="progress-bar">
          <div className="progress" style={{ width: `${progress}%` }} />
        </div>
      )}
    </div>
  );
}
```

### 9. Frontend Import Dialog

```typescript
// frontend/src/components/dialogs/ImportDialog.tsx

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

interface ImportDialogProps {
  onImportComplete: (documentId: string) => void;
  onClose: () => void;
}

export function ImportDialog({ onImportComplete, onClose }: ImportDialogProps) {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [detectedFormat, setDetectedFormat] = useState<string | null>(null);
  const [isImporting, setIsImporting] = useState(false);

  const handleFileSelect = async () => {
    const filePath = await open({
      directory: false,
      multiple: false,
      filters: [
        { name: 'All Supported', extensions: ['docx', 'html', 'htm', 'txt', 'md', 'notes'] },
        { name: 'Word Document', extensions: ['docx'] },
        { name: 'HTML Document', extensions: ['html', 'htm'] },
        { name: 'Text File', extensions: ['txt'] },
        { name: 'Markdown', extensions: ['md'] },
        { name: 'Novus Notes', extensions: ['notes'] },
      ],
    });
    
    if (filePath) {
      setSelectedFile(filePath);
      
      // Detect format
      const format = await invoke('detect_file_format', { filePath });
      setDetectedFormat(format);
    }
  };

  const handleImport = async () => {
    if (!selectedFile) return;
    
    setIsImporting(true);
    
    try {
      const document = await invoke('import_document', {
        filePath: selectedFile,
        options: {},
      });
      
      onImportComplete(document.id);
      onClose();
    } catch (error) {
      console.error('Import failed:', error);
    } finally {
      setIsImporting(false);
    }
  };

  return (
    <div className="import-dialog">
      <h2>Import Document</h2>
      
      <div className="file-selection">
        <button onClick={handleFileSelect}>Choose File</button>
        {selectedFile && (
          <div className="selected-file">
            <span>{selectedFile}</span>
            {detectedFormat && (
              <span className="format-badge">{detectedFormat.toUpperCase()}</span>
            )}
          </div>
        )}
      </div>
      
      <div className="dialog-actions">
        <button onClick={onClose} disabled={isImporting}>Cancel</button>
        <button 
          onClick={handleImport} 
          disabled={!selectedFile || isImporting}
        >
          {isImporting ? 'Importing...' : 'Import'}
        </button>
      </div>
    </div>
  );
}
```

## File Structure

```
novus-writer/
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── dialogs/
│   │   │   │   ├── ExportDialog.tsx
│   │   │   │   ├── ImportDialog.tsx
│   │   │   │   ├── BatchExportDialog.tsx
│   │   │   │   └── ExportPreview.tsx
│   │   │   └── export/
│   │   │       ├── PageSizeSelector.tsx
│   │   │       ├── OrientationSelector.tsx
│   │   │       ├── MarginsConfig.tsx
│   │   │       └── ImageQualitySlider.tsx
│   │   ├── hooks/
│   │   │   └── useExport.ts
│   │   └── types/
│   │       └── export.ts
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   │   ├── export.rs
│   │   │   └── import.rs
│   │   ├── services/
│   │   │   ├── export_service.rs
│   │   │   └── import_service.rs
│   │   ├── plugins/
│   │   │   ├── exporters/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── pdf_exporter.rs
│   │   │   │   ├── docx_exporter.rs
│   │   │   │   ├── html_exporter.rs
│   │   │   │   ├── text_exporter.rs
│   │   │   │   ├── markdown_exporter.rs
│   │   │   │   └── notes_exporter.rs
│   │   │   └── importers/
│   │   │       ├── mod.rs
│   │   │       ├── docx_importer.rs
│   │   │       ├── html_importer.rs
│   │   │       ├── text_importer.rs
│   │   │       ├── markdown_importer.rs
│   │   │       └── notes_importer.rs
│   │   ├── models/
│   │   │   └── export.rs
│   │   └── utils/
│   │       ├── conversion.rs
│   │       └── format_detection.rs
│   └── Cargo.toml (updated dependencies)
```

## Testing Strategy

### Unit Tests

```rust
// src-tauri/src/plugins/exporters/pdf_exporter_test.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pdf_export_basic() {
        let exporter = PdfExporter::new();
        let document = create_test_document();
        let options = ExportOptions::default();
        
        let result = exporter.export(&document, options);
        
        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }
    
    #[test]
    fn test_pdf_export_with_images() {
        // Test image embedding in PDF
    }
    
    #[test]
    fn test_docx_roundtrip() {
        // Export to DOCX and import back
        // Verify content matches
    }
}
```

### Integration Tests

```rust
// src-tauri/tests/integration/export_import.rs

#[tokio::test]
async fn test_export_import_cycle() {
    let document = create_test_document();
    
    // Export to DOCX
    let exported = export_service.export(&document, "docx", options).await.unwrap();
    
    // Import back
    let imported = import_service.import(&exported, "docx").await.unwrap();
    
    // Verify content matches
    assert_eq!(document.content, imported.content);
}

#[tokio::test]
async fn test_batch_export() {
    let documents = vec![/* multiple documents */];
    let results = export_service.batch_export(documents, "pdf", output_dir).await;
    
    assert_eq!(results.successful, documents.len());
}
```

### E2E Tests

```typescript
// frontend/tests/e2e/export-import.spec.ts

test('exports document to PDF', async ({ page }) => {
  // Create document
  await page.fill('[data-testid="editor"]', 'Test content');
  
  // Open export dialog
  await page.click('[data-testid="export-button"]');
  
  // Select PDF format
  await page.selectOption('select[name="format"]', 'pdf');
  
  // Configure options
  await page.selectOption('select[name="pageSize"]', 'A4');
  
  // Export
  await page.click('button:has-text("Export")');
  
  // Verify file created
  // ...
});

test('imports DOCX file', async ({ page }) => {
  // Open import dialog
  await page.click('[data-testid="import-button"]');
  
  // Select file
  await page.setInputFiles('input[type="file"]', 'test.docx');
  
  // Import
  await page.click('button:has-text("Import")');
  
  // Verify content loaded
  const content = await page.textContent('[data-testid="editor"]');
  expect(content).toContain('Expected content');
});
```

## Performance Considerations

1. **Streaming Export**: For large documents, stream output instead of buffering entirely in memory
2. **Async Processing**: Use tokio async runtime for non-blocking I/O
3. **Progress Reporting**: Emit progress events for long-running exports
4. **Image Optimization**: Downscale images during export based on target format
5. **Parallel Batch Operations**: Process multiple documents concurrently
6. **Memory Management**: Clean up temporary files after export completes

## Security

1. **Path Traversal Prevention**: Validate and sanitize output paths
2. **File Size Limits**: Enforce maximum file sizes for import
3. **Content Validation**: Validate imported content for malicious payloads
4. **Sandbox Execution**: Run format converters in isolated contexts where possible
5. **Password Protection**: Support encryption for sensitive exports

## Success Criteria

- [ ] PDF export produces valid, formatted documents
- [ ] DOCX export preserves formatting, styles, and images
- [ ] HTML export generates clean, semantic HTML
- [ ] Plain text and Markdown exports work correctly
- [ ] DOCX import converts Word documents accurately
- [ ] HTML import parses and renders web content
- [ ] Markdown import converts to rich text
- [ ] Export dialog provides all necessary options
- [ ] Import dialog detects file formats automatically
- [ ] Batch export handles multiple documents
- [ ] Progress indicators show export/import status
- [ ] Error messages are clear and actionable
- [ ] Round-trip export/import preserves content

## Timeline

- **Week 1-2**: PDF exporter implementation
- **Week 3-4**: DOCX exporter and importer
- **Week 5**: HTML, Text, Markdown exporters/importers
- **Week 6**: Export/Import UI components
- **Week 7**: Batch operations and progress tracking
- **Week 8**: Testing, optimization, bug fixes

## Dependencies

### Rust Crates (add to Cargo.toml)

```toml
# PDF generation
printpdf = "0.6"

# DOCX handling
docx-rs = "0.4"

# HTML parsing
html5ever = "0.26"
markup5ever = "0.11"

# Markdown parsing
pulldown-cmark = "0.9"

# Image processing (for PDF/DOCX)
image = "0.24"

# Font handling
font-kit = "0.11"
```

### Frontend (already available)

- No additional NPM packages required
- Existing Tauri dialog and FS plugins sufficient

## Next Steps (Phase 6)

After completing Phase 5:
- Track changes and revision history
- Comments and annotations
- Collaboration features (multi-user editing)
- Templates system
- Advanced search and replace
- Table of contents generation
- Index creation

---

*This document outlines the Phase 5 implementation plan for Export & Import functionality. All code should follow the architecture principles defined in ARCHITECTURE.md and integrate seamlessly with Phases 3 and 4.*
