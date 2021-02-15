use super::Color;

const DEFAULT_UNDERLINE_CHAR: char = '~';

/// Describes style information for a particular element.
pub struct ElementStyle {
    pub content_color: Color,
    pub is_underlined: bool,
    pub underline_char: char,
    pub underline_color: Color,
}

impl ElementStyle {
    /// Returns a new ElementStyle.
    pub fn new() -> ElementStyle {
        ElementStyle {
            content_color: Color::White,
            is_underlined: false,
            underline_char: DEFAULT_UNDERLINE_CHAR,
            underline_color: Color::White,
        }
    }
}
