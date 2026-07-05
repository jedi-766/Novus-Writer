import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getSelection, $isRangeSelection, FORMAT_TEXT_COMMAND, COMMAND_PRIORITY_LOW } from 'lexical';

export function KeyboardShortcutsPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
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

      // Ctrl+E or Cmd+E: Strikethrough
      if ((e.ctrlKey || e.metaKey) && e.key === 'e') {
        e.preventDefault();
        editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            editor.dispatchCommand(FORMAT_TEXT_COMMAND, 'strikethrough');
          }
        });
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [editor]);

  return null;
}
