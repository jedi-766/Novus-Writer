import { ElementNode, LexicalNode } from 'lexical';

export class HorizontalRuleNode extends ElementNode {
  static getType(): string {
    return 'horizontalrule';
  }

  static clone(node: HorizontalRuleNode): HorizontalRuleNode {
    return new HorizontalRuleNode();
  }

  createDOM(): HTMLElement {
    const hr = document.createElement('hr');
    hr.className = 'editor-horizontal-rule';
    return hr;
  }

  updateDOM(prevNode: HorizontalRuleNode, dom: HTMLElement): boolean {
    return false;
  }

  static importJSON(serializedNode: any): HorizontalRuleNode {
    return new HorizontalRuleNode();
  }

  exportJSON(): Record<string, any> {
    return {
      type: 'horizontalrule',
      version: 1,
    };
  }

  isInline(): boolean {
    return false;
  }
}

export function $createHorizontalRuleNode(): HorizontalRuleNode {
  return new HorizontalRuleNode();
}

export function $isHorizontalRuleNode(node: LexicalNode | null | undefined): node is HorizontalRuleNode {
  return node instanceof HorizontalRuleNode;
}
