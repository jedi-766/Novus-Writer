import { useCallback } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getSelection, $isRangeSelection, FORMAT_TEXT_COMMAND, TextFormatType } from 'lexical';

export function useTextFormatting() {
  const [editor] = useLexicalComposerContext();

  const toggleFormat = useCallback((format: TextFormatType) => {
    editor.dispatchCommand(FORMAT_TEXT_COMMAND, format);
  }, [editor]);

  const toggleBold = useCallback(() => {
    toggleFormat('bold');
  }, [toggleFormat]);

  const toggleItalic = useCallback(() => {
    toggleFormat('italic');
  }, [toggleFormat]);

  const toggleUnderline = useCallback(() => {
    toggleFormat('underline');
  }, [toggleFormat]);

  const toggleStrikethrough = useCallback(() => {
    toggleFormat('strikethrough');
  }, [toggleFormat]);

  const toggleCode = useCallback(() => {
    toggleFormat('code');
  }, [toggleFormat]);

  return {
    toggleBold,
    toggleItalic,
    toggleUnderline,
    toggleStrikethrough,
    toggleCode,
    toggleFormat,
  };
}
