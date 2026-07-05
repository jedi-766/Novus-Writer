import React, { useState } from 'react';
import { LexicalEditor } from 'lexical';
import { $getRoot } from 'lexical';

interface ImportTabProps {
  editor: LexicalEditor;
}

export function ImportTab({ editor }: ImportTabProps) {
  const [isImporting, setIsImporting] = useState(false);

  const importFromDOCX = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.docx,application/vnd.openxmlformats-officedocument.wordprocessingml.document';
    
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        setIsImporting(true);
        try {
          // In a real implementation, this would use a library like mammoth.js
          // or call a backend service to parse DOCX
          const reader = new FileReader();
          reader.onload = async (event) => {
            const arrayBuffer = event.target?.result as ArrayBuffer;
            console.log('DOCX file loaded:', file.name, arrayBuffer.byteLength);
            
            // Simulate parsing delay
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            // For now, just insert placeholder text
            editor.update(() => {
              const root = $getRoot();
              root.append($createParagraphNode(`[DOCX Content from ${file.name} - Full import requires backend processing]`));
            });
            
            alert(`DOCX import completed! (Simulation)\nFile: ${file.name}`);
          };
          reader.readAsArrayBuffer(file);
        } catch (error) {
          console.error('DOCX import failed:', error);
          alert('DOCX import failed');
        } finally {
          setIsImporting(false);
        }
      }
    };
    
    input.click();
  };

  const importFromHTML = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.html,.htm,text/html';
    
    input.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          const htmlContent = event.target?.result as string;
          
          // Parse HTML and convert to Lexical nodes
          editor.update(() => {
            const root = $getRoot();
            // Simple HTML to text conversion for demo
            const tempDiv = document.createElement('div');
            tempDiv.innerHTML = htmlContent;
            const textContent = tempDiv.textContent || tempDiv.innerText || '';
            
            root.append($createParagraphNode(textContent.substring(0, 500)));
          });
          
          alert(`HTML import completed!\nFile: ${file.name}`);
        };
        reader.readAsText(file);
      }
    };
    
    input.click();
  };

  const importFromMarkdown = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.md,.markdown,text/markdown';
    
    input.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          const markdownContent = event.target?.result as string;
          
          // Parse Markdown and convert to Lexical nodes
          editor.update(() => {
            const root = $getRoot();
            // Simple markdown parsing for demo
            const lines = markdownContent.split('\n');
            lines.forEach(line => {
              let text = line;
              
              // Handle headers
              if (line.startsWith('# ')) {
                text = line.substring(2);
                // Would create H1 node in full implementation
              } else if (line.startsWith('## ')) {
                text = line.substring(3);
                // Would create H2 node
              } else if (line.startsWith('### ')) {
                text = line.substring(4);
                // Would create H3 node
              }
              
              // Handle bold/italic markers
              text = text.replace(/\*\*(.*?)\*\*/g, '$1');
              text = text.replace(/\*(.*?)\*/g, '$1');
              
              root.append($createParagraphNode(text));
            });
          });
          
          alert(`Markdown import completed!\nFile: ${file.name}`);
        };
        reader.readAsText(file);
      }
    };
    
    input.click();
  };

  const importFromPlainText = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.txt,text/plain';
    
    input.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          const textContent = event.target?.result as string;
          
          editor.update(() => {
            const root = $getRoot();
            const lines = textContent.split('\n');
            lines.forEach(line => {
              root.append($createParagraphNode(line));
            });
          });
          
          alert(`Text import completed!\nFile: ${file.name}`);
        };
        reader.readAsText(file);
      }
    };
    
    input.click();
  };

  return (
    <div className="ribbon-tab-content import-tab">
      {/* Document Formats Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={importFromDOCX}
          disabled={isImporting}
          title="Import from Word Document"
        >
          📝 From DOCX
        </button>
        <button 
          className="ribbon-btn" 
          onClick={importFromHTML}
          disabled={isImporting}
          title="Import from HTML"
        >
          🌐 From HTML
        </button>
      </div>

      {/* Text Formats Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={importFromMarkdown}
          disabled={isImporting}
          title="Import from Markdown"
        >
          # From MD
        </button>
        <button 
          className="ribbon-btn" 
          onClick={importFromPlainText}
          disabled={isImporting}
          title="Import from Plain Text"
        >
          📃 From TXT
        </button>
      </div>

      {/* Info Group */}
      <div className="ribbon-group">
        <div className="ribbon-label" style={{ fontSize: '11px', color: '#666', maxWidth: '150px' }}>
          Note: Some formats may require backend processing for full fidelity.
        </div>
      </div>
    </div>
  );
}

// Helper function to create paragraph nodes
function $createParagraphNode(text: string): any {
  // This is a simplified version - in reality you'd use Lexical's ParagraphNode
  return { type: 'paragraph', text };
}
