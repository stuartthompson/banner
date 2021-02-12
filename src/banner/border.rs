use super::style::Color;

use colored::Colorize;

/**
 * Describes the glyphs used to render the border.
 */
pub struct BorderGlyphs {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    top: char,
    left: char,
    right: char,
    bottom: char,
}

const DEFAULT_TOP_LEFT_CHAR: char = '┌';
const DEFAULT_TOP_RIGHT_CHAR: char = '┐';
const DEFAULT_BOTTOM_LEFT_CHAR: char = '└';
const DEFAULT_BOTTOM_RIGHT_CHAR: char = '┘';
const DEFAULT_TOP_CHAR: char = '─';
const DEFAULT_LEFT_CHAR: char = '│';
const DEFAULT_RIGHT_CHAR: char = '│';
const DEFAULT_BOTTOM_CHAR: char = '─';

impl BorderGlyphs {
    /**
     * Creates a new border glyphs descriptor with default values.
     */
    pub fn new() -> BorderGlyphs {
        return BorderGlyphs {
            top_left: DEFAULT_TOP_LEFT_CHAR,
            top_right: DEFAULT_TOP_RIGHT_CHAR,
            bottom_left: DEFAULT_BOTTOM_LEFT_CHAR,
            bottom_right: DEFAULT_BOTTOM_RIGHT_CHAR,
            top: DEFAULT_TOP_CHAR,
            left: DEFAULT_LEFT_CHAR,
            right: DEFAULT_RIGHT_CHAR,
            bottom: DEFAULT_BOTTOM_CHAR,
        };
    }
}

/**
 * Describes a banner border.
 */
pub struct Border {
    pub color: Color,
    pub is_visible: bool,
    pub glyphs: BorderGlyphs,
}

impl Border {
    /**
     * Creates a new banner border with default values.
     */
    pub fn new() -> Border {
        return Border {
            color: Color::White,
            is_visible: true,
            glyphs: BorderGlyphs::new(),
        };
    }

    /**
     * Formats the border top as a colored string.
     */
    pub fn fmt_top(self: &Border, width: u8) -> String {
        format!(
            "{}{}{}",
            self.glyphs.top_left,
            (1..width - 1).map(|_| self.glyphs.top).collect::<String>(),
            self.glyphs.top_right.to_string()
        )
        .color(self.color.to_string())
        .to_string()
    }

    /**
     * Formats the border bottom as a colored string.
     */
    pub fn fmt_bottom(self: &Border, width: u8) -> String {
        format!(
            "{}{}{}",
            self.glyphs.bottom_left,
            (1..width - 1)
                .map(|_| self.glyphs.bottom)
                .collect::<String>(),
            self.glyphs.bottom_right
        )
        .color(self.color.to_string())
        .to_string()
    }

    /**
     * Formats the border left-side as a colored string.
     */
    pub fn fmt_left(self: &Border) -> String {
        format!(
            "{}",
            String::from(self.glyphs.left).color(self.color.to_string())
        )
    }

    /**
     * Formats the border right-side as a colored string.
     */
    pub fn fmt_right(self: &Border) -> String {
        format!(
            "{}",
            String::from(self.glyphs.right).color(self.color.to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_top() {
        let border: Border = Border::new();
        let expected = format!(
            "{}{}{}",
            DEFAULT_TOP_LEFT_CHAR,
            (0..2).map(|_| DEFAULT_TOP_CHAR).collect::<String>(),
            DEFAULT_TOP_RIGHT_CHAR
        )
        .color(border.color.to_string())
        .to_string();
        assert_eq!(expected, border.fmt_top(4));
    }

    #[test]
    fn test_fmt_bottom() {
        let border: Border = Border::new();
        let expected = format!(
            "{}{}{}",
            DEFAULT_BOTTOM_LEFT_CHAR,
            (0..2).map(|_| DEFAULT_BOTTOM_CHAR).collect::<String>(),
            DEFAULT_BOTTOM_RIGHT_CHAR
        )
        .color(border.color.to_string())
        .to_string();
        assert_eq!(expected, border.fmt_bottom(4));
    }

    #[test]
    fn test_fmt_left() {
        let border: Border = Border::new();
        let expected = format!(
            "{}",
            DEFAULT_LEFT_CHAR
                .to_string()
                .color(border.color.to_string())
        );
        assert_eq!(expected, border.fmt_left());
    }

    #[test]
    fn test_fmt_right() {
        let border: Border = Border::new();
        let expected = format!(
            "{}",
            DEFAULT_RIGHT_CHAR
                .to_string()
                .color(border.color.to_string())
        );
        assert_eq!(expected, border.fmt_right());
    }
}
