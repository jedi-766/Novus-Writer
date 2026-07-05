# Novus Writer - Phase 4: Editor Integration

## Overview

Phase 4 implements the Lexical rich text editor integration with custom nodes, plugins, and full Word-like editing capabilities. This phase builds upon the completed database layer (Phase 3) to provide a production-ready editing experience.

## Objectives

1. **Lexical Editor Core Integration**
   - Set up Lexical editor with React
   - Configure editor settings for Word-like behavior
   - Implement editor state management

2. **Custom Node Types**
   - Image nodes with captions and resizing
   - Table nodes with advanced editing
   - Horizontal rule nodes
   - Code block nodes with syntax highlighting

3. **Editor Plugins**
   - AutoSave plugin (integrates with Phase 3 database)
   - KeyboardShortcuts plugin (Word-like shortcuts)
   - History plugin (undo/redo)
   - Clipboard plugin (copy/paste with formatting)
   - SpellCheck plugin
   - ToolbarSync plugin (ribbon UI synchronization)

4. **Ribbon UI Components**
   - Home tab (formatting, styles, clipboard)
   - Insert tab (images, tables, links, pages)
   - Layout tab (margins, orientation, columns)
   - View tab (zoom, rulers, navigation)

5. **Document Management UI**
   - Document tabs/multi-document support
   - Recent documents list
   - Search and filter

6. **Status Bar**
   - Word/character count
   - Page number
   - Language
   - Zoom control

## Architecture

### Frontend Editor Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Application Shell                     │
├──────────────┬──────────────────────────┬───────────────┤
│   Ribbon     │      Editor Canvas       │    Sidebar    │
│   (Toolbar)  │    (Lexical Editor)      │  (Optional)   │
├──────────────┴──────────────────────────┴───────────────┤
│                      Status Bar                          │
└─────────────────────────────────────────────────────────┘
```

### Editor State Flow

```
User Action → Lexical Command → Plugin Handler → State Update
                                         ↓
                                  AutoSave Plugin
                                         ↓
                                  Backend (Phase 3)
                                         ↓
                                  SQLite Database
```

### Component Hierarchy

```
App
├── Ribbon (Toolbar)
│   ├── HomeTab
│   ├── InsertTab
│   ├── LayoutTab
│   └── ViewTab
├── EditorArea
│   ├── DocumentTabs
│   └── LexicalEditor
│       ├── ToolbarPlugin
│       ├── AutoSavePlugin
│       ├── HistoryPlugin
│       ├── KeyboardShortcutsPlugin
│       ├── ClipboardPlugin
│       ├── SpellCheckPlugin
│       └── CustomNodes
│           ├── ImageNode
│           ├── TableNode
│           ├── HorizontalRuleNode
│           └── CodeNode
├── Sidebar (optional)
│   ├── DocumentTree
│   └── Bookmarks
└── StatusBar
```

## Implementation Details

### 1. Lexical Editor Setup

#### Editor Configuration

```typescript
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';

const editorConfig = {
  namespace: 'NovusWriter',
  theme: {
    paragraph: 'editor-paragraph',
    heading: {
      h1: 'editor-heading-h1',
      h2: 'editor-heading-h2',
      h3: 'editor-heading-h3',
    },
    list: {
      ul: 'editor-list-ul',
      ol: 'editor-list-ol',
      listitem: 'editor-list-item',
    },
    image: 'editor-image',
    table: 'editor-table',
  },
  nodes: [
    // Custom nodes registered here
  ],
  onError: (error: Error) => {
    console.error('Lexical error:', error);
  },
};
```

### 2. Custom Node Implementations

#### Image Node

```typescript
import { DecoratorNode, DOMConversion, DOMExportOutput } from 'lexical';

export class ImageNode extends DecoratorNode<React.ElementType> {
  __src: string;
  __altText: string;
  __width: number;
  __height: number;
  __caption?: string;
  __maxWidth: number;

  constructor(
    src: string,
    altText: string,
    width: number,
    height: number,
    caption?: string,
    maxWidth?: number,
  ) {
    super();
    this.__src = src;
    this.__altText = altText;
    this.__width = width;
    this.__height = height;
    this.__caption = caption;
    this.__maxWidth = maxWidth || 700;
  }

  // Implementation details...
}
```

#### Table Node

```typescript
import { ElementNode } from 'lexical';

export class TableNode extends ElementNode {
  __rows: number;
  __columns: number;
  __cells: TableCellNode[][];

