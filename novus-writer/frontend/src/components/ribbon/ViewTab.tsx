import React, { useCallback } from 'react';
import { LexicalEditor } from 'lexical';
import { useUIStore } from '../../stores/uiStore';
import { useEditorStore } from '../../stores/editorStore';

interface ViewTabProps {
  editor: LexicalEditor;
}

export function ViewTab({ editor }: ViewTabProps) {
  const { isSidebarOpen, toggleSidebar } = useUIStore();
  const { zoom, setZoom } = useEditorStore();

  const setZoomLevel = useCallback((level: number) => {
    setZoom(level);
    const editorElement = editor.getRootElement();
    if (editorElement) {
      editorElement.style.zoom = `${level / 100}`;
    }
  }, [editor, setZoom]);

  return (
    <div className="ribbon-tab-content view-tab">
      {/* Views Group */}
      <div className="ribbon-group">
        <button className="ribbon-btn active" title="Print Layout">
          📄 Print Layout
        </button>
        <button className="ribbon-btn" title="Web Layout">
          🌐 Web Layout
        </button>
        <button className="ribbon-btn" title="Outline">
          📝 Outline
        </button>
      </div>

      {/* Show/Hide Group */}
      <div className="ribbon-group">
        <button 
          className={`ribbon-btn ${isSidebarOpen ? 'active' : ''}`} 
          onClick={toggleSidebar}
          title="Toggle Navigation Pane"
        >
          ☰ Navigation
        </button>
        <button className="ribbon-btn" title="Show Ruler">
          📏 Ruler
        </button>
        <button className="ribbon-btn" title="Show Gridlines">
          ⊞ Gridlines
        </button>
      </div>

      {/* Zoom Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => setZoomLevel(100)}
          title="100% Zoom"
        >
          100%
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => setZoomLevel(zoom - 10)}
          disabled={zoom <= 50}
          title="Zoom Out"
        >
          🔍-
        </button>
        <span className="ribbon-label">{zoom}%</span>
        <button 
          className="ribbon-btn" 
          onClick={() => setZoomLevel(zoom + 10)}
          disabled={zoom >= 200}
          title="Zoom In"
        >
          🔍+
        </button>
        <button 
          className="ribbon-btn" 
          onClick={() => setZoomLevel(100)}
          title="Reset Zoom"
        >
          Reset
        </button>
      </div>

      {/* Window Group */}
      <div className="ribbon-group">
        <button className="ribbon-btn" title="New Window">
          🪟 New Window
        </button>
        <button className="ribbon-btn" title="Arrange All">
          ⧉ Arrange All
        </button>
        <button className="ribbon-btn" title="Split">
          ⫿ Split
        </button>
      </div>
    </div>
  );
}
