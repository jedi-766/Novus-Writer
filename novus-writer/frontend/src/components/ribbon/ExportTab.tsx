import React, { useState } from 'react';
import { LexicalEditor } from 'lexical';
import { $getRoot, $selectAll } from 'lexical';

interface ExportTabProps {
  editor: LexicalEditor;
}

export function ExportTab({ editor }: ExportTabProps) {
  const [isExporting, setIsExporting] = useState(false);

  const exportToPDF = async () => {
    setIsExporting(true);
    try {
      // In a real implementation, this would call a backend service
      // or use a library like jsPDF or pdf-lib
      editor.getEditorState().read(() => {
        const root = $getRoot();
        const content = root.getTextContent();
        console.log('Exporting to PDF:', content.substring(0, 100));
      });
      
      // Simulate export delay
      await new Promise(resolve => setTimeout(resolve, 1000));
      alert('PDF export completed! (Simulation)');
    } catch (error) {
      console.error('PDF export failed:', error);
      alert('PDF export failed');
    } finally {
      setIsExporting(false);
    }
  };

  const exportToDOCX = async () => {
    setIsExporting(true);
    try {
      // In a real implementation, this would use a library like docx.js
      // or call a backend service
      editor.getEditorState().read(() => {
        const root = $getRoot();
        const content = root.getTextContent();
        console.log('Exporting to DOCX:', content.substring(0, 100));
      });
      
      await new Promise(resolve => setTimeout(resolve, 1000));
      alert('DOCX export completed! (Simulation)');
    } catch (error) {
      console.error('DOCX export failed:', error);
      alert('DOCX export failed');
    } finally {
      setIsExporting(false);
    }
  };

  const exportToHTML = () => {
    editor.getEditorState().read(() => {
      const root = $getRoot();
      // Generate HTML content
      const htmlContent = generateHTMLFromEditor(editor);
      
      // Create download blob
      const blob = new Blob([htmlContent], { type: 'text/html' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'document.html';
      a.click();
      URL.revokeObjectURL(url);
    });
  };

  const exportToMarkdown = () => {
    editor.getEditorState().read(() => {
      const root = $getRoot();
      const markdownContent = generateMarkdownFromEditor(editor);
      
      const blob = new Blob([markdownContent], { type: 'text/markdown' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'document.md';
      a.click();
      URL.revokeObjectURL(url);
    });
  };

  const exportToPlainText = () => {
    editor.getEditorState().read(() => {
      const root = $getRoot();
      const textContent = root.getTextContent();
      
      const blob = new Blob([textContent], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'document.txt';
      a.click();
      URL.revokeObjectURL(url);
    });
  };

  return (
    <div className="ribbon-tab-content export-tab">
      {/* Export Formats Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={exportToPDF}
          disabled={isExporting}
          title="Export as PDF"
        >
          📄 PDF
        </button>
        <button 
          className="ribbon-btn" 
          onClick={exportToDOCX}
          disabled={isExporting}
          title="Export as Word Document"
        >
          📝 DOCX
        </button>
      </div>

      {/* Web Formats Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={exportToHTML}
          disabled={isExporting}
          title="Export as HTML"
        >
          🌐 HTML
        </button>
        <button 
          className="ribbon-btn" 
          onClick={exportToMarkdown}
          disabled={isExporting}
          title="Export as Markdown"
        >
          # MD
        </button>
      </div>

      {/* Text Format Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={exportToPlainText}
          disabled={isExporting}
          title="Export as Plain Text"
        >
          📃 TXT
        </button>
      </div>

      {/* Print Group */}
      <div className="ribbon-group">
        <button 
          className="ribbon-btn" 
          onClick={() => window.print()}
          title="Print Document"
        >
          🖨️ Print
        </button>
      </div>
    </div>
  );
}

// Helper functions for export
function generateHTMLFromEditor(editor: LexicalEditor): string {
  let html = '<!DOCTYPE html>\n<html>\n<head>\n<meta charset="utf-8">\n<title>Document</title>\n</head>\n<body>\n';
  
  editor.getEditorState().read(() => {
    const root = $getRoot();
    html += root.getTextContent().replace(/\n/g, '<br>');
  });
  
  html += '\n</body>\n</html>';
  return html;
}

function generateMarkdownFromEditor(editor: LexicalEditor): string {
  let markdown = '';
  
  editor.getEditorState().read(() => {
    const root = $getRoot();
    markdown = root.getTextContent();
  });
  
  return markdown;
}
