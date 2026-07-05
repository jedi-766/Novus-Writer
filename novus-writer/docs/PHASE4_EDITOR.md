# Novus Writer - Phase 4: Editor Implementation

## Overview

Phase 4 implements the complete Lexical editor integration with custom nodes, plugins, and advanced editing features. This phase builds upon the completed database layer from Phase 3 to deliver a production-ready rich text editing experience.

## Goals

1. **Lexical Editor Core Integration**: Set up Lexical editor with React
2. **Custom Nodes**: Implement Image, Table, HorizontalRule, and other custom nodes
3. **Editor Plugins**: Build AutoSave, KeyboardShortcuts, History, and other plugins
4. **Clipboard Operations**: Full copy/paste/cut support with rich content
5. **Undo/Redo System**: Robust history management
6. **Spell Checking**: Integrated spell check functionality

## Architecture

### Frontend Editor Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Editor Component                       │
│  ┌─────────────────────────────────────────────────────┐│
│  │              LexicalContentEditable                  ││
│  │  ┌───────────────────────────────────────────────┐  ││
│  │  │            Custom Node Renderers               │  ││
│  │  │  • ImageNode    • TableNode    • HRNode       │  ││
│  │  │  • TableCell    • TableRow     • Mention      │  ││
│  │  └───────────────────────────────────────────────┘  ││
│  └─────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────┐│
│  │                 Editor Plugins                       ││
│  │  • AutoSavePlugin  • HistoryPlugin                   ││
│  │  • KeyboardPlugin  • SpellCheckPlugin                ││
│  │  • ClipboardPlugin • ToolbarPlugin                   ││
│  └─────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────┘
```

### Plugin Communication Flow

```
User Action → Lexical Command → Plugin Handler → State Update
                                      ↓
                              Tauri Command (if needed)
                                      ↓
                              Backend Service
                                      ↓
                              Database Operation
```

## File Structure

```
frontend/src/
├── components/
│   ├── editor/
│   │   ├── Editor.tsx                      # Main editor container
│   │   ├── EditorContent.tsx               # Lexical ContentEditable wrapper
│   │   ├── EditorToolbar.tsx               # Formatting toolbar
│   │   ├── FloatingFormatBar.tsx           # Contextual formatting bar
│   │   └── index.ts
│   ├── nodes/
│   │   ├── ImageNode.tsx                   # Custom image node component
│   │   ├── ImageNodeComponent.tsx          # Image node UI renderer
│   │   ├── TableNode.tsx                   # Custom table node
│   │   ├── TableNodeComponent.tsx          # Table UI renderer
│   │   ├── TableCellNode.tsx               # Table cell node
│   │   ├── TableRowNode.tsx                # Table row node
│   │   ├── HorizontalRuleNode.tsx          # Horizontal rule node
│   │   ├── HorizontalRuleComponent.tsx     # HR UI renderer
│   │   └── index.ts
│   └── plugins/
│       ├── AutoSavePlugin.tsx              # Auto-save functionality
│       ├── HistoryPlugin.tsx               # Undo/redo management
│       ├── KeyboardShortcutsPlugin.tsx     # Keyboard command handling
│       ├── SpellCheckPlugin.tsx            # Spell checking integration
│       ├── ClipboardPlugin.tsx             # Clipboard operations
│       ├── ToolbarPlugin.tsx               # Toolbar state sync
│       ├── ClickableLinkPlugin.tsx         # Link interaction
│       ├── TableCellResizerPlugin.tsx      # Table column resize
│       └── index.ts
├── plugins/
│   ├── lexical/
│   │   ├── nodes/
│   │   │   ├── ImageNode.ts                # Image node class definition
│   │   │   ├── TableNode.ts                # Table node class
│   │   │   ├── TableCellNode.ts            # Table cell node
│   │   │   ├── TableRowNode.ts             # Table row node
│   │   │   ├── HorizontalRuleNode.ts       # HR node class
│   │   │   └── index.ts
│   │   ├── transforms/
│   │   │   ├── image.ts                    # Image insertion transforms
│   │   │   ├── table.ts                    # Table manipulation transforms
│   │   │   └── index.ts
│   │   └── utils/
│   │       ├── nodeHelpers.ts              # Node utility functions
│   │       └── selection.ts                # Selection helpers
│   └── spellcheck/
│       ├── SpellChecker.ts                 # Spell check engine
│       ├── Dictionary.ts                   # Custom dictionary management
│       └── index.ts
├── stores/
│   ├── editorStore.ts                      # Editor state (Zustand)
│   └── index.ts
└── hooks/
    ├── useEditor.ts                        # Editor context hook
    ├── useSelection.ts                     # Selection state hook
    └── index.ts

