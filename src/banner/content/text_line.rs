use colored::Colorize;
use super::super::style::ElementStyle;

/// Describes a line of text.
/// 
/// # Arguments
/// 
/// * `style` - The style to apply when printing this line of text.
/// * `text` - The text content.
pub struct TextLine<'a> {
    pub style: &'a ElementStyle,
    pub text: String
}

impl<'a> TextLine<'a> {
    /// Creates a new TextLine.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The content of the text line.
    pub fn new(
        style: &'a ElementStyle,
        text: String
    ) -> TextLine<'a> {
        TextLine { 
            style: style,
            text: text 
        }
    }

    /// Formats the text line
    /// 
    /// # Arguments
    /// 
    /// * `self` - The text line to format.
    pub fn fmt(self: &Self, is_monochrome: bool) -> String {
        if is_monochrome {
            self.text.to_string()
        }
        else {
            format!("{}", 
            self.text.color(self.style.content_color.to_string()))
        }
    }
}
