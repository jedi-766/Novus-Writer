import React from 'react';

interface AppShellProps {
  children: React.ReactNode;
  ribbon?: React.ReactNode;
  sidebar?: React.ReactNode;
  statusBar?: React.ReactNode;
}

export function AppShell({ children, ribbon, sidebar, statusBar }: AppShellProps) {
  return (
    <div className="app-shell">
      {ribbon && <header className="app-header">{ribbon}</header>}
      <div className="app-body">
        {sidebar && <aside className="app-sidebar">{sidebar}</aside>}
        <main className="app-main">{children}</main>
      </div>
      {statusBar && <footer className="app-footer">{statusBar}</footer>}
    </div>
  );
}
