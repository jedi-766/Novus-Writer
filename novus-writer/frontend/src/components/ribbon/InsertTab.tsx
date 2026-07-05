import React, { useCallback } from 'react';
import { LexicalEditor } from 'lexical';
import { $getSelection, $isRangeSelection } from 'lexical';
import { $createLinkNode } from '@lexical/link';
import { $createTableNode } from '../../plugins/nodes/TableNode';
import { $createImageNode } from '../../plugins/nodes/ImageNode';

interface InsertTabProps {
  editor: LexicalEditor;
}

export function InsertTab({ editor }: InsertTabProps) {
  const insertTable = useCallback((rows: number, columns: number) => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        const tableNode = $createTableNode(rows, columns);
        selection.insertNodes([tableNode]);
      }
    });
  }, [editor]);

  const insertImage = useCallback((src: string, altText: string = '') => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        const imageNode = $createImageNode(src, altText);
        selection.insertNodes([imageNode]);
      }
    });
  }, [editor]);

  const insertLink = useCallback((url: string, text?: string) => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        const linkNode = $createLinkNode(url);
        selection.insertNodes([linkNode]);
      }
    });
  }, [editor]);

  const insertPageBreak = useCallback(() => {
    editor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        // Insert page break (implementation depends on pagination strategy)
        console.log('Insert page break');
      }
    });
  }, [editor]);

  return (
    <div className="ribbon-tab-content insert-tab">
      {/* Pages Group */}
      <div className="ribbon-group">
        <button className="ribbon-btn" onClick={insertPageBreak} title="Insert Page Break">
          📄 Page Break
        </button>
        <button className="ribbon-btn" title="Cover Page">
          📑 Cover Page
        </button>
      </div>

      {/* Tables Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => insertTable(3, 3)}
          title="Insert Table"
        >
          ▦ Table
        </button>
      </div>

      {/* Images Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => {
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = 'image/*';
            input.onchange = (e) => {
              const file = (e.target as HTMLInputElement).files?.[0];
              if (file) {
                const reader = new FileReader();
                reader.onload = (event) => {
                  const src = event.target?.result as string;
                  insertImage(src, file.name);
                };
                reader.readAsDataURL(file);
              }
            };
            input.click();
          }}
          title="Insert Image"
        >
          🖼️ Image
        </button>
        <button className="ribbon-btn" title="Insert Online Pictures">
          🌐 Online Pictures
        </button>
      </div>

      {/* Links Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => {
            const url = prompt('Enter URL:');
            if (url) {
              insertLink(url);
            }
          }}
          title="Insert Link (Ctrl+K)"
        >
          🔗 Link
        </button>
        <button className="ribbon-btn" title="Insert Bookmark">
          🔖 Bookmark
        </button>
      </div>

      {/* Symbols Group */}
      <div className="ribbon-group">
        <button className="ribbon-btn" title="Insert Symbol">
          Ω Symbol
        </button>
        <button className="ribbon-btn" title="Insert Equation">
          √ Equation
        </button>
      </div>
    </div>
  );
}
