//! Formatting module - Text formatting definitions
//! 
//! Defines formatting options for rich text editing.

use serde::{Deserialize, Serialize};

/// Text format flags (bitmask)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextFormat(pub u32);

impl TextFormat {
    pub const NONE: Self = Self(0);
    pub const BOLD: Self = Self(1);
    pub const ITALIC: Self = Self(2);
    pub const UNDERLINE: Self = Self(4);
    pub const STRIKETHROUGH: Self = Self(8);
    pub const SUPERSCRIPT: Self = Self(16);
    pub const SUBSCRIPT: Self = Self(32);
    pub const SMALL_CAPS: Self = Self(64);
    pub const HIGHLIGHT: Self = Self(128);

    /// Check if a format flag is set
    pub fn has(self, flag: TextFormat) -> bool {
        self.0 & flag.0 != 0
    }

    /// Add a format flag
    pub fn add(&mut self, flag: TextFormat) {
        self.0 |= flag.0;
    }

    /// Remove a format flag
    pub fn remove(&mut self, flag: TextFormat) {
        self.0 &= !flag.0;
    }

    /// Toggle a format flag
    pub fn toggle(&mut self, flag: TextFormat) {
        self.0 ^= flag.0;
    }

    /// Clear all formats
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

/// Paragraph alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

/// Line spacing type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineSpacing {
    Single,
    OnePointFive,
    Double,
    Exact(f32), // Exact point value
    Multiple(f32), // Multiplier
}

impl Default for LineSpacing {
    fn default() -> Self {
        Self::Single
    }
}

/// Font family definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamily {
    pub name: String,
    pub stack: String,
}

impl FontFamily {
    pub const fn new(name: &'static str, stack: &'static str) -> Self {
        Self {
            name: name.to_string(),
            stack: stack.to_string(),
        }
    }
}

/// Predefined font families
impl FontFamily {
    pub const CALIBRI: Self = Self::new("Calibri", "Calibri, Candara, Segoe, sans-serif");
    pub const ARIAL: Self = Self::new("Arial", "Arial, Helvetica, sans-serif");
    pub const TIMES_NEW_ROMAN: Self = Self::new("Times New Roman", "\"Times New Roman\", Times, serif");
    pub const GEORGIA: Self = Self::new("Georgia", "Georgia, serif");
    pub const VERDANA: Self = Self::new("Verdana", "Verdana, Geneva, sans-serif");
    pub const TREBUCHET: Self = Self::new("Trebuchet MS", "\"Trebuchet MS\", sans-serif");
    pub const IMPACT: Self = Self::new("Impact", "Impact, Charcoal, sans-serif");
    pub const COMIC_SANS: Self = Self::new("Comic Sans MS", "\"Comic Sans MS\", cursive, sans-serif");
    pub const COURIER_NEW: Self = Self::new("Courier New", "\"Courier New\", Courier, monospace");
    pub const CONSOLAS: Self = Self::new("Consolas", "Consolas, Monaco, monospace");
}

/// Color representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);

    /// Convert to CSS color string
    pub fn to_css(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a as f32 / 255.0)
        }
    }

    /// Parse from hex string
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self::from_rgb(r, g, b))
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Self::new(r, g, b, a))
        } else {
            None
        }
    }
}

/// Paragraph formatting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParagraphFormat {
    pub alignment: Alignment,
    pub line_spacing: LineSpacing,
    pub space_before: f32, // points
    pub space_after: f32,  // points
    pub left_indent: f32,  // points
    pub right_indent: f32, // points
    pub first_line_indent: f32, // points
}

/// Character formatting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterFormat {
    pub font_family: Option<String>,
    pub font_size: Option<f32>, // points
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub text_format: TextFormat,
}

impl CharacterFormat {
    /// Check if any formatting is applied
    pub fn is_empty(&self) -> bool {
        self.font_family.is_none()
            && self.font_size.is_none()
            && self.color.is_none()
            && self.background_color.is_none()
            && self.text_format == TextFormat::NONE
    }
}