src-tauri/src/
├── editor/
│   ├── mod.rs
│   ├── lexical_types.rs                    # Already exists
│   ├── editor_service.rs                   # NEW: Editor operations service
│   ├── clipboard.rs                        # NEW: Clipboard handling
│   └── history.rs                          # NEW: Undo/redo backend support
├── commands/
│   ├── editor.rs                           # Enhance existing
│   ├── clipboard.rs                        # NEW: Clipboard commands
│   └── spellcheck.rs                       # NEW: Spell check commands
├── services/
│   ├── editor_service.rs                   # NEW: Backend editor logic
│   └── spellcheck_service.rs               # NEW: Spell check service
└── utils/
    ├── html_conversion.rs                  # NEW: HTML ↔ Lexical JSON
    └── markdown_conversion.rs              # NEW: Markdown conversion
```

## Implementation Details

### 1. Custom Node Definitions

#### ImageNode

**Purpose**: Support embedded images with captions, resizing, and alignment.

**Frontend (plugins/lexical/nodes/ImageNode.ts)**:
```typescript
import { DecoratorNode, DOMConversionMap, DOMExportOutput, EditorConfig, LexicalEditor, LexicalNode, NodeKey, SerializedEditorState, Spread } from 'lexical';

export type ImageStatus = 'loading' | 'loaded' | 'error' | 'uploading';

export interface ImagePayload {
  altText: string;
  caption?: LexicalEditor;
  height?: number;
  key?: NodeKey;
  maxWidth?: number;
  showCaption?: boolean;
  src: string;
  width?: number;
  captionsEnabled?: boolean;
}

export interface SerializedImageNode extends Spread<{
  altText: string;
  caption: SerializedEditorState;
  height?: number;
  maxWidth?: number;
  showCaption: boolean;
  src: string;
  width?: number;
  status: ImageStatus;
}, SerializedLexicalNode> {
  type: 'image';
  version: 1;
}

export class ImageNode extends DecoratorNode<HTMLElement> {
  __altText: string;
  __caption: LexicalEditor;
  __height?: number;
  __maxWidth: number;
  __showCaption: boolean;
  __src: string;
  __width?: number;
  __status: ImageStatus;

  static getType(): string { return 'image'; }
  static clone(node: ImageNode): ImageNode { return new ImageNode(node); }
  
  // Implement required Lexical node methods
  createDOM(config: EditorConfig): HTMLElement { /* ... */ }
  updateDOM(): boolean { return false; }
  decorate(): HTMLElement { /* ... */ }
  
  // Serialization
  serialize(): SerializedImageNode { /* ... */ }
  static importJSON(serialized: SerializedImageNode): ImageNode { /* ... */ }
  
  // Mutators
  setAltText(altText: string): void { /* ... */ }
  setHeight(height: number): void { /* ... */ }
  setWidth(width: number): void { /* ... */ }
  setStatus(status: ImageStatus): void { /* ... */ }
}
```

**Backend Support (src-tauri/src/editor/image_node.rs)**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageNodeData {
    pub alt_text: String,
    pub src: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub caption: Option<String>,
    pub status: String, // "loading", "loaded", "error", "uploading"
}

impl ImageNodeData {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.src.is_empty() {
            return Err(AppError::ValidationError("Image source cannot be empty".to_string()));
        }
        Ok(())
    }
    
    pub fn is_data_uri(&self) -> bool {
        self.src.starts_with("data:")
    }
}
```

#### TableNode

**Purpose**: Full-featured table support with cell merging, resizing, and formatting.

