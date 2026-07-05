import React, { useCallback } from 'react';
import { LexicalEditor } from 'lexical';
import { $getSelection, $isRangeSelection } from 'lexical';
import { $patchStyleText } from '@lexical/selection';

interface HomeTabProps {
  editor: LexicalEditor;
}

export function HomeTab({ editor }: HomeTabProps) {
  const formatText = useCallback((format: string) => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        $patchStyleText(selection, { [format]: 'true' });
      }
    });
  }, [editor]);

  const toggleBold = useCallback(() => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        selection.toggleFormat('bold');
      }
    });
  }, [editor]);

  const toggleItalic = useCallback(() => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        selection.toggleFormat('italic');
      }
    });
  }, [editor]);

  const toggleUnderline = useCallback(() => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        selection.toggleFormat('underline');
      }
    });
  }, [editor]);

  const toggleStrikethrough = useCallback(() => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        selection.toggleFormat('strikethrough');
      }
    });
  }, [editor]);

  const alignText = useCallback((alignment: 'left' | 'center' | 'right' | 'justify') => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        $patchStyleText(selection, { 'text-align': alignment });
      }
    });
  }, [editor]);

  return (
    <div className="ribbon-tab-content home-tab">
      {/* Clipboard Group */}
      <div className="ribbon-group">
        <button className="ribbon-btn" title="Paste (Ctrl+V)">
          📋 Paste
        </button>
        <button className="ribbon-btn" title="Cut (Ctrl+X)">
          ✂️ Cut
        </button>
        <button className="ribbon-btn" title="Copy (Ctrl+C)">
          📄 Copy
        </button>
      </div>

      {/* Font Group */}
      <div className="ribbon-group">
        <select className="ribbon-select" onChange={(e) => formatText(`font-family: ${e.target.value}`)}>
          <option value="">Font Family</option>
          <option value="Arial">Arial</option>
          <option value="Times New Roman">Times New Roman</option>
          <option value="Georgia">Georgia</option>
          <option value="Verdana">Verdana</option>
          <option value="Courier New">Courier New</option>
        </select>
        <select className="ribbon-select" onChange={(e) => formatText(`font-size: ${e.target.value}`)}>
          <option value="">Size</option>
          <option value="12px">12</option>
          <option value="14px">14</option>
          <option value="16px">16</option>
          <option value="18px">18</option>
          <option value="24px">24</option>
          <option value="36px">36</option>
        </select>
      </div>

      {/* Formatting Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={toggleBold}
          title="Bold (Ctrl+B)"
        >
          <strong>B</strong>
        </button>
        <button 
          className="ribbon-btn" 
          onClick={toggleItalic}
          title="Italic (Ctrl+I)"
        >
          <em>I</em>
        </button>
        <button 
          className="ribbon-btn" 
          onClick={toggleUnderline}
          title="Underline (Ctrl+U)"
        >
          <u>U</u>
        </button>
        <button 
          className="ribbon-btn" 
          onClick={toggleStrikethrough}
          title="Strikethrough"
        >
          <s>S</s>
        </button>
      </div>

      {/* Alignment Group */}
      <div className="ribbon-group">
        <button className="ribbon-btn" onClick={() => alignText('left')} title="Align Left">
          ⫷ Left
        </button>
        <button className="ribbon-btn" onClick={() => alignText('center')} title="Center">
          Center
        </button>
        <button className="ribbon-btn" onClick={() => alignText('right')} title="Align Right">
          Right ⫸
        </button>
        <button className="ribbon-btn" onClick={() => alignText('justify')} title="Justify">
          Justify
        </button>
      </div>

      {/* Styles Group */}
      <div className="ribbon-group">
        <select className="ribbon-select" onChange={(e) => {
          const heading = e.target.value;
          if (heading) {
            editor.update(() => {
              const selection = $getSelection();
              if ($isRangeSelection(selection)) {
                // Format as heading
                selection.insertNodes([]);
              }
            });
          }
        }}>
          <option value="">Styles</option>
          <option value="h1">Heading 1</option>
          <option value="h2">Heading 2</option>
          <option value="h3">Heading 3</option>
          <option value="normal">Normal</option>
        </select>
      </div>
    </div>
  );
}
