import { create } from 'zustand';
import type { EditorStore, Document } from '../types/document';

export const useEditorStore = create<EditorStore>((set) => ({
  activeDocument: null,
  isDirty: false,
  wordCount: 0,
  characterCount: 0,
  zoom: 100,
  currentPage: 1,
  totalPages: 1,
  language: 'en-US',
  
  setActiveDocument: (doc: Document | null) => set({ activeDocument: doc }),
  setDirty: (dirty: boolean) => set({ isDirty: dirty }),
  updateWordCount: (count: number) => set({ wordCount: count }),
  updateCharacterCount: (count: number) => set({ characterCount: count }),
  setZoom: (zoom: number) => set({ zoom }),
  setCurrentPage: (page: number) => set({ currentPage: page }),
  setTotalPages: (pages: number) => set({ totalPages: pages }),
  setLanguage: (lang: string) => set({ language: lang }),
  reset: () => set({
    activeDocument: null,
    isDirty: false,
    wordCount: 0,
    characterCount: 0,
    zoom: 100,
    currentPage: 1,
    totalPages: 1,
    language: 'en-US',
  }),
}));
