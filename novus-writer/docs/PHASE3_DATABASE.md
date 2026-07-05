# Novus Writer - Database Implementation

## Phase 3 Complete: Database Layer

This document describes the completed database implementation for Novus Writer.

### Overview

The database layer provides persistent storage for documents, versions, autosaves, bookmarks, and backups using SQLite with SQLx for type-safe async queries.

### Key Features Implemented

#### 1. Database Schema (`schema.sql`)

Complete SQLite schema with:
- **documents**: Main document storage with compressed content
- **document_versions**: Full version history tracking
- **autosaves**: Crash recovery snapshots
- **bookmarks**: Document navigation points
- **backups**: Automatic backup tracking
- **schema_migrations**: Migration version tracking

**Optimizations:**
- WAL mode enabled for concurrent access
- Foreign key constraints enforced
- 12 performance indexes on frequently queried columns
- Automatic trigger for `modified_at` timestamp updates
- Views for common queries (recent_documents, document_stats, latest_versions)

#### 2. Database Module (`src/database/mod.rs`)

**DatabaseConfig:**
```rust
pub struct DatabaseConfig {
    pub max_connections: u32,      // Default: 10
    pub min_connections: u32,      // Default: 2
    pub acquire_timeout: Duration, // Default: 30s
    pub idle_timeout: Duration,    // Default: 600s
}
```

**Key Methods:**
- `new()` - In-memory database for testing
- `with_path(path)` - File-based database
- `with_path_and_config(path, config)` - Custom configuration
- `init()` - Initialize schema with all tables and indexes
- `health_check()` - Verify database connectivity
- `close()` - Graceful connection pool shutdown

#### 3. Model Types

Created dedicated model files in `src/models/`:

**version.rs:**
```rust
pub struct DocumentVersion {
    pub id: i64,
    pub document_id: String,
    pub content: String,           // Decompressed
    pub version_number: u32,
    pub created_at: DateTime<Utc>,
    pub change_summary: Option<String>,
}
```

**autosave.rs:**
```rust
pub struct Autosave {
    pub id: i64,
    pub document_id: String,
    pub content: String,           // Decompressed
    pub created_at: DateTime<Utc>,
}
```

**bookmark.rs:**
```rust
pub struct Bookmark {
    pub id: i64,
    pub document_id: String,
    pub name: String,
    pub position: i64,             // Character offset
    pub created_at: DateTime<Utc>,
}
```

**backup.rs:**
```rust
pub struct Backup {
    pub id: i64,
    pub document_id: String,
    pub backup_path: String,       // Filesystem path
    pub created_at: DateTime<Utc>,
}
```

#### 4. Document Repository (`src/core/infrastructure/repository/document_repository.rs`)

Complete CRUD operations with compression:

**Document Operations:**
- `save(doc)` - Insert or update with UPSERT
- `find_by_id(id)` - Fetch with decompression
- `find_all()` - List all documents
- `delete(id)` - Soft delete with cascade
- `find_recent(limit)` - Recently accessed documents
- `search(query)` - Full-text search by title/tags

**Version Operations:**
- `create_version(doc_id, content, version_num, summary)` - Create snapshot
- `get_versions(doc_id)` - Get full history
- `restore_version(version_id)` - Restore and update document

**Autosave Operations:**
- `save_autosave(doc_id, content)` - Upsert autosave
- `get_autosave(doc_id)` - Retrieve for recovery
- `delete_autosave(doc_id)` - Clear after recovery

**Bookmark Operations:**
- `add_bookmark(doc_id, name, position)` - Create bookmark
- `get_bookmarks(doc_id)` - List sorted by position
- `remove_bookmark(id)` - Delete bookmark

**Backup Operations:**
- `create_backup(doc_id, path, size)` - Track backup file
- `get_backups(doc_id)` - List backups descending
- `search(query)` - Search documents

#### 5. Tauri Commands

Created command modules for each feature:

**commands/version.rs:**
- `create_version` - Create new version snapshot
- `get_version_history` - List all versions
- `restore_version` - Restore and update document
- `delete_version` - Remove specific version

**commands/autosave.rs:**
- `save_autosave` - Save autosave snapshot
- `get_autosave` - Retrieve autosave
- `delete_autosave` - Clear autosave
- `recover_autosave` - Recovery with optional cleanup
- `cleanup_old_autosaves` - Keep only recent N per document

