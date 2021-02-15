use super::super::style::ElementStyle;
use super::Line;
use colored::Colorize;

/// Describes a line of text.
///
/// # Arguments
///
/// * `text` - The text content.
pub struct TextLine {
    pub text: String,
}

impl Line for TextLine {
    /// Formats the text line.
    ///
    /// # Arguments
    ///
    /// * `self` - The text line to format.
    /// * `style` - The style to use.
    /// * `is_monochrome` - A flag indicating whether to use color codes.
    fn fmt(self: &Self, style: &ElementStyle, is_monochrome: bool) -> String {
        if is_monochrome {
            self.text.to_string()
        } else {
            self.text.color(style.content_color.to_string()).to_string()
        }
    }
}

impl TextLine {
    /// Creates a new TextLine.
    ///
    /// # Arguments
    ///
    /// * `text` - The content of the text line.
    pub fn new(text: String) -> TextLine {
        TextLine { text: text }
    }
}
