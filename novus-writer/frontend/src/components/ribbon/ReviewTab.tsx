import React, { useState } from 'react';
import { LexicalEditor } from 'lexical';
import { $getSelection, $isRangeSelection } from 'lexical';

interface ReviewTabProps {
  editor: LexicalEditor;
}

export function ReviewTab({ editor }: ReviewTabProps) {
  const [showComments, setShowComments] = useState(true);
  const [trackChangesEnabled, setTrackChangesEnabled] = useState(false);

  const toggleSpellCheck = () => {
    // Toggle browser spellcheck
    const editorElement = editor.getRootElement();
    if (editorElement) {
      editorElement.spellcheck = !editorElement.spellcheck;
      alert(`Spell check ${editorElement.spellcheck ? 'enabled' : 'disabled'}`);
    }
  };

  const toggleThesaurus = () => {
    editor.getEditorState().read(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        const selectedText = selection.getTextContent();
        if (selectedText) {
          // In a real implementation, this would call a thesaurus API
          alert(`Thesaurus for: "${selectedText}"\n(Simulation - would show synonyms)`);
        } else {
          alert('Please select a word to look up in thesaurus');
        }
      }
    });
  };

  const toggleWordCount = () => {
    // Word count is already shown in status bar
    alert('Word count is displayed in the status bar at the bottom of the window.');
  };

  const addComment = () => {
    editor.getEditorState().read(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        const selectedText = selection.getTextContent();
        if (selectedText) {
          const comment = prompt('Enter your comment:');
          if (comment) {
            console.log('Comment added:', { text: selectedText, comment });
            alert(`Comment added:\n"${comment}"\nfor text: "${selectedText.substring(0, 50)}..."`);
          }
        } else {
          alert('Please select text to add a comment');
        }
      }
    });
  };

  return (
    <div className="ribbon-tab-content review-tab">
      {/* Proofing Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={toggleSpellCheck}
          title="Toggle Spell Check"
        >
          ✓ Spelling
        </button>
        <button 
          className="ribbon-btn" 
          onClick={toggleThesaurus}
          title="Thesaurus"
        >
          📖 Thesaurus
        </button>
        <button 
          className="ribbon-btn" 
          onClick={toggleWordCount}
          title="Word Count"
        >
          📊 Word Count
        </button>
      </div>

      {/* Comments Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={addComment}
          title="Add Comment"
        >
          💬 New Comment
        </button>
        <button 
          className={`ribbon-btn ${showComments ? 'active' : ''}`} 
          onClick={() => setShowComments(!showComments)}
          title="Show/Hide Comments"
        >
          👁️ Show Comments
        </button>
        <button 
          className="ribbon-btn" 
          title="Delete Comment"
        >
          🗑️ Delete
        </button>
      </div>

      {/* Tracking Group */}
      <div className="ribbon-group">
        <button 
          className={`ribbon-btn ${trackChangesEnabled ? 'active' : ''}`} 
          onClick={() => setTrackChangesEnabled(!trackChangesEnabled)}
          title="Track Changes"
        >
          📝 Track Changes
        </button>
        <button 
          className="ribbon-btn" 
          disabled={!trackChangesEnabled}
          title="Show Markup"
        >
          👁️ Show Markup
        </button>
        <button 
          className="ribbon-btn" 
          disabled={!trackChangesEnabled}
          title="Reviewing Pane"
        >
          📋 Review Pane
        </button>
      </div>

      {/* Compare Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          title="Compare Documents"
        >
          ⚖️ Compare
        </button>
        <button 
          className="ribbon-btn" 
          title="Combine Documents"
        >
          🔀 Combine
        </button>
      </div>

      {/* Info Panel */}
      {trackChangesEnabled && (
        <div className="ribbon-group" style={{ border: '1px solid #ffd700', borderRadius: '4px', padding: '8px' }}>
          <div className="ribbon-label" style={{ fontSize: '11px', color: '#b8860b' }}>
            ⚠️ Track Changes is ON - All edits will be marked
          </div>
        </div>
      )}
    </div>
  );
}
