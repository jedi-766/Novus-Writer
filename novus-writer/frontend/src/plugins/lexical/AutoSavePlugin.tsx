import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { invoke } from '@tauri-apps/api/core';

interface AutoSavePluginProps {
  documentId: string;
  interval?: number;
}

export function AutoSavePlugin({ documentId, interval = 30000 }: AutoSavePluginProps) {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    const saveContent = async () => {
      try {
        const editorState = editor.getEditorState();
        const content = editorState.toJSON();
        
        // Call backend autosave command
        await invoke('save_autosave', {
          docId: documentId,
          content: JSON.stringify(content),
        });
        
        console.log(`Auto-saved document ${documentId} at ${new Date().toISOString()}`);
      } catch (error) {
        console.error('Auto-save failed:', error);
      }
    };

    const timer = setInterval(saveContent, interval);
    return () => clearInterval(timer);
  }, [editor, documentId, interval]);

  return null;
}
