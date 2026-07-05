/**
 * Tauri IPC Bridge - Provides type-safe communication between frontend and Rust backend
 * 
 * This module wraps Tauri's invoke API with proper error handling and type safety
 * for all document operations, editor commands, and system functions.
 */

import { invoke } from '@tauri-apps/api/core';
import type { Document, DocumentMetadata } from '../types/document';
import type { ExportFormat } from '../types/export';
import type { SearchResult } from '../types/search';

// Error types for better error handling
export class TauriError extends Error {
  constructor(
    public message: string,
    public code?: string,
    public details?: Record<string, unknown>
  ) {
    super(message);
    this.name = 'TauriError';
  }
}

/**
 * Document Operations
 */

export async function createDocument(
  title: string,
  folderId?: number
): Promise<Document> {
  try {
    const doc = await invoke<Document>('create_document', { title, folderId });
    return doc;
  } catch (error) {
    throw new TauriError(
      `Failed to create document: ${error}`,
      'CREATE_DOCUMENT_FAILED',
      { title, folderId }
    );
  }
}

export async function openDocument(documentId: number): Promise<Document> {
  try {
    const doc = await invoke<Document>('open_document', { documentId });
    return doc;
  } catch (error) {
    throw new TauriError(
      `Failed to open document: ${error}`,
      'OPEN_DOCUMENT_FAILED',
      { documentId }
    );
  }
}

export async function saveDocument(
  documentId: number,
  content: string,
  metadata?: Partial<DocumentMetadata>
): Promise<Document> {
  try {
    const doc = await invoke<Document>('save_document', { 
      documentId, 
      content,
      metadata: metadata ? JSON.stringify(metadata) : undefined
    });
    return doc;
  } catch (error) {
    throw new TauriError(
      `Failed to save document: ${error}`,
      'SAVE_DOCUMENT_FAILED',
      { documentId }
    );
  }
}

export async function deleteDocument(documentId: number): Promise<void> {
  try {
    await invoke('delete_document', { documentId });
  } catch (error) {
    throw new TauriError(
      `Failed to delete document: ${error}`,
      'DELETE_DOCUMENT_FAILED',
      { documentId }
    );
  }
}

export async function listDocuments(
  folderId?: number,
  includeDeleted?: boolean
): Promise<Document[]> {
  try {
    const docs = await invoke<Document[]>('list_documents', { 
      folderId,
      includeDeleted
    });
    return docs;
  } catch (error) {
    throw new TauriError(
      `Failed to list documents: ${error}`,
      'LIST_DOCUMENTS_FAILED'
    );
  }
}

export async function renameDocument(
  documentId: number,
  newTitle: string
): Promise<Document> {
  try {
    const doc = await invoke<Document>('rename_document', { 
      documentId, 
      newTitle 
    });
    return doc;
  } catch (error) {
    throw new TauriError(
      `Failed to rename document: ${error}`,
      'RENAME_DOCUMENT_FAILED',
      { documentId, newTitle }
    );
  }
}

/**
 * Editor Operations
 */

export async function insertText(
  documentId: number,
  text: string,
  position?: number
): Promise<void> {
  try {
    await invoke('insert_text', { documentId, text, position });
  } catch (error) {
    throw new TauriError(
      `Failed to insert text: ${error}`,
      'INSERT_TEXT_FAILED',
      { documentId }
    );
  }
}

export async function formatText(
  documentId: number,
  formatType: string,
  value?: string,
  startOffset?: number,
  endOffset?: number
): Promise<void> {
  try {
    await invoke('format_text', { 
      documentId, 
      formatType, 
      value,
      startOffset,
      endOffset
    });
  } catch (error) {
    throw new TauriError(
      `Failed to format text: ${error}`,
      'FORMAT_TEXT_FAILED',
      { documentId, formatType }
    );
  }
}

