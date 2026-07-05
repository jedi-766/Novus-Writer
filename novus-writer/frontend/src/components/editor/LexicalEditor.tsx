import React, { useRef } from 'react';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin as LexicalHistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { LinkPlugin } from '@lexical/react/LexicalLinkPlugin';
import { ListPlugin } from '@lexical/react/LexicalListPlugin';
import { TablePlugin } from '@lexical/react/LexicalTablePlugin';
import { AutoSavePlugin } from '../../plugins/lexical/AutoSavePlugin';
import { KeyboardShortcutsPlugin } from '../../plugins/lexical/KeyboardShortcutsPlugin';
import { WordCountPlugin } from '../../plugins/lexical/WordCountPlugin';
import { SpellCheckPlugin } from '../../plugins/lexical/SpellCheckPlugin';
import { ToolbarSyncPlugin } from '../../plugins/lexical/ToolbarSyncPlugin';
import { ImageNode } from '../../plugins/nodes/ImageNode';
import { TableNode } from '../../plugins/nodes/TableNode';
import { HorizontalRuleNode } from '../../plugins/nodes/HorizontalRuleNode';
import { CodeNode } from '../../plugins/nodes/CodeNode';
import { defaultEditorConfig } from '../../types/editor';
import { useEditorStore } from '../../stores/editorStore';

interface LexicalEditorProps {
  documentId?: string;
  initialContent?: string;
  onContentChange?: (content: string, wordCount: number, charCount: number) => void;
  onEditorInit?: (editor: any) => void;
}

const editorNodes = [ImageNode, TableNode, HorizontalRuleNode, CodeNode];

export function LexicalEditorComponent({ 
  documentId, 
  initialContent,
  onContentChange,
  onEditorInit 
}: LexicalEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const { setDirty } = useEditorStore();

  const handleEditorChange = async (editorState: any, editor: any) => {
    if (setDirty) {
      setDirty(true);
    }
    
    if (onContentChange) {
      try {
        const content = JSON.stringify(editorState.toJSON());
        onContentChange(content, 0, 0);
      } catch (err) {
        console.error('Error serializing editor state:', err);
      }
    }
  };

  const handleEditorReady = (editor: any) => {
    if (onEditorInit) {
      onEditorInit(editor);
    }
  };

  const initialConfig = {
    ...defaultEditorConfig,
    nodes: editorNodes,
    editable: true,
  };

  return (
    <div className="lexical-editor-container" ref={containerRef}>
      <LexicalComposer initialConfig={initialConfig}>
        <div className="editor-wrapper">
          <RichTextPlugin
            contentEditable={<ContentEditable className="editor-content-editable" />}
            placeholder={<div className="editor-placeholder">Start typing...</div>}
            ErrorBoundary={({ error }) => <div>Error: {error}</div>}
          />
          <LexicalHistoryPlugin />
          <LinkPlugin />
          <ListPlugin />
          <TablePlugin />
          <KeyboardShortcutsPlugin />
          <SpellCheckPlugin />
          <WordCountPlugin onWordCountChange={onContentChange} />
          <ToolbarSyncPlugin />
          {documentId && <AutoSavePlugin documentId={parseInt(documentId, 10)} interval={30000} />}
          <OnChangePlugin onChange={handleEditorChange} onEditorReady={handleEditorReady} />
        </div>
      </LexicalComposer>
    </div>
  );
}
