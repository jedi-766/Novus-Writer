import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getSelection, $isRangeSelection, $createTextNode } from 'lexical';

export function SpellCheckPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    // Enable browser spell check on the editor content editable
    const enableSpellCheck = () => {
      const editorElement = editor.getRootElement();
      if (editorElement) {
        editorElement.setAttribute('spellcheck', 'true');
        // Set language for spell checking
        editorElement.setAttribute('lang', 'en-US');
      }
    };

    enableSpellCheck();

    // Re-apply on focus
    const unsubscribe = editor.registerRootListener((root) => {
      if (root) {
        root.setAttribute('spellcheck', 'true');
        root.setAttribute('lang', 'en-US');
      }
    });

    return () => {
      unsubscribe();
    };
  }, [editor]);

  return null;
}
