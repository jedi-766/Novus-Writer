export interface Document {
  id: string;
  title: string;
  content?: string;
  createdAt: Date;
  updatedAt: Date;
  wordCount: number;
  characterCount: number;
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