export async function insertImage(
  documentId: number,
  imagePath: string,
  position?: number
): Promise<void> {
  try {
    await invoke('insert_image', { documentId, imagePath, position });
  } catch (error) {
    throw new TauriError(
      `Failed to insert image: ${error}`,
      'INSERT_IMAGE_FAILED',
      { documentId, imagePath }
    );
  }
}

export async function insertTable(
  documentId: number,
  rows: number,
  cols: number,
  position?: number
): Promise<void> {
  try {
    await invoke('insert_table', { documentId, rows, cols, position });
  } catch (error) {
    throw new TauriError(
      `Failed to insert table: ${error}`,
      'INSERT_TABLE_FAILED',
      { documentId, rows, cols }
    );
  }
}

/**
 * Search Operations
 */

export async function findText(
  documentId: number,
  query: string,
  caseSensitive?: boolean,
  wholeWord?: boolean
): Promise<SearchResult[]> {
  try {
    const results = await invoke<SearchResult[]>('find_text', { 
      documentId, 
      query,
      caseSensitive,
      wholeWord
    });
    return results;
  } catch (error) {
    throw new TauriError(
      `Failed to find text: ${error}`,
      'FIND_TEXT_FAILED',
      { documentId, query }
    );
  }
}

export async function replaceText(
  documentId: number,
  searchQuery: string,
  replacement: string,
  replaceAll?: boolean
): Promise<number> {
  try {
    const count = await invoke<number>('replace_text', { 
      documentId, 
      searchQuery, 
      replacement,
      replaceAll
    });
    return count;
  } catch (error) {
    throw new TauriError(
      `Failed to replace text: ${error}`,
      'REPLACE_TEXT_FAILED',
      { documentId, searchQuery }
    );
  }
}

/**
 * Export Operations
 */

export async function exportDocument(
  documentId: number,
  format: ExportFormat,
  outputPath?: string
): Promise<string> {
  try {
    const path = await invoke<string>('export_document', { 
      documentId, 
      format,
      outputPath
    });
    return path;
  } catch (error) {
    throw new TauriError(
      `Failed to export document: ${error}`,
      'EXPORT_DOCUMENT_FAILED',
      { documentId, format }
    );
  }
}

export async function exportToPDF(
  documentId: number,
  outputPath: string
): Promise<string> {
  return exportDocument(documentId, 'pdf', outputPath);
}

export async function exportToDOCX(
  documentId: number,
  outputPath: string
): Promise<string> {
  return exportDocument(documentId, 'docx', outputPath);
}

export async function exportToMarkdown(
  documentId: number,
  outputPath: string
): Promise<string> {
  return exportDocument(documentId, 'markdown', outputPath);
}

export async function exportToHTML(
  documentId: number,
  outputPath: string
): Promise<string> {
  return exportDocument(documentId, 'html', outputPath);
}

/**
 * Import Operations
 */

export async function importDocument(
  filePath: string,
  targetFolderId?: number
): Promise<Document> {
  try {
    const doc = await invoke<Document>('import_document', { 
      filePath,
      targetFolderId
    });
    return doc;
  } catch (error) {
    throw new TauriError(
      `Failed to import document: ${error}`,
      'IMPORT_DOCUMENT_FAILED',
      { filePath }
    );
  }
}

/**
 * Version Control Operations
 */

export async function createVersion(
  documentId: number,
  versionName?: string
): Promise<number> {
  try {
    const versionId = await invoke<number>('create_version', { 
      documentId, 
      versionName 
    });
    return versionId;
  } catch (error) {
    throw new TauriError(
      `Failed to create version: ${error}`,
      'CREATE_VERSION_FAILED',
      { documentId }
    );
  }
}

export async function restoreVersion(
  versionId: number
): Promise<Document> {
  try {
    const doc = await invoke<Document>('restore_version', { versionId });
    return doc;
  } catch (error) {
    throw new TauriError(
      `Failed to restore version: ${error}`,
      'RESTORE_VERSION_FAILED',
      { versionId }
    );
  }
}