**Frontend (plugins/lexical/nodes/TableNode.ts)**:
```typescript
import { ElementNode, DOMExportOutput, EditorConfig, LexicalNode, NodeKey, SerializedElementNode } from 'lexical';
import { TableCellNode } from './TableCellNode';
import { TableRowNode } from './TableRowNode';

export interface TableGridSize {
  rows: number;
  columns: number;
}

export interface SerializedTableNode extends SerializedElementNode {
  type: 'table';
  version: 1;
  colWidths?: number[];
  hasHeader?: boolean;
}

export class TableNode extends ElementNode {
  __colWidths?: number[];
  __hasHeader: boolean;

  static getType(): string { return 'table'; }
  static clone(node: TableNode): TableNode { return new TableNode(node); }
  
  createDOM(config: EditorConfig): HTMLElement {
    const table = document.createElement('table');
    table.setAttribute('data-lexical-table', 'true');
    return table;
  }
  
  insertRow(index: number): void { /* ... */ }
  insertColumn(index: number): void { /* ... */ }
  deleteRow(index: number): void { /* ... */ }
  deleteColumn(index: number): void { /* ... */ }
  mergeCells(cells: Array<[number, number]>): void { /* ... */ }
  setColWidth(col: number, width: number): void { /* ... */ }
  
  serialize(): SerializedTableNode { /* ... */ }
  static importJSON(serialized: SerializedTableNode): TableNode { /* ... */ }
}
```

