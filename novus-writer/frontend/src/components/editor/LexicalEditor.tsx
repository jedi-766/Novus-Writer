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
}

const editorNodes = [ImageNode, TableNode, HorizontalRuleNode, CodeNode];

export function LexicalEditorComponent({ documentId, initialContent }: LexicalEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const { activeDocument } = useEditorStore();

  const handleEditorChange = (editorState: any) => {
    // Handle editor state changes if needed
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
          <WordCountPlugin />
          <ToolbarSyncPlugin />
          {documentId && <AutoSavePlugin documentId={documentId} interval={30000} />}
          <OnChangePlugin onChange={handleEditorChange} />
        </div>
      </LexicalComposer>
    </div>
  );
}
