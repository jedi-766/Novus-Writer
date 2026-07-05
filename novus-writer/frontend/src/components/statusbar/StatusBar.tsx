import React from 'react';
import { useEditorStore } from '../../stores/editorStore';

interface StatusBarProps {
  editor?: any;
}

export function StatusBar({ editor }: StatusBarProps) {
  const { wordCount, characterCount, zoom, currentPage, totalPages, language } = useEditorStore();

  return (
    <div className="status-bar">
      <div className="status-bar-left">
        <span className="status-bar-item">
          Pages: {currentPage} of {totalPages}
        </span>
        <span className="status-bar-divider">|</span>
        <span className="status-bar-item">
          Words: {wordCount}
        </span>
        <span className="status-bar-divider">|</span>
        <span className="status-bar-item">
          Characters: {characterCount}
        </span>
      </div>
      
      <div className="status-bar-right">
        <span className="status-bar-item">
          Language: {language}
        </span>
        <span className="status-bar-divider">|</span>
        <span className="status-bar-item">
          Zoom: {zoom}%
        </span>
        <select 
          className="status-bar-select"
          value={zoom}
          onChange={(e) => {
            // Zoom change would be handled by store action
            console.log('Zoom changed to:', e.target.value);
          }}
        >
          <option value="50">50%</option>
          <option value="75">75%</option>
          <option value="100">100%</option>
          <option value="125">125%</option>
          <option value="150">150%</option>
          <option value="200">200%</option>
        </select>
      </div>
    </div>
  );
}
