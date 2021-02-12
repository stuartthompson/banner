use super::Color;

const DEFAULT_UNDERLINE_CHAR: char = '~';

/**
 * Defines style information for a particular element.
 */
pub struct ElementStyle {
    content_color: Color,
    is_underlined: bool,
    underline_char: char,
    underline_color: Color
}

impl ElementStyle {
    pub fn new() -> ElementStyle {
        ElementStyle {
            content_color: Color::White,
            is_underlined: false,
            underline_char: ' ',
            underline_color: Color::White,
        }
    }
}
