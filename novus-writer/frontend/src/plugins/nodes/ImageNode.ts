import { DecoratorNode } from 'lexical';

export class ImageNode extends DecoratorNode<JSX.Element> {
  __src: string;
  __altText: string;
  __width: number;
  __height: number;
  __caption?: string;
  __maxWidth: number;

  constructor(
    src: string,
    altText: string,
    width: number,
    height: number,
    caption?: string,
    maxWidth?: number,
  ) {
    super();
    this.__src = src;
    this.__altText = altText;
    this.__width = width;
    this.__height = height;
    this.__caption = caption;
    this.__maxWidth = maxWidth || 700;
  }

  static getType(): string {
    return 'image';
  }

  static clone(node: ImageNode): ImageNode {
    return new ImageNode(
      node.__src,
      node.__altText,
      node.__width,
      node.__height,
      node.__caption,
      node.__maxWidth,
    );
  }

  getWidth(): number {
    return this.__width;
  }

  getHeight(): number {
    return this.__height;
  }

  setWidthAndHeight(width: number, height: number): void {
    const writable = this.getWritable();
    writable.__width = width;
    writable.__height = height;
  }

  setCaption(caption: string): void {
    const writable = this.getWritable();
    writable.__caption = caption;
  }

  getSrc(): string {
    return this.__src;
  }

  getAltText(): string {
    return this.__altText;
  }

  getCaption(): string | undefined {
    return this.__caption;
  }

  decorate(): JSX.Element {
    // This will be rendered by React
    return {
      __type: 'image-component',
      __src: this.__src,
      __altText: this.__altText,
      __width: this.__width,
      __height: this.__height,
      __caption: this.__caption,
      __maxWidth: this.__maxWidth,
      __nodeKey: this.getKey(),
    } as any;
  }

  toJSON(): Record<string, any> {
    return {
      src: this.__src,
      altText: this.__altText,
      width: this.__width,
      height: this.__height,
      caption: this.__caption,
      maxWidth: this.__maxWidth,
    };
  }

  static importJSON(serializedNode: Record<string, any>): ImageNode {
    const { src, altText, width, height, caption, maxWidth } = serializedNode;
    const node = new ImageNode(src, altText, width, height, caption, maxWidth);
    return node;
  }

  exportJSON(): Record<string, any> {
    return {
      type: 'image',
      version: 1,
      src: this.__src,
      altText: this.__altText,
      width: this.__width,
      height: this.__height,
      caption: this.__caption,
      maxWidth: this.__maxWidth,
    };
  }

  isInline(): boolean {
    return false;
  }
}

export function $createImageNode(
  src: string,
  altText: string = '',
  width?: number,
  height?: number,
  caption?: string,
  maxWidth?: number,
): ImageNode {
  return new ImageNode(src, altText, width || 300, height || 200, caption, maxWidth);
}

export function $isImageNode(node: any | null): node is ImageNode {
  return node instanceof ImageNode;
}
