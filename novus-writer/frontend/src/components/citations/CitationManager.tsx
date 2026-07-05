import React, { useState, useEffect } from 'react';
import { BookOpen, Plus, Edit, Trash2, Quote, List } from 'lucide-react';

interface Citation {
  id: string;
  citationType: string;
  entryType: string;
  fields: Record<string, string>;
  formattedCitation: string;
  bibliographyEntry: string;
}

interface CitationStyle {
  id: string;
  name: string;
  description?: string;
}

interface CitationManagerProps {
  documentId: string;
  onInsertCitation: (citationId: string, formattedText: string) => void;
}

const CitationManager: React.FC<CitationManagerProps> = ({ documentId, onInsertCitation }) => {
  const [citations, setCitations] = useState<Citation[]>([]);
  const [styles, setStyles] = useState<CitationStyle[]>([]);
  const [selectedStyle, setSelectedStyle] = useState<string>('apa');
  const [showAddForm, setShowAddForm] = useState(false);
  const [showBibliography, setShowBibliography] = useState(false);
  const [bibliography, setBibliography] = useState<string[]>([]);
  
  const [newCitation, setNewCitation] = useState({
    entryType: 'book',
    author: '',
    title: '',
    year: '',
    publisher: '',
    journal: '',
    volume: '',
    issue: '',
    pages: '',
    doi: '',
    url: '',
  });

  useEffect(() => {
    fetchCitations();
    fetchStyles();
  }, [documentId]);

  const fetchCitations = async () => {
    try {
      const response = await fetch(`/api/citations?document_id=${documentId}`);
      if (response.ok) {
        const data = await response.json();
        setCitations(data);
      }
    } catch (error) {
      console.error('Failed to fetch citations:', error);
    }
  };

  const fetchStyles = async () => {
    try {
      const response = await fetch('/api/citations/styles');
      if (response.ok) {
        const data = await response.json();
        setStyles(data);
      }
    } catch (error) {
      console.error('Failed to fetch styles:', error);
    }
  };

  const handleAddCitation = async () => {
    try {
      const response = await fetch('/api/citations', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          document_id: documentId,
          citation_type: selectedStyle,
          entry_type: newCitation.entryType,
          fields: newCitation,
        }),
      });

      if (response.ok) {
        await fetchCitations();
        setShowAddForm(false);
        setNewCitation({
          entryType: 'book',
          author: '',
          title: '',
          year: '',
          publisher: '',
          journal: '',
          volume: '',
          issue: '',
          pages: '',
          doi: '',
          url: '',
        });
      }
    } catch (error) {
      console.error('Failed to add citation:', error);
    }
  };

  const handleInsertCitation = (citation: Citation) => {
    onInsertCitation(citation.id, citation.formattedCitation);
  };

  const handleDeleteCitation = async (citationId: string) => {
    try {
      await fetch(`/api/citations/${citationId}`, { method: 'DELETE' });
      await fetchCitations();
    } catch (error) {
      console.error('Failed to delete citation:', error);
    }
  };

  const generateBibliography = async () => {
    try {
      const response = await fetch(`/api/citations/bibliography?document_id=${documentId}&style=${selectedStyle}`);
      if (response.ok) {
        const data = await response.json();
        setBibliography(data.entries);
        setShowBibliography(true);
      }
    } catch (error) {
      console.error('Failed to generate bibliography:', error);
    }
  };

  const entryTypes = [
    { value: 'book', label: 'Book' },
    { value: 'journal_article', label: 'Journal Article' },
    { value: 'conference', label: 'Conference Paper' },
    { value: 'website', label: 'Website' },
    { value: 'thesis', label: 'Thesis/Dissertation' },
    { value: 'report', label: 'Report' },
  ];

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-lg font-semibold flex items-center gap-2">
            <BookOpen className="w-5 h-5" />
            Citations & Bibliography
          </h2>
          <button
            onClick={generateBibliography}
            className="px-3 py-1.5 bg-green-600 text-white text-xs rounded hover:bg-green-700 flex items-center gap-1"
          >
            <List className="w-3 h-3" />
            Bibliography
          </button>
        </div>

        {/* Style Selector */}
        <div className="mb-3">
          <label className="text-xs text-gray-600 block mb-1">Citation Style</label>
          <select
            value={selectedStyle}
            onChange={(e) => setSelectedStyle(e.target.value)}
            className="w-full px-3 py-2 border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {styles.map((style) => (
              <option key={style.id} value={style.id}>
                {style.name}
              </option>
            ))}
          </select>
        </div>

        <button
          onClick={() => setShowAddForm(!showAddForm)}
          className="w-full px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 text-sm flex items-center justify-center gap-2"
        >
          <Plus className="w-4 h-4" />
          Add Citation
        </button>
      </div>

      {/* Add Citation Form */}
      {showAddForm && (
        <div className="p-4 border-b bg-gray-50">
          <h3 className="text-sm font-medium mb-3">New Citation</h3>
          
          <div className="space-y-3">
            <div>
              <label className="text-xs text-gray-600 block mb-1">Entry Type</label>
              <select
                value={newCitation.entryType}
                onChange={(e) => setNewCitation({ ...newCitation, entryType: e.target.value })}
                className="w-full px-2 py-1.5 border rounded text-sm"
              >
                {entryTypes.map((type) => (
                  <option key={type.value} value={type.value}>
                    {type.label}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="text-xs text-gray-600 block mb-1">Author(s)</label>
              <input
                type="text"
                value={newCitation.author}
                onChange={(e) => setNewCitation({ ...newCitation, author: e.target.value })}
                className="w-full px-2 py-1.5 border rounded text-sm"
                placeholder="Smith, J."
              />
            </div>

            <div>
              <label className="text-xs text-gray-600 block mb-1">Title</label>
              <input
                type="text"
                value={newCitation.title}
                onChange={(e) => setNewCitation({ ...newCitation, title: e.target.value })}
                className="w-full px-2 py-1.5 border rounded text-sm"
                placeholder="Document title"
              />
            </div>

            <div>
              <label className="text-xs text-gray-600 block mb-1">Year</label>
              <input
                type="text"
                value={newCitation.year}
                onChange={(e) => setNewCitation({ ...newCitation, year: e.target.value })}
                className="w-full px-2 py-1.5 border rounded text-sm"
                placeholder="2024"
              />
            </div>

            {newCitation.entryType === 'book' && (
              <div>
                <label className="text-xs text-gray-600 block mb-1">Publisher</label>
                <input
                  type="text"
                  value={newCitation.publisher}
                  onChange={(e) => setNewCitation({ ...newCitation, publisher: e.target.value })}
                  className="w-full px-2 py-1.5 border rounded text-sm"
                />
              </div>
            )}

            {newCitation.entryType === 'journal_article' && (
              <>
                <div>
                  <label className="text-xs text-gray-600 block mb-1">Journal</label>
                  <input
                    type="text"
                    value={newCitation.journal}
                    onChange={(e) => setNewCitation({ ...newCitation, journal: e.target.value })}
                    className="w-full px-2 py-1.5 border rounded text-sm"
                  />
                </div>
                <div>
                  <label className="text-xs text-gray-600 block mb-1">Volume(Issue)</label>
                  <input
                    type="text"
                    value={`${newCitation.volume}(${newCitation.issue})`}
                    onChange={(e) => setNewCitation({ ...newCitation, volume: e.target.value })}
                    className="w-full px-2 py-1.5 border rounded text-sm"
                  />
                </div>
              </>
            )}

            <div className="flex gap-2">
              <button
                onClick={handleAddCitation}
                className="flex-1 px-3 py-1.5 bg-blue-600 text-white text-sm rounded hover:bg-blue-700"
              >
                Save
              </button>
              <button
                onClick={() => setShowAddForm(false)}
                className="flex-1 px-3 py-1.5 bg-gray-200 text-gray-700 text-sm rounded hover:bg-gray-300"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Citations List */}
      <div className="flex-1 overflow-y-auto p-4">
        {citations.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            <Quote className="w-12 h-12 mx-auto mb-2 opacity-50" />
            <p>No citations yet</p>
            <p className="text-xs mt-1">Add your first citation above</p>
          </div>
        ) : (
          <div className="space-y-3">
            {citations.map((citation) => (
              <div
                key={citation.id}
                className="border rounded-lg p-3 bg-white hover:shadow-sm transition-shadow"
              >
                <div className="flex justify-between items-start mb-2">
                  <span className="text-xs px-2 py-0.5 bg-gray-100 rounded">
                    {citation.entryType.replace('_', ' ')}
                  </span>
                  <div className="flex gap-1">
                    <button
                      onClick={() => handleInsertCitation(citation)}
                      className="p-1 text-blue-600 hover:bg-blue-50 rounded"
                      title="Insert in text"
                    >
                      <Quote className="w-3 h-3" />
                    </button>
                    <button
                      onClick={() => handleDeleteCitation(citation.id)}
                      className="p-1 text-red-600 hover:bg-red-50 rounded"
                      title="Delete"
                    >
                      <Trash2 className="w-3 h-3" />
                    </button>
                  </div>
                </div>
                
                <p className="text-sm text-gray-800 mb-1">{citation.formattedCitation}</p>
                <p className="text-xs text-gray-500 line-clamp-2">{citation.bibliographyEntry}</p>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Bibliography Modal */}
      {showBibliography && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg max-w-2xl w-full max-h-[80vh] flex flex-col m-4">
            <div className="p-4 border-b flex justify-between items-center">
              <h3 className="text-lg font-semibold">Bibliography</h3>
              <button
                onClick={() => setShowBibliography(false)}
                className="text-gray-500 hover:text-gray-700"
              >
                ✕
              </button>
            </div>
            
            <div className="flex-1 overflow-y-auto p-4">
              <ol className="list-decimal list-inside space-y-2">
                {bibliography.map((entry, index) => (
                  <li key={index} className="text-sm text-gray-800 pl-4 -ml-4">
                    {entry}
                  </li>
                ))}
              </ol>
            </div>

            <div className="p-4 border-t flex justify-end gap-2">
              <button
                onClick={() => setShowBibliography(false)}
                className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded"
              >
                Close
              </button>
              <button
                onClick={() => {
                  navigator.clipboard.writeText(bibliography.join('\n\n'));
                  alert('Bibliography copied to clipboard!');
                }}
                className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
              >
                Copy to Clipboard
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default CitationManager;
