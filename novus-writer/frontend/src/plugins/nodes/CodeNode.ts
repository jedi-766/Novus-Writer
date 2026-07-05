import { ElementNode, LexicalNode } from 'lexical';

export class CodeNode extends ElementNode {
  __language?: string;
  __code: string;

  constructor(code: string, language?: string) {
    super();
    this.__code = code;
    this.__language = language || 'plaintext';
  }

  static getType(): string {
    return 'code';
  }

  static clone(node: CodeNode): CodeNode {
    return new CodeNode(node.__code, node.__language);
  }

  getLanguage(): string | undefined {
    return this.__language;
  }

  getCode(): string {
    return this.__code;
  }

  setCode(code: string): void {
    const writable = this.getWritable();
    writable.__code = code;
  }

  setLanguage(language: string): void {
    const writable = this.getWritable();
    writable.__language = language;
  }

  createDOM(): HTMLElement {
    const pre = document.createElement('pre');
    pre.className = 'editor-code';
    pre.style.backgroundColor = '#f5f5f5';
    pre.style.padding = '12px';
    pre.style.borderRadius = '4px';
    pre.style.overflowX = 'auto';
    
    const code = document.createElement('code');
    code.textContent = this.__code;
    pre.appendChild(code);
    
    return pre;
  }

  updateDOM(prevNode: CodeNode, dom: HTMLElement): boolean {
    if (prevNode.__code !== this.__code || prevNode.__language !== this.__language) {
      const code = dom.querySelector('code');
      if (code) {
        code.textContent = this.__code;
      }
      return true;
    }
    return false;
  }

  static importJSON(serializedNode: { code: string; language?: string }): CodeNode {
    const { code, language } = serializedNode;
    return new CodeNode(code, language);
  }

  exportJSON(): Record<string, any> {
    return {
      type: 'code',
      version: 1,
      code: this.__code,
      language: this.__language,
    };
  }

  isInline(): boolean {
    return false;
  }
}

export function $createCodeNode(code: string, language?: string): CodeNode {
  return new CodeNode(code, language);
}

export function $isCodeNode(node: LexicalNode | null | undefined): node is CodeNode {
  return node instanceof CodeNode;
}