**Backend Support (src-tauri/src/editor/table_node.rs)**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNodeData {
    pub rows: usize,
    pub cols: usize,
    pub col_widths: Vec<u32>,
    pub has_header: bool,
    pub cells: Vec<Vec<TableCellData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCellData {
    pub content: String,
    pub col_span: u32,
    pub row_span: u32,
    pub is_header: bool,
    pub alignment: CellAlignment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellAlignment {
    Left,
    Center,
    Right,
    Justify,
}
```

#### HorizontalRuleNode

**Purpose**: Visual section separator.

**Frontend (plugins/lexical/nodes/HorizontalRuleNode.ts)**:
```typescript
import { DecoratorNode, DOMConversionMap, EditorConfig, LexicalNode, NodeKey, SerializedLexicalNode } from 'lexical';

export interface SerializedHorizontalRuleNode extends SerializedLexicalNode {
  type: 'horizontalrule';
  version: 1;
}

export class HorizontalRuleNode extends DecoratorNode<HTMLElement> {
  static getType(): string { return 'horizontalrule'; }
  static clone(node: HorizontalRuleNode): HorizontalRuleNode { return new HorizontalRuleNode(node); }
  
  createDOM(config: EditorConfig): HTMLElement {
    const hr = document.createElement('hr');
    hr.setAttribute('data-lexical-hr', 'true');
    return hr;
  }
  
  updateDOM(): boolean { return false; }
  decorate(): HTMLElement {
    const div = document.createElement('div');
    div.innerHTML = '<hr data-lexical-hr="true" />';
    return div;
  }
  
  serialize(): SerializedHorizontalRuleNode {
    return { type: 'horizontalrule', version: 1 };
  }
  
  static importJSON(): HorizontalRuleNode {
    return new HorizontalRuleNode();
  }
}
```

### 2. Editor Plugins

#### AutoSavePlugin

**Purpose**: Automatically save document changes at configurable intervals.

**Frontend (components/plugins/AutoSavePlugin.tsx)**:
```typescript
import { useEffect, useRef } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getRoot, EditorState } from 'lexical';
import { invoke } from '@tauri-apps/api/core';
import { useDebounce } from '../../hooks/useDebounce';

interface AutoSavePluginProps {
  documentId: string;
  intervalMs?: number;
  onAutosave?: (state: EditorState) => void;
}

export function AutoSavePlugin({ documentId, intervalMs = 5000, onAutosave }: AutoSavePluginProps) {
  const [editor] = useLexicalComposerContext();
  const lastSavedRef = useRef<string>('');
  
  const saveToBackend = useDebounce(async (editorState: EditorState) => {
    editorState.read(() => {
      const root = $getRoot();
      const content = JSON.stringify(root);
      
      if (content !== lastSavedRef.current) {
        invoke('save_autosave', { documentId, content })
          .then(() => {
            lastSavedRef.current = content;
            console.log('Auto-saved successfully');
          })
          .catch(err => console.error('Auto-save failed:', err));
      }
    });
    
    onAutosave?.(editorState);
  }, intervalMs);
  
  useEffect(() => {
    const unregister = editor.registerUpdateListener(({ editorState }) => {
      saveToBackend(editorState);
    });
    
    return () => {
      unregister();
      saveToBackend.cancel();
    };
  }, [editor, documentId, saveToBackend]);
  
  return null;
}
```

#### HistoryPlugin

**Purpose**: Enhanced undo/redo with visual history panel.

**Frontend (components/plugins/HistoryPlugin.tsx)**:
```typescript
import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { HISTORY_NODE, HistoryNode } from '@lexical/history';
import { EditorState } from 'lexical';

interface HistoryEntry {
  timestamp: number;
  description: string;
  state: EditorState;
}

interface HistoryPluginProps {
  maxHistorySize?: number;
  onHistoryChange?: (canUndo: boolean, canRedo: boolean) => void;
}

export function HistoryPlugin({ maxHistorySize = 100, onHistoryChange }: HistoryPluginProps) {
  const [editor] = useLexicalComposerContext();
  
  useEffect(() => {
    const historyNode = editor._nodes.get(HISTORY_NODE);
    if (!historyNode) return;
    
    const unsubscribe = editor.registerUpdateListener(({ tags }) => {
      if (tags.has('historic')) {
        const canUndo = editor.canUndo();
        const canRedo = editor.canRedo();
        onHistoryChange?.(canUndo, canRedo);
      }
    });
    
    return unsubscribe;
  }, [editor, onHistoryChange]);
  
  return null;
}
```

#### KeyboardShortcutsPlugin

**Purpose**: Handle keyboard shortcuts for formatting and navigation.

**Frontend (components/plugins/KeyboardShortcutsPlugin.tsx)**:
```typescript
import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import {
  COMMAND_PRIORITY_HIGH,
  KEY_ENTER_COMMAND,
  KEY_TAB_COMMAND,
  createCommand,
} from 'lexical';
import { INSERT_TABLE_COMMAND } from '../nodes/TableNode';
import { INSERT_IMAGE_COMMAND } from '../nodes/ImageNode';

export const SAVE_DOCUMENT_COMMAND = createCommand('save_document');
export const FORMAT_BOLD_COMMAND = createCommand('format_bold');
export const FORMAT_ITALIC_COMMAND = createCommand('format_italic');

interface KeyboardShortcutsPluginProps {
  onSave?: () => void;
}

export function KeyboardShortcutsPlugin({ onSave }: KeyboardShortcutsPluginProps) {
  const [editor] = useLexicalComposerContext();
  
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Ctrl/Cmd + S: Save
      if ((event.ctrlKey || event.metaKey) && event.key === 's') {
        event.preventDefault();
        editor.dispatchCommand(SAVE_DOCUMENT_COMMAND, undefined);
        onSave?.();
        return true;
      }
      
      // Ctrl/Cmd + B: Bold
      if ((event.ctrlKey || event.metaKey) && event.key === 'b') {
        event.preventDefault();
        editor.dispatchCommand(FORMAT_BOLD_COMMAND, undefined);
        return true;
      }
      
      // Ctrl/Cmd + I: Italic
      if ((event.ctrlKey || event.metaKey) && event.key === 'i') {
        event.preventDefault();
        editor.dispatchCommand(FORMAT_ITALIC_COMMAND, undefined);
        return true;
      }
      
      // Ctrl/Cmd + Alt + T: Insert table
      if ((event.ctrlKey || event.metaKey) && event.altKey && event.key === 't') {
        event.preventDefault();
        editor.dispatchCommand(INSERT_TABLE_COMMAND, { rows: 3, columns: 3 });
        return true;
      }
      
      // Ctrl/Cmd + Alt + I: Insert image
      if ((event.ctrlKey || event.metaKey) && event.altKey && event.key === 'i') {
        event.preventDefault();
        editor.dispatchCommand(INSERT_IMAGE_COMMAND, {});
        return true;
      }
      
      return false;
    };
    
    const removeListener = editor.registerCommand(
      KEY_ENTER_COMMAND,
      (event) => {
        if (event?.shiftKey) {
          // Shift+Enter: Soft break
          return true;
        }
        return false;
      },
      COMMAND_PRIORITY_HIGH,
    );
    
    window.addEventListener('keydown', handleKeyDown);
    
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      removeListener();
    };
  }, [editor, onSave]);
  
  return null;
}
```

#### SpellCheckPlugin

**Purpose**: Real-time spell checking with custom dictionary support.

**Frontend (components/plugins/SpellCheckPlugin.tsx)**:
```typescript
import { useEffect, useState } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { TextNode } from 'lexical';
import { invoke } from '@tauri-apps/api/core';

