import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getSelection, $isRangeSelection } from 'lexical';
import { useUIStore } from '../../stores/uiStore';

interface ToolbarSyncPluginProps {
  onSelectionChange?: (formats: {
    bold: boolean;
    italic: boolean;
    underline: boolean;
    strikethrough: boolean;
    code: boolean;
    fontSize?: string;
    fontFamily?: string;
  }) => void;
}

export function ToolbarSyncPlugin({ onSelectionChange }: ToolbarSyncPluginProps) {
  const [editor] = useLexicalComposerContext();
  const { setActiveRibbonTab } = useUIStore();

  useEffect(() => {
    const updateToolbar = () => {
      editor.getEditorState().read(() => {
        const selection = $getSelection();
        
        if ($isRangeSelection(selection)) {
          const formats = {
            bold: selection.hasFormat('bold'),
            italic: selection.hasFormat('italic'),
            underline: selection.hasFormat('underline'),
            strikethrough: selection.hasFormat('strikethrough'),
            code: selection.hasFormat('code'),
            fontSize: selection.getStyleProperty('font-size') || undefined,
            fontFamily: selection.getStyleProperty('font-family') || undefined,
          };

          if (onSelectionChange) {
            onSelectionChange(formats);
          }
        }
      });
    };

    // Subscribe to cursor movement and selection changes
    const unsubscribe = editor.registerUpdateListener(({ tags }) => {
      if (!tags.has('skip-toolbar-sync')) {
        updateToolbar();
      }
    });

    // Also update on selection change
    const unregisterSelectionChange = editor.registerSelectionChangeListener(() => {
      updateToolbar();
    });

    return () => {
      unsubscribe();
      unregisterSelectionChange();
    };
  }, [editor, onSelectionChange]);

  return null;
}
