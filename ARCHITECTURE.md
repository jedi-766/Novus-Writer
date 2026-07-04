# WordPad Pro - Architecture Document

## Overview

WordPad Pro is a production-quality offline desktop notes application built with Rust, Tauri v2, and React. It provides a Microsoft Word-like editing experience while remaining lightweight and fully offline.

## Technology Stack

### Backend (Rust)
- **Tauri v2**: Desktop application framework
- **SQLx**: Async SQLite database access
- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **tracing**: Logging and observability
- **thiserror + anyhow**: Error handling

### Frontend (React + TypeScript)
- **React 18**: UI framework
- **TypeScript**: Type safety
- **TailwindCSS**: Styling
- **Lexical**: Rich text editor engine
- **Zustand**: State management
- **React Query**: Data fetching and caching

### Editor Engine
- **Lexical**: Facebook's extensible rich text editor
  - Better performance than Draft.js
  - More flexible than ProseMirror for our use case
  - Excellent plugin architecture
  - Strong TypeScript support

## Project Structure

```
wordpad-pro/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── test.yml
├── frontend/
│   ├── public/
│   │   ├── icons/
│   │   └── dictionaries/
│   ├── src/
│   │   ├── assets/
│   │   ├── components/
│   │   │   ├── common/
│   │   │   ├── ribbon/
│   │   │   ├── editor/
│   │   │   ├── sidebar/
│   │   │   ├── statusbar/
│   │   │   ├── dialogs/
│   │   │   └── layout/
│   │   ├── hooks/
│   │   ├── lib/
│   │   ├── plugins/
│   │   │   ├── lexical/
│   │   │   ├── spellcheck/
│   │   │   └── features/
│   │   ├── stores/
│   │   ├── styles/
│   │   ├── types/
│   │   ├── utils/
│   │   ├── App.tsx
│   │   ├── main.tsx
│   │   └── vite-env.d.ts
│   ├── index.html
│   ├── package.json
│   ├── tsconfig.json
│   ├── tailwind.config.js
│   ├── postcss.config.js
│   └── vite.config.ts
├── src-tauri/
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/
│   ├── migrations/
│   │   └── 001_initial.sql
│   ├── src/
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── document.rs
│   │   │   ├── editor.rs
│   │   │   ├── export.rs
│   │   │   ├── import.rs
│   │   │   ├── settings.rs
│   │   │   └── backup.rs
│   │   ├── core/
│   │   │   ├── mod.rs
│   │   │   ├── error.rs
│   │   │   └── result.rs
│   │   ├── database/
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   ├── models.rs
│   │   │   ├── repositories/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── document.rs
│   │   │   │   ├── version.rs
│   │   │   │   └── settings.rs
│   │   │   └── schema.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── document.rs
│   │   │   ├── version.rs
│   │   │   ├── settings.rs
│   │   │   └── asset.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── document_service.rs
│   │   │   ├── export_service.rs
│   │   │   ├── import_service.rs
│   │   │   ├── backup_service.rs
│   │   │   └── spellcheck_service.rs
│   │   ├── utils/
│   │   │   ├── mod.rs
│   │   │   ├── paths.rs
│   │   │   ├── file.rs
│   │   │   └── compression.rs
│   │   ├── plugins/
│   │   │   ├── mod.rs
│   │   │   └── plugin_manager.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── build.rs
│   ├── Cargo.toml
│   ├── rustfmt.toml
│   ├── tauri.conf.json
│   └── tests/
│       ├── integration/
│       └── fixtures/
├── .gitignore
├── .editorconfig
├── LICENSE
├── README.md
└── ARCHITECTURE.md
```

## Core Architecture Principles

### 1. Clean Architecture

The application follows Clean Architecture principles:

```
┌─────────────────────────────────────┐
│         Presentation Layer          │
│    (React Components, Tauri UI)     │
├─────────────────────────────────────┤
│         Application Layer           │
│    (Commands, Services, Use Cases)  │
├─────────────────────────────────────┤
│           Domain Layer              │
│        (Models, Entities, Rules)    │
├─────────────────────────────────────┤
│        Infrastructure Layer         │
│   (Database, File System, Plugins)  │
└─────────────────────────────────────┘
```

### 2. State Management

#### Frontend State (Zustand)
- **DocumentStore**: Active document, tabs, recent documents
- **EditorStore**: Editor state, selection, formatting
- **UIS tore**: Ribbon state, sidebar, theme, zoom
- **SettingsStore**: User preferences

#### Backend State
- **AppState**: Global application state
- **DbState**: Database connection pool
- **DocumentState**: Per-document state

### 3. Data Flow

```
User Action → Tauri Command → Service → Repository → Database
                ↓
          Event Emitted
                ↓
        Frontend Listener → Store Update → UI Re-render
```

### 4. Document Format (.notes)