  static getType(): string {
    return 'table';
  }

  // Table manipulation methods...
}
```

### 3. Plugin Implementations

#### AutoSave Plugin

```typescript
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function AutoSavePlugin({ documentId, interval = 30000 }: AutoSaveProps) {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    const saveContent = async () => {
      editor.getEditorState().read(() => {
        const content = JSON.stringify($getRoot());
        
        // Call backend autosave command
        invoke('save_autosave', {
          docId: documentId,
          content: content,
        });
      });
    };

    const timer = setInterval(saveContent, interval);
    return () => clearInterval(timer);
  }, [editor, documentId, interval]);

  return null;
}
```

#### KeyboardShortcuts Plugin

```typescript
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useEffect } from 'react';
import { $getSelection, $isRangeSelection } from 'lexical';

export function KeyboardShortcutsPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+S: Save
      if (e.ctrlKey && e.key === 's') {
        e.preventDefault();
        // Trigger save
      }
      
      // Ctrl+B: Bold
      if (e.ctrlKey && e.key === 'b') {
        e.preventDefault();
        editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            selection.toggleFormat('bold');
          }
        });
      }
      
      // Ctrl+I: Italic
      if (e.ctrlKey && e.key === 'i') {
        e.preventDefault();
        editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            selection.toggleFormat('italic');
          }
        });
      }
      
      // Ctrl+U: Underline
      if (e.ctrlKey && e.key === 'u') {
        e.preventDefault();
        editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            selection.toggleFormat('underline');
          }
        });
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [editor]);

  return null;
}
```

### 4. Ribbon/Toolbar Component

```typescript
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $patchStyleText } from '@lexical/selection';
import { $getSelection } from 'lexical';

export function Ribbon() {
  const [editor] = useLexicalComposerContext();
  const [activeTab, setActiveTab] = useState('home');

  const formatText = (format: string) => {
    editor.update(() => {
      const selection = $getSelection();
      if (selection) {
        $patchStyleText(selection, { [format]: 'true' });
      }
    });
  };

  return (
    <div className="ribbon">
      <Tabs value={activeTab} onChange={setActiveTab}>
        <Tab label="Home">
          <HomeTab editor={editor} />
        </Tab>
        <Tab label="Insert">
          <InsertTab editor={editor} />
        </Tab>
        <Tab label="Layout">
          <LayoutTab editor={editor} />
        </Tab>
        <Tab label="View">
          <ViewTab editor={editor} />
        </Tab>
      </Tabs>
    </div>
  );
}
```

### 5. State Management (Zustand)

```typescript
import { create } from 'zustand';

interface EditorStore {
  activeDocument: Document | null;
  isDirty: boolean;
  wordCount: number;
  characterCount: number;
  zoom: number;
  
  // Actions
  setActiveDocument: (doc: Document | null) => void;
  setDirty: (dirty: boolean) => void;
  updateWordCount: (count: number) => void;
  setZoom: (zoom: number) => void;
}

