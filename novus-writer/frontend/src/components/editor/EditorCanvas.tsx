import React, { useState } from 'react';
import { LexicalEditorComponent } from './LexicalEditor';

interface EditorCanvasProps {
  documentId?: string;
}

export function EditorCanvas({ documentId }: EditorCanvasProps) {
  const [editor, setEditor] = useState<any>(null);

  const handleEditorInit = (editorInstance: any) => {
    setEditor(editorInstance);
  };

  return (
    <div className="editor-canvas">
      <LexicalEditorComponent 
        documentId={documentId}
      />
    </div>
  );
}
