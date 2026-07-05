import { ElementNode, LexicalNode } from 'lexical';

export class TableNode extends ElementNode {
  __rows: number;
  __columns: number;

  constructor(rows: number, columns: number) {
    super();
    this.__rows = rows;
    this.__columns = columns;
  }

  static getType(): string {
    return 'table';
  }

  static clone(node: TableNode): TableNode {
    return new TableNode(node.__rows, node.__columns);
  }

  getRows(): number {
    return this.__rows;
  }

  getColumns(): number {
    return this.__columns;
  }

  setRows(rows: number): void {
    const writable = this.getWritable();
    writable.__rows = rows;
  }

  setColumns(columns: number): void {
    const writable = this.getWritable();
    writable.__columns = columns;
  }

  createDOM(): HTMLElement {
    const table = document.createElement('table');
    table.className = 'editor-table';
    table.style.borderCollapse = 'collapse';
    table.style.width = '100%';
    return table;
  }

  updateDOM(prevNode: TableNode, dom: HTMLElement): boolean {
    return false;
  }

  static importJSON(serializedNode: { rows: number; columns: number }): TableNode {
    const { rows, columns } = serializedNode;
    const node = new TableNode(rows, columns);
    return node;
  }

  exportJSON(): Record<string, any> {
    return {
      type: 'table',
      version: 1,
      rows: this.__rows,
      columns: this.__columns,
    };
  }

  insertRow(index?: number): void {
    // Implementation for inserting a row
    const writable = this.getWritable();
    if (index === undefined) {
      writable.__rows++;
    } else {
      writable.__rows++;
      // Additional logic to insert at specific index
    }
  }

  insertColumn(index?: number): void {
    // Implementation for inserting a column
    const writable = this.getWritable();
    if (index === undefined) {
      writable.__columns++;
    } else {
      writable.__columns++;
      // Additional logic to insert at specific index
    }
  }

  deleteRow(index: number): void {
    if (this.__rows > 1) {
      const writable = this.getWritable();
      writable.__rows--;
    }
  }

  deleteColumn(index: number): void {
    if (this.__columns > 1) {
      const writable = this.getWritable();
      writable.__columns--;
    }
  }
}

export function $createTableNode(rows: number, columns: number): TableNode {
  return new TableNode(rows, columns);
}

export function $isTableNode(node: LexicalNode | null | undefined): node is TableNode {
  return node instanceof TableNode;
}
