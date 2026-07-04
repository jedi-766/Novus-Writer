//! Content module - Document content structures
//! 
//! Defines the structure for rich text content using Lexical editor format.

use serde::{Deserialize, Serialize};

/// Lexical editor root node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexicalRoot {
    pub root: LexicalRootNode,
}

/// Root node containing all document children
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexicalRootNode {
    pub children: Vec<LexicalNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    #[serde(rename = "type")]
    pub node_type: String,
    pub version: u32,
}

/// A node in the Lexical editor tree
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LexicalNode {
    #[serde(rename = "paragraph")]
    Paragraph(ParagraphNode),
    #[serde(rename = "heading")]
    Heading(HeadingNode),
    #[serde(rename = "list")]
    List(ListNode),
    #[serde(rename = "listitem")]
    ListItem(ListItemNode),
    #[serde(rename = "table")]
    Table(TableNode),
    #[serde(rename = "image")]
    Image(ImageNode),
    #[serde(rename = "link")]
    Link(LinkNode),
    #[serde(rename = "code")]
    Code(CodeNode),
    #[serde(rename = "quote")]
    Quote(QuoteNode),
    #[serde(rename = "horizontalrule")]
    HorizontalRule(HorizontalRuleNode),
    #[serde(rename = "text")]
    Text(TextNode),
}

/// Paragraph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphNode {
    pub children: Vec<TextNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub version: u32,
}

/// Heading node with level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingNode {
    pub children: Vec<TextNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub tag: String, // h1, h2, h3, etc.
    pub version: u32,
}

/// List node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNode {
    pub children: Vec<ListItemNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub list_type: String, // bullet, number, check
    pub start: u32,
    pub version: u32,
}

/// List item node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemNode {
    pub children: Vec<TextNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub checked: Option<bool>,
    pub version: u32,
}

/// Table node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNode {
    pub children: Vec<TableRowNode>,
    pub version: u32,
}

/// Table row node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRowNode {
    pub children: Vec<TableCellNode>,
    pub version: u32,
}

/// Table cell node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCellNode {
    pub children: Vec<LexicalNode>,
    pub version: u32,
    pub header_scope: Option<String>,
    pub col_span: u32,
    pub row_span: u32,
}

/// Image node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageNode {
    pub format: String,
    pub id: String,
    pub src: String,
    pub alt_text: String,
    pub caption: Option<String>,
    pub width: u32,
    pub height: u32,
    pub max_width: u32,
    pub show_caption: bool,
    pub version: u32,
}

/// Link node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkNode {
    pub children: Vec<TextNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub url: String,
    pub target: Option<String>,
    pub title: Option<String>,
    pub rel: Option<String>,
    pub version: u32,
}

/// Code block node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    pub children: Vec<TextNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub language: Option<String>,
    pub version: u32,
}

/// Quote node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteNode {
    pub children: Vec<TextNode>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub version: u32,
}

/// Horizontal rule node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalRuleNode {
    pub version: u32,
}

/// Text node with formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub detail: u32,
    pub format: u32, // bitmask for bold, italic, etc.
    pub mode: String,
    pub style: String,
    pub text: String,
    pub version: u32,
}

impl TextNode {
    /// Create a new text node
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            detail: 0,
            format: 0,
            mode: "normal".to_string(),
            style: "".to_string(),
            text: text.into(),
            version: 1,
        }
    }

    /// Check if text is bold
    pub fn is_bold(&self) -> bool {
        self.format & 1 != 0
    }

    /// Check if text is italic
    pub fn is_italic(&self) -> bool {
        self.format & 2 != 0
    }

    /// Check if text is underlined
    pub fn is_underline(&self) -> bool {
        self.format & 4 != 0
    }
}
