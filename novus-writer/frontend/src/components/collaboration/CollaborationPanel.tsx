import React, { useState, useEffect } from 'react';
import { Users, UserPlus, UserMinus, Cursor, Wifi, WifiOff } from 'lucide-react';

interface ConnectedUser {
  id: string;
  username: string;
  color: string;
  cursorPosition?: {
    offset: number;
    selectionStart?: number;
    selectionEnd?: number;
  };
  lastActivity: string;
}

interface CollaborationPanelProps {
  documentId: string;
  isCollaborating: boolean;
  onToggleCollaboration: () => void;
}

const CollaborationPanel: React.FC<CollaborationPanelProps> = ({
  documentId,
  isCollaborating,
  onToggleCollaboration,
}) => {
  const [connectedUsers, setConnectedUsers] = useState<ConnectedUser[]>([]);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);

  useEffect(() => {
    if (isCollaborating && sessionId) {
      // Simulate fetching connected users - in real app, use WebSocket
      const fetchUsers = async () => {
        try {
          const response = await fetch(`/api/collaboration/${sessionId}/users`);
          if (response.ok) {
            const data = await response.json();
            setConnectedUsers(data);
          }
        } catch (error) {
          console.error('Failed to fetch users:', error);
        }
      };

      const interval = setInterval(fetchUsers, 3000);
      return () => clearInterval(interval);
    }
  }, [isCollaborating, sessionId]);

  const startCollaboration = async () => {
    setIsConnecting(true);
    try {
      const response = await fetch('/api/collaboration/sessions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ document_id: documentId }),
      });

      if (response.ok) {
        const data = await response.json();
        setSessionId(data.id);
        onToggleCollaboration();
      }
    } catch (error) {
      console.error('Failed to start collaboration:', error);
    } finally {
      setIsConnecting(false);
    }
  };

  const stopCollaboration = async () => {
    if (sessionId) {
      try {
        await fetch(`/api/collaboration/sessions/${sessionId}`, {
          method: 'DELETE',
        });
      } catch (error) {
        console.error('Failed to stop collaboration:', error);
      }
    }
    setSessionId(null);
    setConnectedUsers([]);
    onToggleCollaboration();
  };

  const copyInviteLink = () => {
    if (sessionId) {
      const link = `${window.location.origin}/collaborate/${sessionId}`;
      navigator.clipboard.writeText(link);
      alert('Invite link copied to clipboard!');
    }
  };

  if (!isCollaborating) {
    return (
      <div className="p-4 bg-blue-50 rounded-lg">
        <h3 className="text-lg font-semibold text-blue-900 mb-2 flex items-center gap-2">
          <Wifi className="w-5 h-5" />
          Real-time Collaboration
        </h3>
        <p className="text-sm text-blue-700 mb-3">
          Enable real-time collaboration to work with others simultaneously on this document.
        </p>
        <button
          onClick={startCollaboration}
          disabled={isConnecting}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
        >
          {isConnecting ? (
            <>
              <span className="animate-spin">⏳</span>
              Connecting...
            </>
          ) : (
            <>
              <UserPlus className="w-4 h-4" />
              Start Collaboration
            </>
          )}
        </button>
      </div>
    );
  }

  return (
    <div className="p-4 bg-green-50 rounded-lg">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-green-900 flex items-center gap-2">
          <Wifi className="w-5 h-5 text-green-600" />
          Collaboration Active
        </h3>
        <button
          onClick={stopCollaboration}
          className="text-red-600 hover:text-red-800 text-sm flex items-center gap-1"
        >
          <WifiOff className="w-4 h-4" />
          Stop
        </button>
      </div>

      <div className="mb-4">
        <button
          onClick={copyInviteLink}
          className="w-full px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 text-sm"
        >
          Copy Invite Link
        </button>
      </div>

      <div className="border-t border-green-200 pt-3">
        <h4 className="text-sm font-medium text-green-800 mb-2 flex items-center gap-2">
          <Users className="w-4 h-4" />
          Connected Users ({connectedUsers.length + 1})
        </h4>
        
        <div className="space-y-2">
          {/* Current user */}
          <div className="flex items-center gap-2 p-2 bg-white rounded-md">
            <div className="w-3 h-3 rounded-full bg-blue-500"></div>
            <span className="text-sm text-gray-700">You</span>
          </div>
          
          {/* Other connected users */}
          {connectedUsers.map((user) => (
            <div
              key={user.id}
              className="flex items-center gap-2 p-2 bg-white rounded-md"
            >
              <div
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: user.color }}
              ></div>
              <span className="text-sm text-gray-700">{user.username}</span>
              {user.cursorPosition && (
                <Cursor className="w-3 h-3 text-gray-400 ml-auto" />
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="mt-4 p-3 bg-yellow-50 rounded-md border border-yellow-200">
        <p className="text-xs text-yellow-800">
          💡 Tip: Changes are synced automatically. Colored cursors show where others are editing.
        </p>
      </div>
    </div>
  );
};

export default CollaborationPanel;
