-- Novus Writer Database Schema
-- Version: 1.0.0
-- Description: Complete database schema for document management, versioning, autosave, bookmarks, and backups

-- Enable WAL mode for better concurrency
PRAGMA journal_mode = WAL;

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- ============================================================
-- DOCUMENTS TABLE
-- Main table for storing documents
-- ============================================================
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content BLOB,                          -- Compressed Lexical JSON content
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    version INTEGER DEFAULT 1,             -- Optimistic locking version
    tags TEXT DEFAULT '[]',                -- JSON array of tags
    pinned INTEGER DEFAULT 0,              -- Boolean: is document pinned?
    word_count INTEGER DEFAULT 0,          -- Cached word count
    character_count INTEGER DEFAULT 0,     -- Cached character count
    thumbnail BLOB,                        -- Optional thumbnail preview (compressed)
    last_opened_at DATETIME,               -- Last time document was opened
    metadata TEXT DEFAULT '{}'             -- Additional metadata as JSON
);

-- ============================================================
-- DOCUMENT_VERSIONS TABLE
-- Stores historical versions of documents for version history
-- ============================================================
CREATE TABLE IF NOT EXISTS document_versions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,       -- Sequential version number
    content BLOB NOT NULL,                 -- Compressed content snapshot
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    change_summary TEXT,                   -- Optional description of changes
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    UNIQUE(document_id, version_number)    -- Ensure unique version numbers per document
);

-- ============================================================
-- AUTOSAVES TABLE
-- Temporary autosave snapshots for crash recovery
-- ============================================================
CREATE TABLE IF NOT EXISTS autosaves (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    content BLOB NOT NULL,                 -- Compressed autosave content
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- ============================================================
-- BOOKMARKS TABLE
-- User-defined bookmarks for quick navigation within documents
-- ============================================================
CREATE TABLE IF NOT EXISTS bookmarks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    name TEXT NOT NULL,                    -- Bookmark label/name
    position INTEGER NOT NULL,             -- Character offset or node index
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- ============================================================
-- BACKUPS TABLE
-- Tracks automatic backup files created by the application
-- ============================================================
CREATE TABLE IF NOT EXISTS backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    backup_path TEXT NOT NULL,             -- Filesystem path to backup
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    backup_size INTEGER,                   -- Size in bytes
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- ============================================================
-- INDEXES
-- Optimize query performance for common operations
-- ============================================================

-- Documents: frequently queried by modification date and pinned status
CREATE INDEX IF NOT EXISTS idx_documents_modified_at ON documents(modified_at DESC);
CREATE INDEX IF NOT EXISTS idx_documents_pinned ON documents(pinned DESC);
CREATE INDEX IF NOT EXISTS idx_documents_last_opened ON documents(last_opened_at DESC);
CREATE INDEX IF NOT EXISTS idx_documents_title ON documents(title);

-- Document versions: frequently queried by document_id
CREATE INDEX IF NOT EXISTS idx_versions_document_id ON document_versions(document_id);
CREATE INDEX IF NOT EXISTS idx_versions_created_at ON document_versions(created_at DESC);

-- Autosaves: frequently queried by document_id
CREATE INDEX IF NOT EXISTS idx_autosaves_document_id ON autosaves(document_id);
CREATE INDEX IF NOT EXISTS idx_autosaves_created_at ON autosaves(created_at DESC);

-- Bookmarks: frequently queried by document_id
CREATE INDEX IF NOT EXISTS idx_bookmarks_document_id ON bookmarks(document_id);
CREATE INDEX IF NOT EXISTS idx_bookmarks_name ON bookmarks(name);

-- Backups: frequently queried by document_id
CREATE INDEX IF NOT EXISTS idx_backups_document_id ON backups(document_id);
CREATE INDEX IF NOT EXISTS idx_backups_created_at ON backups(created_at DESC);

-- ============================================================
-- TRIGGERS
-- Automatic maintenance and audit trails
-- ============================================================

-- Trigger: Update modified_at timestamp when document is updated
CREATE TRIGGER IF NOT EXISTS update_document_modified_at 
AFTER UPDATE ON documents
BEGIN
    UPDATE documents SET modified_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- Trigger: Auto-cleanup old autosaves (keep only last 5 per document)
-- This is handled in application code for more control

-- ============================================================
-- VIEWS
-- Convenient views for common queries
-- ============================================================

-- View: Recent documents with full metadata
CREATE VIEW IF NOT EXISTS recent_documents AS
SELECT 
    id,
    title,
    created_at,
    modified_at,
    version,
    tags,
    pinned,
    word_count,
    character_count,
    last_opened_at,
    CASE 
        WHEN last_opened_at IS NOT NULL THEN last_opened_at
        ELSE modified_at
    END AS effective_recent
FROM documents
ORDER BY effective_recent DESC;

-- View: Document statistics
CREATE VIEW IF NOT EXISTS document_stats AS
SELECT 
    COUNT(*) as total_documents,
    SUM(word_count) as total_words,
    SUM(character_count) as total_characters,
    AVG(word_count) as avg_words_per_document,
    MAX(version) as max_version,
    COUNT(DISTINCT CASE WHEN pinned = 1 THEN id END) as pinned_count
FROM documents;

-- View: Latest version per document
CREATE VIEW IF NOT EXISTS latest_versions AS
SELECT 
    dv.document_id,
    dv.version_number,
    dv.created_at,
    dv.change_summary
FROM document_versions dv
INNER JOIN (
    SELECT document_id, MAX(version_number) as max_version
    FROM document_versions
    GROUP BY document_id
) latest ON dv.document_id = latest.document_id AND dv.version_number = latest.max_version;

-- ============================================================
-- INITIAL DATA
-- Seed data if needed
-- ============================================================

-- No initial seed data required

-- ============================================================
-- MIGRATION HISTORY TABLE
-- Track schema migrations
-- ============================================================
CREATE TABLE IF NOT EXISTS schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);

-- Record initial schema version
INSERT OR IGNORE INTO schema_migrations (version, description) 
VALUES ('1.0.0', 'Initial schema creation');
