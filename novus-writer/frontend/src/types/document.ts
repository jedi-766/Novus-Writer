export interface Document {
  id: number;
  title: string;
  content?: string;
  createdAt: string;
  updatedAt: string;
  wordCount: number;
  characterCount: number;
  folderId?: number;
  isDeleted?: boolean;
  metadata?: DocumentMetadata;
}

export interface DocumentMetadata {
  author?: string;
  tags?: string[];
  category?: string;
  customFields?: Record<string, string>;
}

export interface DocumentFolder {
  id: number;
  name: string;
  parentId?: number;
  createdAt: string;
  documentCount: number;
}

export interface EditorStore {
  activeDocument: Document | null;
  isDirty: boolean;
  wordCount: number;
  characterCount: number;
  zoom: number;
  currentPage: number;
  totalPages: number;
  language: string;
  
  // Actions
  setActiveDocument: (doc: Document | null) => void;
  setDirty: (dirty: boolean) => void;
  updateWordCount: (count: number) => void;
  updateCharacterCount: (count: number) => void;
  setZoom: (zoom: number) => void;
  setCurrentPage: (page: number) => void;
  setTotalPages: (pages: number) => void;
  setLanguage: (lang: string) => void;
  reset: () => void;
}