**commands/bookmark.rs:**
- `add_bookmark` - Create bookmark
- `get_bookmarks` - List bookmarks
- `remove_bookmark` - Delete bookmark
- `update_bookmark_position` - Move bookmark
- `rename_bookmark` - Update name

**commands/backup.rs:**
- `create_backup` - Record backup file
- `get_backups` - List backups
- `delete_backup` - Remove record (optionally file)
- `restore_from_backup` - Read backup content
- `cleanup_old_backups` - Retention policy
- `get_backup_stats` - Statistics (count, size, date range)

### Design Decisions

#### 1. Compression Strategy
- All document content is gzip-compressed before storage
- Reduces storage by ~60-80% for typical documents
- Decompression happens on read in repository layer
- Transparent to command handlers and frontend

#### 2. Connection Pooling
- SQLx connection pool prevents connection overhead
- Configurable min/max connections
- Timeout handling for long-running queries
- Proper cleanup on application shutdown

#### 3. WAL Mode
- Write-Ahead Logging enables concurrent reads during writes
- Critical for responsive UI during autosave operations
- Better crash recovery than default journal mode

#### 4. Index Strategy
- 12 indexes covering all common query patterns
- Composite indexes for sorting (modified_at DESC, pinned DESC)
- Separate indexes for foreign key lookups
- Maintained automatically by SQLite

#### 5. Transaction Usage
- Used for multi-table operations
- Automatic rollback on error
- Ensures data consistency

#### 6. Error Handling
- All errors mapped to AppError enum
- Proper error propagation with ? operator
- Informative error messages for debugging

### Testing

Comprehensive unit tests included:

**Database Tests:**
- Initialization verification
- Health check validation
- Index creation confirmation

**Repository Tests:**
- Save and retrieve document
- Delete document
- Version creation and retrieval
- Autosave save and recovery
- Bookmark CRUD operations

### Performance Considerations

1. **Lazy Loading**: Content decompressed only when needed
2. **Connection Pooling**: Reuse connections across requests
3. **Indexed Queries**: All frequent queries use indexes
4. **Batch Operations**: Multiple inserts use single transaction
5. **Compression**: Reduced I/O for large documents

### Security

- All data stored locally (no cloud sync)
- No telemetry or analytics
- Parameterized queries prevent SQL injection
- File paths validated before access

### Next Steps (Phase 4)

With the database layer complete, the next phase will implement:
- Lexical editor integration
- Custom nodes (Image, Table, HorizontalRule)
- Editor plugins (AutoSave, KeyboardShortcuts, History)
- Clipboard operations
- Undo/redo system
- Spell checking integration

### File Structure

```
src-tauri/
├── schema.sql                          # Complete SQL schema
├── src/
│   ├── database/
│   │   └── mod.rs                      # Database service
│   ├── models/
│   │   ├── mod.rs                      # Model exports
│   │   ├── document.rs                 # Document DTO
│   │   ├── error.rs                    # Error types
│   │   ├── version.rs                  # Version model
│   │   ├── autosave.rs                 # Autosave model
│   │   ├── bookmark.rs                 # Bookmark model
│   │   └── backup.rs                   # Backup model
│   ├── core/
│   │   └── infrastructure/
│   │       └── repository/
│   │           ├── mod.rs              # Repository trait
│   │           └── document_repository.rs  # Implementation
│   └── commands/
│       ├── mod.rs                      # Command exports
│       ├── document.rs                 # Document commands
│       ├── version.rs                  # Version commands
│       ├── autosave.rs                 # Autosave commands
│       ├── bookmark.rs                 # Bookmark commands
│       └── backup.rs                   # Backup commands
```

### Usage Example

```rust
// Initialize database
let db = Database::with_path(&data_dir.join("novus.db")).await?;
db.init().await?;

// Create repository
let repo = SqliteDocumentRepository::new(db.pool().clone());

// Save document
let doc = Document::new("My Document");
repo.save(&doc).await?;

// Create version
repo.create_version(&doc.id, &doc.content, 1, Some("Initial"))
    .await?;

// Add bookmark
repo.add_bookmark(&doc.id, "Chapter 1", 100).await?;

// Autosave
repo.save_autosave(&doc.id, "Updated content").await?;
```

All code is production-ready with proper error handling, logging, and documentation.
