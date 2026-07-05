import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getRoot, $createParagraphNode, $createTextNode } from 'lexical';
import { useEditorStore } from '../../stores/editorStore';

interface WordCountPluginProps {
  onUpdate?: (wordCount: number, charCount: number) => void;
}

export function WordCountPlugin({ onUpdate }: WordCountPluginProps) {
  const [editor] = useLexicalComposerContext();
  const { updateWordCount, updateCharacterCount } = useEditorStore();

  useEffect(() => {
    const updateCounts = () => {
      editor.getEditorState().read(() => {
        const root = $getRoot();
        const textContent = root.getTextContent();
        
        // Count words
        const words = textContent.trim() === '' ? 0 : textContent.trim().split(/\s+/).filter(w => w.length > 0).length;
        const characters = textContent.length;
        
        // Update store
        updateWordCount(words);
        updateCharacterCount(characters);
        
        // Also call callback if provided
        if (onUpdate) {
          onUpdate(words, characters);
        }
      });
    };

    // Initial count
    updateCounts();

    // Subscribe to editor changes
    const unsubscribe = editor.registerUpdateListener(() => {
      updateCounts();
    });

    return () => {
      unsubscribe();
    };
  }, [editor, updateWordCount, updateCharacterCount, onUpdate]);

  return null;
}