export const useEditorStore = create<EditorStore>((set) => ({
  activeDocument: null,
  isDirty: false,
  wordCount: 0,
  characterCount: 0,
  zoom: 100,
  
  setActiveDocument: (doc) => set({ activeDocument: doc }),
  setDirty: (dirty) => set({ isDirty: dirty }),
  updateWordCount: (count) => set({ wordCount: count }),
  setZoom: (zoom) => set({ zoom }),
}));
```

## File Structure

```
novus-writer/
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── common/
│   │   │   │   ├── Button.tsx
│   │   │   │   ├── Dropdown.tsx
│   │   │   │   ├── Modal.tsx
│   │   │   │   └── Tooltip.tsx
│   │   │   ├── ribbon/
│   │   │   │   ├── Ribbon.tsx
│   │   │   │   ├── HomeTab.tsx
│   │   │   │   ├── InsertTab.tsx
│   │   │   │   ├── LayoutTab.tsx
│   │   │   │   └── ViewTab.tsx
│   │   │   ├── editor/
│   │   │   │   ├── LexicalEditor.tsx
│   │   │   │   ├── EditorCanvas.tsx
│   │   │   │   ├── DocumentTabs.tsx
│   │   │   │   └── FloatingToolbar.tsx
│   │   │   ├── sidebar/
│   │   │   │   ├── DocumentTree.tsx
│   │   │   │   └── BookmarksPanel.tsx
│   │   │   ├── statusbar/
│   │   │   │   └── StatusBar.tsx
│   │   │   └── layout/
│   │   │       ├── AppShell.tsx
│   │   │       └── SplitPane.tsx
│   │   ├── plugins/
│   │   │   ├── lexical/
│   │   │   │   ├── AutoSavePlugin.tsx
│   │   │   │   ├── KeyboardShortcutsPlugin.tsx
│   │   │   │   ├── HistoryPlugin.tsx
│   │   │   │   ├── ClipboardPlugin.tsx
│   │   │   │   ├── SpellCheckPlugin.tsx
│   │   │   │   └── ToolbarSyncPlugin.tsx
│   │   │   ├── nodes/
│   │   │   │   ├── ImageNode.tsx
│   │   │   │   ├── TableNode.tsx
│   │   │   │   ├── HorizontalRuleNode.tsx
│   │   │   │   └── CodeNode.tsx
│   │   │   └── features/
│   │   │       ├── FindReplace.tsx
│   │   │       └── Navigation.tsx
│   │   ├── stores/
│   │   │   ├── editorStore.ts
│   │   │   ├── documentStore.ts
│   │   │   └── uiStore.ts
│   │   ├── hooks/
│   │   │   ├── useEditor.ts
│   │   │   ├── useDocument.ts
│   │   │   └── useKeyboardShortcut.ts
│   │   ├── utils/
│   │   │   ├── wordCounter.ts
│   │   │   └── exportHelpers.ts
│   │   ├── styles/
│   │   │   ├── editor.css
│   │   │   ├── ribbon.css
│   │   │   └── themes/
│   │   │       ├── light.css
│   │   │       └── dark.css
│   │   ├── types/
│   │   │   ├── editor.ts
│   │   │   └── document.ts
│   │   ├── App.tsx
│   │   └── main.tsx
│   ├── public/
│   │   └── icons/
│   ├── package.json
│   ├── tsconfig.json
│   ├── tailwind.config.js
│   └── vite.config.ts
└── src-tauri/
    └── src/
        ├── commands/
        │   └── editor.rs (enhanced)
        └── services/
            └── spellcheck_service.rs
```

## Testing Strategy

### Unit Tests
- Custom node serialization/deserialization
- Plugin functionality
- State management actions
- Utility functions (word count, etc.)

### Integration Tests
- Editor + AutoSave + Database flow
- Keyboard shortcuts execution
- Clipboard operations
- Export/import round-trip

### E2E Tests
- Document creation and editing
- Multi-tab workflow
- Undo/redo across sessions
- Recovery from crash

## Performance Considerations

1. **Debounced AutoSave**: Prevent excessive database writes
2. **Virtual Scrolling**: For large document lists
3. **Lazy Loading**: Load document content on demand
4. **Memoization**: React.memo for expensive components
5. **Web Workers**: For heavy operations (spell check, export)

## Security

- All data stored locally
- No external API calls without user consent
- Input sanitization for pasted content
- File path validation

## Success Criteria

- [ ] Lexical editor renders and accepts input
- [ ] All custom nodes render correctly
- [ ] AutoSave saves to database every 30 seconds
- [ ] Keyboard shortcuts work (Ctrl+S, Ctrl+B, Ctrl+I, Ctrl+U)
- [ ] Undo/redo functions properly
- [ ] Ribbon toolbar updates based on selection
- [ ] Word/character count updates in real-time
- [ ] Images can be inserted and resized
- [ ] Tables can be created and edited
- [ ] Documents can be opened in multiple tabs

## Timeline

- **Week 1-2**: Lexical core setup, basic plugins
- **Week 3-4**: Custom nodes (Image, Table)
- **Week 5-6**: Ribbon UI implementation
- **Week 7-8**: Polish, testing, bug fixes

## Dependencies

### Frontend (NPM)
Already defined in package.json:
- lexical + @lexical/* plugins
- zustand (state management)
- react-query (data fetching)
- tailwindcss (styling)

### Backend (Rust)
Already defined in Cargo.toml:
- Existing dependencies sufficient for Phase 4

## Next Steps (Phase 5)

After completing Phase 4:
- Advanced export formats (PDF, DOCX)
- Import from external formats
- Track changes feature
- Comments and annotations
- Templates system

---

*This document outlines the Phase 4 implementation plan for Editor Integration. All code should follow the architecture principles defined in ARCHITECTURE.md.*
