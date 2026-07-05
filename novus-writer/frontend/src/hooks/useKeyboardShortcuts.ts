import { useEffect, useCallback } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getSelection, $isRangeSelection, FORMAT_TEXT_COMMAND } from 'lexical';
import { invoke } from '@tauri-apps/api/core';

interface KeyboardShortcutOptions {
  onSave?: () => void;
  documentId?: string;
}

export function useKeyboardShortcuts(options: KeyboardShortcutOptions = {}) {
  const [editor] = useLexicalComposerContext();
  const { onSave, documentId } = options;

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+S or Cmd+S: Save
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        if (onSave) {
          onSave();
        } else if (documentId) {
          editor.getEditorState().read(() => {
            // Trigger save via backend
            invoke('save_document', {
              docId: documentId,
              // content would be serialized here
            }).catch(console.error);
          });
        }
      }

      // Ctrl+B or Cmd+B: Bold
      if ((e.ctrlKey || e.metaKey) && e.key === 'b') {
        e.preventDefault();
        editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            editor.dispatchCommand(FORMAT_TEXT_COMMAND, 'bold');
          }
        });
      }

      // Ctrl+I or Cmd+I: Italic
      if ((e.ctrlKey || e.metaKey) && e.key === 'i') {
        e.preventDefault();
        editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            editor.dispatchCommand(FORMAT_TEXT_COMMAND, 'italic');
          }
        });
      }

      // Ctrl+U or Cmd+U: Underline
      if ((e.ctrlKey || e.metaKey) && e.key === 'u') {
        e.preventDefault();
        editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            editor.dispatchCommand(FORMAT_TEXT_COMMAND, 'underline');
          }
        });
      }

      // Ctrl+Z or Cmd+Z: Undo (handled by Lexical HistoryPlugin)
      
      // Ctrl+Y or Cmd+Shift+Z: Redo (handled by Lexical HistoryPlugin)
      
      // Ctrl+K or Cmd+K: Insert Link
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        // Could trigger link insertion dialog
        console.log('Insert link triggered');
      }

      // Ctrl+H: Find and Replace
      if ((e.ctrlKey || e.metaKey) && e.key === 'h') {
        e.preventDefault();
        console.log('Find and Replace triggered');
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [editor, onSave, documentId]);

  return null;
}
