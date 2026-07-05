/**
 * Search type definitions for Novus Writer
 */

export interface SearchResult {
  documentId: number;
  offset: number;
  length: number;
  context: string;
  lineNumber?: number;
}

export interface SearchOptions {
  query: string;
  caseSensitive?: boolean;
  wholeWord?: boolean;
  useRegex?: boolean;
  searchInComments?: boolean;
  searchInHeaders?: boolean;
}

export interface ReplaceResult {
  replacementsCount: number;
  documentsAffected: number[];
}

export interface FindReplaceHistory {
  findHistory: string[];
  replaceHistory: string[];
}