```json
{
  "version": "1.0",
  "metadata": {
    "id": "uuid",
    "title": "Document Title",
    "created_at": "ISO8601",
    "modified_at": "ISO8601",
    "author": "User"
  },
  "content": {
    "type": "lexical",
    "data": { /* Lexical editor state */ }
  },
  "assets": [
    {
      "id": "uuid",
      "type": "image",
      "mime": "image/png",
      "data": "base64"
    }
  ],
  "settings": {
    "page_size": "A4",
    "orientation": "portrait",
    "margins": { "top": 25.4, "bottom": 25.4, "left": 25.4, "right": 25.4 }
  }
}
```

## Database Schema

### Tables

#### documents
```sql
CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content BLOB NOT NULL,  -- Compressed JSON
    created_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL,
    last_opened_at INTEGER,
    word_count INTEGER DEFAULT 0,
    character_count INTEGER DEFAULT 0,
    thumbnail BLOB,
    is_pinned INTEGER DEFAULT 0,
    tags TEXT  -- JSON array
);
```

#### document_versions
```sql
CREATE TABLE document_versions (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    content BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    message TEXT,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
```

#### autosaves
```sql
CREATE TABLE autosaves (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    content BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
```

#### settings
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);
```

#### bookmarks
```sql
CREATE TABLE bookmarks (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    name TEXT NOT NULL,
    position TEXT NOT NULL,  -- JSON path in document
    created_at INTEGER NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
```

#### backups
```sql
CREATE TABLE backups (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    backup_path TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    size_bytes INTEGER NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
```

## Plugin Architecture

### Plugin Types

1. **Editor Plugins**: Extend Lexical editor functionality
2. **Export Plugins**: Add new export formats
3. **Import Plugins**: Add new import formats
4. **Service Plugins**: Background services (grammar check, AI)

### Plugin Interface (Rust)

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
}

pub trait ExportPlugin: Plugin {
    fn supported_formats(&self) -> Vec<&str>;
    fn export(&self, document: &Document, format: &str) -> Result<Vec<u8>>;
}

pub trait ImportPlugin: Plugin {
    fn supported_formats(&self) -> Vec<&str>;
    fn import(&self, data: &[u8], format: &str) -> Result<Document>;
}
```

## Performance Considerations

### 1. Large Document Handling
- Virtual scrolling for document list
- Lazy loading of document content
- Incremental rendering in editor
- Content chunking for documents > 1000 pages

### 2. Memory Optimization
- Asset streaming instead of full loading
- Image thumbnails cached separately
- Automatic garbage collection of unused assets
- Efficient undo/redo using delta compression

### 3. Database Optimization
- Connection pooling
- Prepared statements
- Indexed queries
- WAL mode for better concurrency

### 4. Editor Performance
- Lexical's node-based architecture
- Minimal re-renders using React.memo
- Web Workers for heavy operations
- Debounced auto-save

## Security Model

### Threat Model
- All data stored locally
- No network access required
- No telemetry
- No third-party dependencies with network access

### Data Protection
- Optional document encryption (future)
- Secure deletion on delete
- Backup integrity verification

## Accessibility

### WCAG 2.1 AA Compliance
- Keyboard navigation throughout
- Screen reader support
- High contrast themes
- Configurable font sizes
- Focus indicators
- ARIA labels

## Testing Strategy

### Unit Tests
- All services
- All repositories
- Utility functions
- Editor plugins

### Integration Tests
- Tauri commands
- Database operations
- Import/export pipelines
- Full document workflows

### E2E Tests
- Critical user journeys
- Regression testing
- Performance benchmarks

## Build & Deployment

### Development
```bash
# Frontend
npm run dev

# Backend
cargo watch -x run

# Tauri Dev
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```

### Distribution Formats
- .deb (Debian/Ubuntu)
- .rpm (Fedora/RHEL)
- AppImage (Universal Linux)
- Flatpak (Future)

## Future Enhancements

### Phase 2+ Features
- Advanced table editing
- Drawing canvas
- Equation editor
- Citation management
- Track changes
- Comments
- Templates
- Mail merge

### Plugin Ecosystem
- Grammar checking (LanguageTool)
- AI assistance (local LLM)
- PDF annotation
- OCR integration
- Cloud sync (optional)

## Dependencies Summary

### Rust Crates
```toml
tauri = "2.0"
tauri-plugin-shell = "2.0"
tauri-plugin-dialog = "2.0"
tauri-plugin-fs = "2.0"
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "1.0"
anyhow = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
flate2 = "1.0"  # Compression
image = "0.24"  # Image processing
printpdf = "0.6"  # PDF generation
docx-rs = "0.4"  # DOCX handling
```

### NPM Packages
```json
{
  "react": "^18.2.0",
  "react-dom": "^18.2.0",
  "lexical": "^0.12.0",
  "@lexical/rich-text": "^0.12.0",
  "@lexical/table": "^0.12.0",
  "@lexical/list": "^0.12.0",
  "@lexical/link": "^0.12.0",
  "@lexical/image": "^0.12.0",
  "zustand": "^4.4.0",
  "@tanstack/react-query": "^5.0.0",
  "tailwindcss": "^3.3.0",
  "clsx": "^2.0.0",
  "uuid": "^9.0.0"
}
```

---

*This architecture document serves as the foundation for WordPad Pro development. All subsequent phases should align with these architectural decisions.*