interface MisspelledWord {
  word: string;
  position: number;
  suggestions: string[];
}

interface SpellCheckPluginProps {
  enabled?: boolean;
  language?: string;
  onMisspelledWords?: (words: MisspelledWord[]) => void;
}

export function SpellCheckPlugin({ enabled = true, language = 'en-US', onMisspelledWords }: SpellCheckPluginProps) {
  const [editor] = useLexicalComposerContext();
  const [misspelledWords, setMisspelledWords] = useState<MisspelledWord[]>([]);
  
  useEffect(() => {
    if (!enabled) return;
    
    const checkSpelling = async (text: string) => {
      try {
        const result = await invoke<MisspelledWord[]>('check_spelling', { text, language });
        setMisspelledWords(result);
        onMisspelledWords?.(result);
        
        // Highlight misspelled words in editor
        highlightMisspelledWords(editor, result);
      } catch (err) {
        console.error('Spell check failed:', err);
      }
    };
    
    const unregister = editor.registerUpdateListener(({ editorState }) => {
      editorState.read(() => {
        const root = $getRoot();
        const text = root.getTextContent();
        checkSpelling(text);
      });
    });
    
    return unregister;
  }, [editor, enabled, language, onMisspelledWords]);
  
  return null;
}

function highlightMisspelledWords(editor: any, words: MisspelledWord[]) {
  editor.update(() => {
    // Implementation for highlighting
  });
}
```

#### ClipboardPlugin

**Purpose**: Advanced clipboard operations with rich content support.

**Frontend (components/plugins/ClipboardPlugin.tsx)**:
```typescript
import { useEffect } from 'react';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import {
  COMMAND_PRIORITY_CRITICAL,
  COPY_COMMAND,
  CUT_COMMAND,
  PASTE_COMMAND,
  CommandPayloadType,
} from 'lexical';
import { invoke } from '@tauri-apps/api/core';

interface ClipboardPluginProps {
  onPaste?: (data: string, type: string) => void;
  onCopy?: (data: string) => void;
}

export function ClipboardPlugin({ onPaste, onCopy }: ClipboardPluginProps) {
  const [editor] = useLexicalComposerContext();
  
  useEffect(() => {
    const handlePaste = async (event: ClipboardEvent) => {
      event.preventDefault();
      
      const items = event.clipboardData?.items;
      if (!items) return;
      
      for (const item of items) {
        if (item.type.startsWith('image/')) {
          // Handle image paste
          const blob = item.getAsFile();
          if (blob) {
            const reader = new FileReader();
            reader.onload = async (e) => {
              const base64 = e.target?.result as string;
              await invoke('insert_image_from_clipboard', { imageData: base64 });
            };
            reader.readAsDataURL(blob);
          }
          return;
        }
        
        if (item.type === 'text/html') {
          item.getAsString(async (html) => {
            const result = await invoke('convert_html_to_lexical', { html });
            editor.parseEditorState(result);
            onPaste?.(html, 'html');
          });
          return;
        }
        
        if (item.type === 'text/plain') {
          item.getAsString((text) => {
            editor.update(() => {
              const selection = $getSelection();
              if (selection) {
                selection.insertText(text);
              }
            });
            onPaste?.(text, 'text');
          });
          return;
        }
      }
    };
    
    const handleCopy = (event: ClipboardEvent) => {
      event.preventDefault();
      
      editor.getEditorState().read(() => {
        const selection = $getSelection();
        if (selection) {
          const html = selection.getTextContent();
          event.clipboardData.setData('text/plain', html);
          event.clipboardData.setData('text/html', `<div>${html}</div>`);
          onCopy?.(html);
        }
      });
    };
    
    const rootElement = editor.getRootElement();
    if (rootElement) {
      rootElement.addEventListener('paste', handlePaste);
      rootElement.addEventListener('copy', handleCopy);
    }
    
    return () => {
      if (rootElement) {
        rootElement.removeEventListener('paste', handlePaste);
        rootElement.removeEventListener('copy', handleCopy);
      }
    };
  }, [editor, onPaste, onCopy]);
  
  return null;
}
```

### 3. Backend Services

#### Editor Service (src-tauri/src/services/editor_service.rs)

```rust
use crate::core::domain::document::Document;
use crate::editor::lexical_types::EditorState;
use crate::models::error::AppError;
use crate::utils::compression;
use sqlx::SqlitePool;
use tracing::{info, warn};

