import { useCallback } from 'react';

export function useWordCounter() {
  const countWords = useCallback((text: string): number => {
    if (!text || text.trim() === '') {
      return 0;
    }
    // Split by whitespace and filter out empty strings
    const words = text.trim().split(/\s+/).filter(word => word.length > 0);
    return words.length;
  }, []);

  const countCharacters = useCallback((text: string, includeSpaces: boolean = true): number => {
    if (!text) {
      return 0;
    }
    if (includeSpaces) {
      return text.length;
    }
    return text.replace(/\s/g, '').length;
  }, []);

  const countParagraphs = useCallback((text: string): number => {
    if (!text || text.trim() === '') {
      return 0;
    }
    return text.split(/\n\s*\n/).filter(p => p.trim().length > 0).length;
  }, []);

  return {
    countWords,
    countCharacters,
    countParagraphs,
  };
}
