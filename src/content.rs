mod text_line;
mod key_value_line;

// Re-exports
pub use text_line::TextLine;
pub use key_value_line::KeyValueLine;

/// Lines render a line of text within a banner.
pub trait Line {
    /// Formats the line.
    ///
    /// # Arguments
    ///
    /// * `self` - The line to format.
    /// * `no_color_codes` - A flag indicating whether to suppress color codes.
    fn fmt(self: &Self, no_color_codes: bool) -> String;

    /// Returns the width of the line when formatted.
    fn width(self: &Self) -> u8;
}