pub struct EditorService {
    db_pool: SqlitePool,
}

impl EditorService {
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
    
    /// Process and validate editor state before saving
    pub async fn process_editor_state(&self, state: &EditorState) -> Result<(), AppError> {
        // Validate structure
        self.validate_editor_structure(state)?;
        
        // Extract and count words
        let word_count = state.word_count();
        let char_count = state.char_count();
        
        info!("Processed editor state: {} words, {} characters", word_count, char_count);
        
        Ok(())
    }
    
    /// Convert HTML to Lexical JSON
    pub fn html_to_lexical(&self, html: &str) -> Result<EditorState, AppError> {
        // Use simplified HTML parser or external library
        // For now, create basic paragraph structure
        let state = EditorState {
            root: crate::editor::lexical_types::EditorNode {
                node_type: "root".to_string(),
                children: vec![
                    crate::editor::lexical_types::EditorChild::Element(
                        crate::editor::lexical_types::ElementNode {
                            node_type: "paragraph".to_string(),
                            children: vec![],
                            direction: None,
                            format: "".to_string(),
                            indent: 0,
                            version: 1,
                        }
                    )
                ],
                direction: None,
                format: "".to_string(),
                indent: 0,
                version: 1,
            },
        };
        
        Ok(state)
    }
    
    /// Convert Lexical JSON to HTML
    pub fn lexical_to_html(&self, state: &EditorState) -> Result<String, AppError> {
        let mut html = String::new();
        self.node_to_html(&state.root, &mut html)?;
        Ok(html)
    }
    
    fn node_to_html(&self, node: &crate::editor::lexical_types::EditorNode, html: &mut String) -> Result<(), AppError> {
        match node.node_type.as_str() {
            "paragraph" => {
                html.push_str("<p>");
                for child in &node.children {
                    self.child_to_html(child, html)?;
                }
                html.push_str("</p>");
            }
            "heading" => {
                html.push_str("<h1>");
                for child in &node.children {
                    self.child_to_html(child, html)?;
                }
                html.push_str("</h1>");
            }
            _ => {
                for child in &node.children {
                    self.child_to_html(child, html)?;
                }
            }
        }
        Ok(())
    }
    
    fn child_to_html(&self, child: &crate::editor::lexical_types::EditorChild, html: &mut String) -> Result<(), AppError> {
        match child {
            crate::editor::lexical_types::EditorChild::Text(text_node) => {
                html.push_str(&text_node.text);
            }
            crate::editor::lexical_types::EditorChild::Node(node) | 
            crate::editor::lexical_types::EditorChild::Element(node) => {
                self.node_to_html(node, html)?;
            }
        }
        Ok(())
    }
    
    fn validate_editor_structure(&self, state: &EditorState) -> Result<(), AppError> {
        if state.root.node_type != "root" {
            return Err(AppError::ValidationError("Invalid root node type".to_string()));
        }
        Ok(())
    }
}
```

#### Spell Check Service (src-tauri/src/services/spellcheck_service.rs)

```rust
use crate::models::error::AppError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MisspelledWord {
    pub word: String,
    pub position: i64,
    pub suggestions: Vec<String>,
}

pub struct SpellCheckService {
    dictionary: HashSet<String>,
    custom_words: HashSet<String>,
    word_pattern: Regex,
}

impl SpellCheckService {
    pub fn new() -> Self {
        let mut dictionary = HashSet::new();
        // Load basic English dictionary
        Self::load_basic_dictionary(&mut dictionary);
        
        Self {
            dictionary,
            custom_words: HashSet::new(),
            word_pattern: Regex::new(r"\b[a-zA-Z]+\b").unwrap(),
        }
    }
    
    pub fn add_custom_word(&mut self, word: String) {
        self.custom_words.insert(word.to_lowercase());
        info!("Added custom word: {}", word);
    }
    
    pub fn load_user_dictionary(&mut self, words: Vec<String>) {
        for word in words {
            self.custom_words.insert(word.to_lowercase());
        }
        info!("Loaded {} user dictionary words", self.custom_words.len());
    }
    
