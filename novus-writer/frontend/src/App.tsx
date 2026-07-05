import React, { useState } from 'react';
import { AppShell } from './components/layout/AppShell';
import { Ribbon } from './components/ribbon/Ribbon';
import { Sidebar } from './components/sidebar/Sidebar';
import { StatusBar } from './components/statusbar/StatusBar';
import { EditorCanvas } from './components/editor/EditorCanvas';

export function App() {
  const [editor, setEditor] = useState<any>(null);

  return (
    <AppShell
      ribbon={<Ribbon editor={editor} />}
      sidebar={<Sidebar />}
      statusBar={<StatusBar />}
    >
      <EditorCanvas documentId="sample-doc-id" />
    </AppShell>
  );
}
