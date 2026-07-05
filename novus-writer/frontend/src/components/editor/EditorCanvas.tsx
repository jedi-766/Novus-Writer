import React, { useEffect, useState, useCallback } from 'react';
import { LexicalEditorComponent } from './LexicalEditor';
import { useEditorStore } from '../../stores/editorStore';
import * as tauriBridge from '../../utils/tauriBridge';
import type { Document } from '../../types/document';

interface EditorCanvasProps {
  documentId?: string;
}

export function EditorCanvas({ documentId }: EditorCanvasProps) {
  const [editor, setEditor] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const { 
    activeDocument, 
    setActiveDocument, 
    setDirty,
    updateWordCount,
    updateCharacterCount 
  } = useEditorStore();

  // Load document on mount
  useEffect(() => {
    async function loadDocument() {
      if (!documentId) return;
      
      setIsLoading(true);
      setError(null);
      
      try {
        const docId = parseInt(documentId, 10);
        const doc = await tauriBridge.openDocument(docId);
        setActiveDocument(doc);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load document');
      } finally {
        setIsLoading(false);
      }
    }
    
    loadDocument();
  }, [documentId, setActiveDocument]);

  // Auto-save on dirty state changes
  useEffect(() => {
    if (!activeDocument || !setDirty) return;
    
    const saveTimeout = setTimeout(async () => {
      if (setDirty && activeDocument.id) {
        try {
          // Save would be triggered by editor change events
          // This is a placeholder for auto-save logic
        } catch (err) {
          console.error('Auto-save failed:', err);
        }
      }
    }, 30000); // 30 second debounce
    
    return () => clearTimeout(saveTimeout);
  }, [activeDocument, setDirty]);

  const handleEditorInit = useCallback((editorInstance: any) => {
    setEditor(editorInstance);
  }, []);

  const handleContentChange = useCallback((content: string, wordCount: number, charCount: number) => {
    setDirty(true);
    updateWordCount(wordCount);
    updateCharacterCount(charCount);
  }, [setDirty, updateWordCount, updateCharacterCount]);

  if (isLoading) {
    return (
      <div className="editor-canvas loading">
        <div className="loading-spinner">Loading document...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="editor-canvas error">
        <div className="error-message">
          <h3>Error Loading Document</h3>
          <p>{error}</p>
          <button onClick={() => window.location.reload()}>Retry</button>
        </div>
      </div>
    );
  }

  return (
    <div className="editor-canvas">
      <LexicalEditorComponent 
        documentId={activeDocument?.id?.toString()}
        initialContent={activeDocument?.content}
        onContentChange={handleContentChange}
        onEditorInit={handleEditorInit}
      />
    </div>
  );
}
