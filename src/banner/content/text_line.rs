use colored::Colorize;
use super::super::style::ElementStyle;

/// Describes a line of text.
/// 
/// # Arguments
/// 
/// * `text` - The text content.
pub struct TextLine {
    pub text: String
}

impl TextLine {
    /// Creates a new TextLine.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The content of the text line.
    pub fn new(
        text: String
    ) -> TextLine {
        TextLine { 
            text: text 
        }
    }

    /// Formats the text line
    /// 
    /// # Arguments
    /// 
    /// * `self` - The text line to format.
    pub fn fmt(self: &Self, style: &ElementStyle, is_monochrome: bool) -> String {
        if is_monochrome {
            self.text.to_string()
        }
        else {
            format!("{}", 
            self.text.color(style.content_color.to_string()))
        }
    }
}
