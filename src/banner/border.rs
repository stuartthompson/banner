use crate::colors::Color;

use colored::*;

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
    bottom: char
}

impl BorderGlyphs {
    /**
     * Creates a new border glyphs descriptor with default values.
     */
    pub fn new() -> BorderGlyphs {
        return BorderGlyphs {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            top: '─',
            left: '│',
            right: '│',
            bottom: '─'
        }
    }
}

/**
 * Describes a banner border.
 */
pub struct Border {
    pub color: Color,
    pub is_visible: bool,
    pub glyphs: BorderGlyphs
}

impl Border {
    /**
     * Creates a new banner border with default values.
     */
    pub fn new() -> Border {
        return Border {
            color: Color::White,
            is_visible: true,
            glyphs: BorderGlyphs::new()
        };
    }

    /**
     * Creates a new banner border with a specific color.
     */
    pub fn new_with_color(
        color: Color
    ) -> Border {
        return Border {
            color: color,
            is_visible: true,
            glyphs: BorderGlyphs::new()
        };
    }

    /**
     * Creates a new banner border with a specific color and glyph descriptor.
     */
    pub fn new_with_color_and_glyphs(
        color: Color,
        glyphs: BorderGlyphs
    ) -> Border {
        return Border {
            color: color,
            is_visible: true,
            glyphs: glyphs
        };
    }

    /**
     * Formats the border top as a colored string.
     */
    pub fn fmt_top(self: &Border, width: u8) -> String {
        format!("{}{}{}", 
            self.glyphs.top_left, 
            (1..width-1).map(|_| self.glyphs.top).collect::<String>(), 
            self.glyphs.top_right.to_string())
            .color(self.color.to_string()).to_string()
    }

    /**
     * Formats the border bottom as a colored string.
     */
    pub fn fmt_bottom(self: &Border, width: u8) -> String {
        format!("{}{}{}", 
            self.glyphs.bottom_left, 
            (1..width-1).map(|_| self.glyphs.bottom).collect::<String>(), 
            self.glyphs.bottom_right)
            .color(self.color.to_string()).to_string()
    }

    /**
     * Formats the border left-side as a colored string.
     */
    pub fn fmt_left(self: &Border) -> String {
        format!("{}", String::from(self.glyphs.left).color(self.color.to_string()))
    }

    /**
     * Formats the border right-side as a colored string.
     */
    pub fn fmt_right(self: &Border) -> String {
        format!("{}", String::from(self.glyphs.right).color(self.color.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_top() {
        let border: Border = Border::new();
        let expected = format!("{}", "┌──┐".color(border.color.to_string()));
        assert_eq!(expected, border.fmt_top(4));
    }

    #[test]
    fn test_fmt_bottom() {
        let border: Border = Border::new();
        let expected = format!("{}", "└──┘".color(border.color.to_string()));
        assert_eq!(expected, border.fmt_bottom(4));
    }

    #[test]
    fn test_fmt_left() {
        let border: Border = Border::new();
        let expected = format!("{}", "│".color(border.color.to_string()));
        assert_eq!(expected, border.fmt_left());
    }

    #[test]
    fn test_fmt_right() {
        let border: Border = Border::new();
        let expected = format!("{}", "│".color(border.color.to_string()));
        assert_eq!(expected, border.fmt_right());
    }
}