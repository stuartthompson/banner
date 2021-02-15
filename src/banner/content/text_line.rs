use super::super::style::{ElementStyle};
use super::Line;
use colored::Colorize;

/// Describes a line of text.
///
/// # Arguments
///
/// * `text` - The text content.
pub struct TextLine<'a> {
    pub text: String,
    pub style: &'a ElementStyle,
}

impl<'a> Line for TextLine<'a> {
    /// Formats the text line.
    ///
    /// # Arguments
    ///
    /// * `self` - The text line to format.
    /// * `style` - The style to use.
    /// * `is_monochrome` - A flag indicating whether to use color codes.
    fn fmt(self: &Self, no_color_codes: bool) -> String {
        if no_color_codes {
            self.text.to_string()
        } else {
            self.text.color(self.style.content_color.to_string()).to_string()
        }
    }
}

impl<'a> TextLine<'a> {
    /// Creates a new TextLine to be displayed as a top-level heading.
    ///
    /// # Arguments
    ///
    /// * `text` - The content of the text line.
    /// * `level` - The formatting level of the new text line.
    pub fn new(text: String, style: &'a ElementStyle) -> TextLine<'a> {
        TextLine { text, style }
    }
}
