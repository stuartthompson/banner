use super::super::style::{ElementStyle};
use super::Line;
use colored::Colorize;

/// Describes a line of text containing a key and value pair.
///
/// # Arguments
///
/// * `key` - The key name.
/// * `value` - The value as text.
pub struct KeyValueLine<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub style: &'a ElementStyle,
}

impl<'a> Line for KeyValueLine<'a> {
    /// Formats the key value line.
    ///
    /// # Arguments
    ///
    /// * `self` - The text line to format.
    /// * `no_color_codes` - A flag indicating whether to use color codes.
    fn fmt(self: &Self, no_color_codes: bool) -> String {
        let result = format!("{}: {}", self.key, self.value);

        if no_color_codes {
            result
        } else {
            result.color(self.style.content_color.to_string()).to_string()
        }
    }
}

impl<'a> KeyValueLine<'a> {
    /// Creates a new KeyValueLine.
    ///
    /// # Arguments
    ///
    /// * `key` - The key name.
    /// * `value` - The value as text.
    /// * `style` - The element style to apply when formatting this line.
    pub fn new(key: &'a str, value: &'a str, style: &'a ElementStyle) -> KeyValueLine<'a> {
        KeyValueLine { key, value, style }
    }
}