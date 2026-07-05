import React, { useCallback } from 'react';
import { LexicalEditor } from 'lexical';
import { $getSelection, $isRangeSelection } from 'lexical';
import { $patchStyleText } from '@lexical/selection';

interface LayoutTabProps {
  editor: LexicalEditor;
}

export function LayoutTab({ editor }: LayoutTabProps) {
  const setMargins = useCallback((top: string, right: string, bottom: string, left: string) => {
    editor.update(() => {
      const rootElement = editor.getRootElement();
      if (rootElement) {
        rootElement.style.padding = `${top} ${right} ${bottom} ${left}`;
      }
    });
  }, [editor]);

  const setOrientation = useCallback((orientation: 'portrait' | 'landscape') => {
    editor.update(() => {
      const rootElement = editor.getRootElement();
      if (rootElement) {
        if (orientation === 'landscape') {
          rootElement.style.width = '1056px'; // Letter landscape width approximation
        } else {
          rootElement.style.width = '816px'; // Letter portrait width approximation
        }
      }
    });
  }, [editor]);

  return (
    <div className="ribbon-tab-content layout-tab">
      {/* Page Setup Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => setMargins('96px', '96px', '96px', '96px')}
          title="Normal Margins"
        >
          📐 Normal
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => setMargins('48px', '48px', '48px', '48px')}
          title="Narrow Margins"
        >
          Narrow
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => setMargins('144px', '144px', '144px', '144px')}
          title="Wide Margins"
        >
          Wide
        </button>
      </div>

      {/* Orientation Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => setOrientation('portrait')}
          title="Portrait Orientation"
        >
          📄 Portrait
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => setOrientation('landscape')}
          title="Landscape Orientation"
        >
          📑 Landscape
        </button>
      </div>

      {/* Size Group */}
      <div className="ribbon-group">
        <select className="ribbon-select" onChange={(e) => {
          const size = e.target.value;
          const rootElement = editor.getRootElement();
          if (rootElement && size) {
            switch (size) {
              case 'letter':
                rootElement.style.width = '816px';
                break;
              case 'legal':
                rootElement.style.width = '816px';
                break;
              case 'a4':
                rootElement.style.width = '794px';
                break;
            }
          }
        }}>
          <option value="">Paper Size</option>
          <option value="letter">Letter (8.5" x 11")</option>
          <option value="legal">Legal (8.5" x 14")</option>
          <option value="a4">A4 (210mm x 297mm)</option>
        </select>
      </div>

      {/* Columns Group */}
      <div className="ribbon-group">
        <button className="ribbon-btn" title="One Column">
          1 Column
        </button>
        <button className="ribbon-btn" title="Two Columns">
          2 Columns
        </button>
        <button className="ribbon-btn" title="Three Columns">
          3 Columns
        </button>
      </div>

      {/* Spacing Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => $patchStyleText($getSelection(), { 'line-height': '1.0' })}
          title="1.0 Spacing"
        >
          1.0
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => $patchStyleText($getSelection(), { 'line-height': '1.15' })}
          title="1.15 Spacing"
        >
          1.15
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => $patchStyleText($getSelection(), { 'line-height': '1.5' })}
          title="1.5 Spacing"
        >
          1.5
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => $patchStyleText($getSelection(), { 'line-height': '2.0' })}
          title="2.0 Spacing"
        >
          2.0
        </button>
      </div>
    </div>
  );
}