export async function listVersions(
  documentId: number
): Promise<Array<{ id: number; name: string; created_at: string; size: number }>> {
  try {
    const versions = await invoke<Array<{ id: number; name: string; created_at: string; size: number }>>(
      'list_versions', 
      { documentId }
    );
    return versions;
  } catch (error) {
    throw new TauriError(
      `Failed to list versions: ${error}`,
      'LIST_VERSIONS_FAILED',
      { documentId }
    );
  }
}

/**
 * Autosave Operations
 */

export async function enableAutosave(
  documentId: number,
  intervalSeconds: number
): Promise<void> {
  try {
    await invoke('enable_autosave', { documentId, intervalSeconds });
  } catch (error) {
    throw new TauriError(
      `Failed to enable autosave: ${error}`,
      'ENABLE_AUTOSAVE_FAILED',
      { documentId }
    );
  }
}

export async function disableAutosave(documentId: number): Promise<void> {
  try {
    await invoke('disable_autosave', { documentId });
  } catch (error) {
    throw new TauriError(
      `Failed to disable autosave: ${error}`,
      'DISABLE_AUTOSAVE_FAILED',
      { documentId }
    );
  }
}

/**
 * Bookmark Operations
 */

export async function addBookmark(
  documentId: number,
  name: string,
  position: number
): Promise<number> {
  try {
    const bookmarkId = await invoke<number>('add_bookmark', { 
      documentId, 
      name, 
      position 
    });
    return bookmarkId;
  } catch (error) {
    throw new TauriError(
      `Failed to add bookmark: ${error}`,
      'ADD_BOOKMARK_FAILED',
      { documentId, name }
    );
  }
}

export async function removeBookmark(bookmarkId: number): Promise<void> {
  try {
    await invoke('remove_bookmark', { bookmarkId });
  } catch (error) {
    throw new TauriError(
      `Failed to remove bookmark: ${error}`,
      'REMOVE_BOOKMARK_FAILED',
      { bookmarkId }
    );
  }
}

export async function getBookmarks(
  documentId: number
): Promise<Array<{ id: number; name: string; position: number }>> {
  try {
    const bookmarks = await invoke<Array<{ id: number; name: string; position: number }>>(
      'get_bookmarks',
      { documentId }
    );
    return bookmarks;
  } catch (error) {
    throw new TauriError(
      `Failed to get bookmarks: ${error}`,
      'GET_BOOKMARKS_FAILED',
      { documentId }
    );
  }
}

/**
 * Backup Operations
 */

export async function createBackup(
  documentId: number,
  backupPath?: string
): Promise<string> {
  try {
    const path = await invoke<string>('create_backup', { 
      documentId, 
      backupPath 
    });
    return path;
  } catch (error) {
    throw new TauriError(
      `Failed to create backup: ${error}`,
      'CREATE_BACKUP_FAILED',
      { documentId }
    );
  }
}

export async function restoreFromBackup(
  backupPath: string
): Promise<Document> {
  try {
    const doc = await invoke<Document>('restore_from_backup', { backupPath });
    return doc;
  } catch (error) {
    throw new TauriError(
      `Failed to restore from backup: ${error}`,
      'RESTORE_FROM_BACKUP_FAILED',
      { backupPath }
    );
  }
}

/**
 * System Information
 */

export async function getAppVersion(): Promise<string> {
  try {
    return await invoke<string>('get_app_version');
  } catch (error) {
    throw new TauriError(
      `Failed to get app version: ${error}`,
      'GET_APP_VERSION_FAILED'
    );
  }
}

export async function getDatabaseStats(): Promise<{
  documentCount: number;
  totalSize: number;
  lastBackup: string | null;
}> {
  try {
    return await invoke<{
      documentCount: number;
      totalSize: number;
      lastBackup: string | null;
    }>('get_database_stats');
  } catch (error) {
    throw new TauriError(
      `Failed to get database stats: ${error}`,
      'GET_DATABASE_STATS_FAILED'
    );
  }
}