    pub fn check_text(&self, text: &str) -> Vec<MisspelledWord> {
        let mut misspelled = Vec::new();
        
        for mat in self.word_pattern.find_iter(text) {
            let word = mat.as_str();
            let lower = word.to_lowercase();
            
            if !self.dictionary.contains(&lower) && !self.custom_words.contains(&lower) {
                misspelled.push(MisspelledWord {
                    word: word.to_string(),
                    position: mat.start() as i64,
                    suggestions: self.generate_suggestions(&lower),
                });
            }
        }
        
        misspelled
    }
    
    fn generate_suggestions(&self, word: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Simple suggestion algorithm based on edit distance
        for dict_word in &self.dictionary {
            if self.edit_distance(word, dict_word) <= 2 {
                suggestions.push(dict_word.clone());
                if suggestions.len() >= 5 {
                    break;
                }
            }
        }
        
        suggestions
    }
    
    fn edit_distance(&self, s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        
        let mut matrix = vec![vec![0; s2_chars.len() + 1]; s1_chars.len() + 1];
        
        for i in 0..=s1_chars.len() {
            matrix[i][0] = i;
        }
        for j in 0..=s2_chars.len() {
            matrix[0][j] = j;
        }
        
        for i in 1..=s1_chars.len() {
            for j in 1..=s2_chars.len() {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
        
        matrix[s1_chars.len()][s2_chars.len()]
    }
    
    fn load_basic_dictionary(dictionary: &mut HashSet<String>) {
        // Common English words - in production, load from file
        let common_words = vec![
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "it",
            "for", "not", "on", "with", "he", "as", "you", "do", "at", "this",
            "but", "his", "by", "from", "they", "we", "say", "her", "she", "or",
            // ... more words
        ];
        
        for word in common_words {
            dictionary.insert(word.to_string());
        }
    }
}

impl Default for SpellCheckService {
    fn default() -> Self {
        Self::new()
    }
}
```

### 4. Tauri Commands

#### Clipboard Commands (src-tauri/src/commands/clipboard.rs)

```rust
use crate::editor::lexical_types::EditorState;
use crate::services::editor_service::EditorService;
use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tauri::command]
pub async fn convert_html_to_lexical(
    html: String,
    editor_service: State<'_, Arc<Mutex<EditorService>>>,
) -> Result<String, String> {
    let service = editor_service.lock().await;
    match service.html_to_lexical(&html) {
        Ok(state) => state.to_json().map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn convert_lexical_to_html(
    lexical_json: String,
    editor_service: State<'_, Arc<Mutex<EditorService>>>,
) -> Result<String, String> {
    let state = EditorState::from_json(&lexical_json).map_err(|e| e.to_string())?;
    let service = editor_service.lock().await;
    service.lexical_to_html(&state).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn insert_image_from_clipboard(
    image_data: String,
    // Handle image storage and return image node JSON
) -> Result<String, String> {
    // Implementation for handling clipboard images
    Ok(image_data) // Placeholder
}
```

#### Spell Check Commands (src-tauri/src/commands/spellcheck.rs)

```rust
use crate::services::spellcheck_service::{SpellCheckService, MisspelledWord};
use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tauri::command]
pub async fn check_spelling(
    text: String,
    language: String,
    spellcheck_service: State<'_, Arc<Mutex<SpellCheckService>>>,
) -> Result<Vec<MisspelledWord>, String> {
    let service = spellcheck_service.lock().await;
    Ok(service.check_text(&text))
}

#[tauri::command]
pub async fn add_to_dictionary(
    word: String,
    spellcheck_service: State<'_, Arc<Mutex<SpellCheckService>>>,
) -> Result<(), String> {
    let mut service = spellcheck_service.lock().await;
    service.add_custom_word(word);
    Ok(())
}

#[tauri::command]
pub async fn load_user_dictionary(
    words: Vec<String>,
    spellcheck_service: State<'_, Arc<Mutex<SpellCheckService>>>,
) -> Result<(), String> {
    let mut service = spellcheck_service.lock().await;
    service.load_user_dictionary(words);
    Ok(())
}
```

### 5. Editor Store (Zustand)

**Frontend (stores/editorStore.ts)**:

```typescript
import { create } from 'zustand';
import { EditorState, LexicalEditor } from 'lexical';

interface EditorState {
  activeEditor: LexicalEditor | null;
  isEditable: boolean;
  wordCount: number;
  charCount: number;
  selectionFormat: Record<string, any>;
  canUndo: boolean;
  canRedo: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  setActiveEditor: (editor: LexicalEditor | null) => void;
  setIsEditable: (editable: boolean) => void;
  setWordCount: (count: number) => void;
  setCharCount: (count: number) => void;
  setSelectionFormat: (format: Record<string, any>) => void;
  setCanUndo: (can: boolean) => void;
  setCanRedo: (can: boolean) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  reset: () => void;
}

export const useEditorStore = create<EditorState>((set) => ({
  // State
  activeEditor: null,
  isEditable: true,
  wordCount: 0,
  charCount: 0,
  selectionFormat: {},
  canUndo: false,
  canRedo: false,
  isLoading: false,
  error: null,
  
  // Actions
  setActiveEditor: (editor) => set({ activeEditor: editor }),
  setIsEditable: (editable) => set({ isEditable: editable }),
  setWordCount: (count) => set({ wordCount: count }),
  setCharCount: (count) => set({ charCount: count }),
  setSelectionFormat: (format) => set({ selectionFormat: format }),
  setCanUndo: (can) => set({ canUndo: can }),
  setCanRedo: (can) => set({ canRedo: can }),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error }),
  reset: () => set({
    activeEditor: null,
    isEditable: true,
    wordCount: 0,
    charCount: 0,
    selectionFormat: {},
    canUndo: false,
    canRedo: false,
    isLoading: false,
    error: null,
  }),
}));
```

## Testing Strategy

### Unit Tests

**Custom Node Tests**:
```typescript
describe('ImageNode', () => {
  it('should create node with valid payload', () => {
    const node = new ImageNode({
      altText: 'Test image',
      src: 'test.png',
      width: 100,
      height: 100,
    });
    
    expect(node.getType()).toBe('image');
    expect(node.getAltText()).toBe('Test image');
  });
  
  it('should serialize and deserialize correctly', () => {
    const node = new ImageNode({ altText: 'Test', src: 'test.png' });
    const serialized = node.serialize();
    const imported = ImageNode.importJSON(serialized);
    
    expect(imported.getAltText()).toBe(node.getAltText());
    expect(imported.getSrc()).toBe(node.getSrc());
  });
});
```

**Plugin Tests**:
```typescript
describe('AutoSavePlugin', () => {
  it('should trigger autosave after interval', async () => {
    const mockSave = jest.fn();
    render(<AutoSavePlugin documentId="test" intervalMs={100} onAutosave={mockSave} />);
    
    // Simulate editor update
    act(() => {
      // Trigger editor change
    });
    
    await waitFor(() => {
      expect(mockSave).toHaveBeenCalled();
    });
  });
});
```

### Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_spellcheck_service() {
        let mut service = SpellCheckService::new();
        let result = service.check_text("Ths is a test with mispelled words");
        
        assert!(result.iter().any(|w| w.word == "Ths"));
        assert!(result.iter().any(|w| w.word == "mispelled"));
    }
    
    #[tokio::test]
    async fn test_editor_service_conversion() {
        let service = EditorService::new(pool);
        let html = "<p>Hello world</p>";
        let state = service.html_to_lexical(html).unwrap();
        
        assert_eq!(state.root.node_type, "root");
    }
}
```

## Performance Considerations

1. **Node Rendering**: Use React.memo for custom node components
2. **Plugin Efficiency**: Debounce expensive operations (autosave, spell check)
3. **Memory Management**: Clear history on large paste operations
4. **Lazy Loading**: Load heavy plugins only when needed
5. **Virtual Scrolling**: For large tables and lists

## Accessibility

- All custom nodes must support ARIA labels
- Keyboard navigation for table cells and images
- Screen reader announcements for formatting changes
- Focus management for dialogs and popups

## Migration from Phase 3

No database schema changes required. Phase 4 builds on top of Phase 3's database layer.

## Success Criteria

- [ ] All custom nodes implemented and tested
- [ ] All plugins functional with proper error handling
- [ ] Clipboard operations work across platforms
- [ ] Undo/redo history works correctly
- [ ] Spell checking identifies and suggests corrections
- [ ] Editor performance meets benchmarks (<16ms frame time)
- [ ] Full keyboard accessibility
- [ ] All unit and integration tests passing

## Next Steps (Phase 5)

After completing Phase 4, Phase 5 will focus on:
- Export functionality (PDF, DOCX, Markdown, HTML)
- Import functionality
- Document templates
- Advanced search and replace
- Print preview and printing
