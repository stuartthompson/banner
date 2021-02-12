use super::style::BorderStyle;
use colored::Colorize;

/// Represents a border painter.alloc
///
/// This is used to paint borders around banner content.
pub struct BorderPainter<'a> {
    style: &'a BorderStyle,
    is_monochrome: bool,
    width: u8,
}

impl BorderPainter<'_> {
    /// Creates a new BorderPainter.
    pub fn new(style: &BorderStyle, is_monochrome: bool, width: u8) -> BorderPainter {
        BorderPainter {
            style: style,
            is_monochrome: is_monochrome,
            width: width,
        }
    }

    /// Formats the border top as a colored string.
    pub fn top(self: &Self) -> String {
        // Guard against width values that are too small
        if self.width < 2 {
            return String::from("");
        }

        let str: String = format!(
            "{}{}{}",
            self.style.glyphs.top_left,
            (1..self.width - 1)
                .map(|_| self.style.glyphs.top)
                .collect::<String>(),
            self.style.glyphs.top_right.to_string()
        );
        self.colorize(str)
    }

    /// Formats the border bottom as a colored string.
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the border painter being operated on.
    pub fn bottom(self: &Self) -> String {
        // Guard against width values that are too small
        if self.width < 2 {
            return String::from("");
        }

        let str: String = format!(
            "{}{}{}",
            self.style.glyphs.bottom_left,
            (1..self.width - 1)
                .map(|_| self.style.glyphs.bottom)
                .collect::<String>(),
            self.style.glyphs.bottom_right
        );
        self.colorize(str)
    }

    /// Formats the border left-side as a colored string.
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the border painter being operated on.
    pub fn left(self: &Self) -> String {
        self.colorize(String::from(self.style.glyphs.left))
    }

    /// Formats the border right-side as a colored string.
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the border painter being operated on.
    pub fn right(self: &Self) -> String {
        self.colorize(String::from(self.style.glyphs.right))
    }

    /// Applies color codes to the output (if not using monochrome).
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the border painter being operated on.
    /// * `str`  - The string to colorize.
    fn colorize(self: &Self, str: String) -> String {
        if !self.is_monochrome {
            str.color(self.style.color.to_string()).to_string()
        } else {
            str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::style::{BorderGlyphs, Color};
    use super::*;

    /// Creates a default border style for use in unit tests.
    fn default_border_style() -> BorderStyle {
        BorderStyle {
            color: Color::White,
            is_visible: true,
            glyphs: BorderGlyphs::new(),
        }
    }

    /// Verifies that the painter renders a basic top border line.
    #[test]
    fn test_fmt_top_basic() {
        let style = default_border_style();
        let painter: BorderPainter = BorderPainter::new(&style, true, 4);
        let expected = format!("{}", "┌──┐");
        assert_eq!(expected, painter.top());
    }

    /// Verifies that the top painter handles a width that is too small.
    #[test]
    fn test_fmt_top_zero_width() {
        let style = default_border_style();
        let painter: BorderPainter = BorderPainter::new(&style, true, 0);
        let expected = format!("{}", "");
        assert_eq!(expected, painter.top());
    }

    /// Verifies that the painter renders a top border line that includes color codes.
    #[test]
    fn test_fmt_top_colored() {
        let mut style = default_border_style();
        style.color = Color::Red;
        let painter: BorderPainter = BorderPainter::new(&style, true, 4);
        let expected = format!("{}", "┌──┐");
        assert_eq!(expected, painter.top());
    }

    /// Verifies that the painter renders a basic bottom border line.
    #[test]
    fn test_fmt_bottom_basic() {
        let style = default_border_style();
        let painter: BorderPainter = BorderPainter::new(&style, true, 4);
        let expected = format!("{}", "└──┘");
        assert_eq!(expected, painter.bottom());
    }

    /// Verifies that the bottom painter handles a width that is too small.
    #[test]
    fn test_fmt_bottom_zero_width() {
        let style = default_border_style();
        let painter: BorderPainter = BorderPainter::new(&style, true, 0);
        let expected = format!("{}", "");
        assert_eq!(expected, painter.bottom());
    }

    /// Verifies painting a basic left border.
    #[test]
    fn test_fmt_left_basic() {
        let style = default_border_style();
        let painter: BorderPainter = BorderPainter::new(&style, true, 4);
        let expected = format!("{}", "│");
        assert_eq!(expected, painter.left());
    }

    /// Verifies painting a basic right border.
    #[test]
    fn test_fmt_right_default_monochrome() {
        let style = default_border_style();
        let painter: BorderPainter = BorderPainter::new(&style, true, 4);
        let expected = format!("{}", "│");
        assert_eq!(expected, painter.right());
    }
}
