/**
 * Export/Import type definitions for Novus Writer
 */

export type ExportFormat = 
  | 'pdf'
  | 'docx'
  | 'html'
  | 'markdown'
  | 'txt'
  | 'rtf'
  | 'odt';

export interface ExportOptions {
  format: ExportFormat;
  outputPath?: string;
  includeMetadata?: boolean;
  includeImages?: boolean;
  pageSettings?: PageSettings;
}

export interface PageSettings {
  pageSize: 'A4' | 'Letter' | 'Legal' | 'Custom';
  orientation: 'portrait' | 'landscape';
  marginTop: number;
  marginBottom: number;
  marginLeft: number;
  marginRight: number;
  customWidth?: number;
  customHeight?: number;
}

export interface ImportOptions {
  sourcePath: string;
  targetFolderId?: number;
  detectFormatting?: boolean;
  preserveStyles?: boolean;
}

export interface ImportResult {
  documentId: number;
  importedAt: string;
  warnings?: string[];
  errors?: string[];
}

export interface PrintOptions {
  copies?: number;
  duplex?: 'simplex' | 'duplex' | 'duplexShortEdge';
  colorMode?: 'color' | 'grayscale' | 'monochrome';
  paperSize?: 'A4' | 'Letter' | 'Legal';
  orientation?: 'portrait' | 'landscape';
  printerName?: string;
  printToPdf?: boolean;
  pdfOutputPath?: string;
}
