import React from 'react';
import { useUIStore } from '../../stores/uiStore';

interface SidebarProps {
  children?: React.ReactNode;
}

export function Sidebar({ children }: SidebarProps) {
  const { isSidebarOpen, sidebarWidth } = useUIStore();

  if (!isSidebarOpen) {
    return null;
  }

  return (
    <div 
      className="sidebar"
      style={{ width: `${sidebarWidth}px` }}
    >
      <div className="sidebar-header">
        <h3>Navigation</h3>
      </div>
      <div className="sidebar-content">
        {children || (
          <div className="document-tree">
            <h4>Document Outline</h4>
            <ul>
              <li>Heading 1</li>
              <li>Heading 2</li>
              <li>Heading 3</li>
            </ul>
          </div>
        )}
      </div>
    </div>
  );
}
