//! Lexical editor type definitions for backend processing

use serde::{Deserialize, Serialize};

/// Lexical editor state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorState {
    pub root: EditorNode,
}

/// Editor node in the Lexical tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorNode {
    #[serde(rename = "type")]
    pub node_type: String,
    pub children: Vec<EditorChild>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub version: u32,
}

/// Child elements in editor nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditorChild {
    Node(EditorNode),
    Text(TextNode),
    Element(ElementNode),
}

/// Text node with formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub detail: u32,
    pub format: u32,
    pub mode: String,
    pub style: String,
    pub text: String,
    pub version: u32,
}

/// Element node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementNode {
    #[serde(rename = "type")]
    pub node_type: String,
    pub children: Vec<EditorChild>,
    pub direction: Option<String>,
    pub format: String,
    pub indent: u32,
    pub version: u32,
}

impl EditorState {
    /// Create a new empty editor state
    pub fn empty() -> Self {
        Self {
            root: EditorNode {
                node_type: "root".to_string(),
                children: vec![EditorChild::Element(ElementNode {
                    node_type: "paragraph".to_string(),
                    children: vec![],
                    direction: None,
                    format: "".to_string(),
                    indent: 0,
                    version: 1,
                })],
                direction: None,
                format: "".to_string(),
                indent: 0,
                version: 1,
            },
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Extract plain text from editor state
    pub fn extract_text(&self) -> String {
        self.extract_text_from_children(&self.root.children)
    }

    fn extract_text_from_children(&self, children: &[EditorChild]) -> String {
        let mut text = String::new();
        for child in children {
            match child {
                EditorChild::Text(text_node) => {
                    text.push_str(&text_node.text);
                }
                EditorChild::Node(node) | EditorChild::Element(node) => {
                    text.push_str(&self.extract_text_from_children(&node.children));
                    if node.node_type == "paragraph" || node.node_type == "heading" {
                        text.push('\n');
                    }
                }
            }
        }
        text
    }

    /// Count words in the document
    pub fn word_count(&self) -> usize {
        let text = self.extract_text();
        text.split_whitespace().count()
    }

    /// Count characters in the document
    pub fn char_count(&self) -> usize {
        self.extract_text().chars().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_editor_state() {
        let state = EditorState::empty();
        assert_eq!(state.root.node_type, "root");
        assert_eq!(state.word_count(), 0);
    }

    #[test]
    fn test_word_count() {
        let json = r#"{
            "root": {
                "type": "root",
                "children": [
                    {
                        "type": "paragraph",
                        "children": [
                            {
                                "detail": 0,
                                "format": 0,
                                "mode": "normal",
                                "style": "",
                                "text": "Hello world test",
                                "version": 1
                            }
                        ],
                        "direction": null,
                        "format": "",
                        "indent": 0,
                        "version": 1
                    }
                ],
                "direction": null,
                "format": "",
                "indent": 0,
                "version": 1
            }
        }"#;
        
        let state = EditorState::from_json(json).unwrap();
        assert_eq!(state.word_count(), 3);
        assert_eq!(state.char_count(), 16);
    }
}
