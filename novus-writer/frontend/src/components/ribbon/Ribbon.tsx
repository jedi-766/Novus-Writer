import React, { useState } from 'react';
import { LexicalEditor } from 'lexical';
import { useUIStore } from '../../stores/uiStore';
import { HomeTab } from './HomeTab';
import { InsertTab } from './InsertTab';
import { LayoutTab } from './LayoutTab';
import { ViewTab } from './ViewTab';
import { ExportTab } from './ExportTab';
import { ImportTab } from './ImportTab';
import { ReviewTab } from './ReviewTab';

interface RibbonProps {
  editor: LexicalEditor;
}

type TabType = 'home' | 'insert' | 'layout' | 'view' | 'export' | 'import' | 'review';

export function Ribbon({ editor }: RibbonProps) {
  const { activeRibbonTab, setActiveRibbonTab, isRibbonMinimized, toggleRibbonMinimized } = useUIStore();
  const [hoveredTab, setHoveredTab] = useState<TabType | null>(null);

  const tabs: { id: TabType; label: string }[] = [
    { id: 'home', label: 'Home' },
    { id: 'insert', label: 'Insert' },
    { id: 'layout', label: 'Layout' },
    { id: 'view', label: 'View' },
    { id: 'export', label: 'Export' },
    { id: 'import', label: 'Import' },
    { id: 'review', label: 'Review' },
  ];

  const renderTabContent = () => {
    switch (activeRibbonTab) {
      case 'home':
        return <HomeTab editor={editor} />;
      case 'insert':
        return <InsertTab editor={editor} />;
      case 'layout':
        return <LayoutTab editor={editor} />;
      case 'view':
        return <ViewTab editor={editor} />;
      case 'export':
        return <ExportTab editor={editor} />;
      case 'import':
        return <ImportTab editor={editor} />;
      case 'review':
        return <ReviewTab editor={editor} />;
      default:
        return <HomeTab editor={editor} />;
    }
  };

  if (isRibbonMinimized && hoveredTab === null) {
    return (
      <div className="ribbon-minimized">
        <div className="ribbon-tabs">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              className={`ribbon-tab ${activeRibbonTab === tab.id ? 'active' : ''}`}
              onClick={() => {
                setActiveRibbonTab(tab.id);
                toggleRibbonMinimized();
              }}
              onMouseEnter={() => setHoveredTab(tab.id)}
              onMouseLeave={() => setHoveredTab(null)}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="ribbon">
      <div className="ribbon-header">
        <div className="ribbon-tabs">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              className={`ribbon-tab ${activeRibbonTab === tab.id ? 'active' : ''}`}
              onClick={() => setActiveRibbonTab(tab.id)}
              onMouseEnter={() => isRibbonMinimized && setHoveredTab(tab.id)}
              onMouseLeave={() => isRibbonMinimized && setHoveredTab(null)}
              onDoubleClick={toggleRibbonMinimized}
            >
              {tab.label}
            </button>
          ))}
        </div>
        <button className="ribbon-minimize-btn" onClick={toggleRibbonMinimized} title="Minimize Ribbon">
          ⌄
        </button>
      </div>
      <div className="ribbon-content">
        {renderTabContent()}
      </div>
    </div>
  );
}
