mod content;
mod rendering;
mod style;

use content::{KeyValueLine, Line, TextLine};
use rendering::BorderPainter;
pub use style::{Color, HeaderLevel, Style};

pub struct Banner<'a> {
    pub width: u8,
    pub auto_widen: bool,
    style: &'a Style,
    lines: Vec<Box<dyn Line + 'a>>,
}

impl<'a> Banner<'a> {
    /// Creates a new banner with default values.
    pub fn new(style: &'a Style) -> Banner<'a> {
        return Banner {
            width: 50,
            auto_widen: true,
            style: style,
            lines: Vec::new(),
        };
    }

    /// Adds a header to the banner.
    ///
    /// # Arguments
    ///
    /// * `self` - The banner to add the line of text to.
    /// * `text` - The text content of the header.
    /// * `level` - The header level.
    pub fn add_header<'b>(&'b mut self, text: &'a str, level: HeaderLevel) {
        let line = TextLine::new(text, &self.style.header_style(&level));

        // Check if banner needs to be widened
        let line_width = line.width();
        if self.auto_widen && line_width > self.width {
            self.width = line_width
        }

        self.lines.push(Box::new(line));
    }

    /// Adds a line of text to the banner.
    ///
    /// # Arguments
    ///
    /// * `self` - The banner to add the line of text to.
    /// * `text` - The text to add.
    pub fn add_text<'b>(&'b mut self, text: &'a str) {
        let line = TextLine::new(text, &self.style.text);

        // Check if banner needs to be widened
        let line_width = line.width();
        if self.auto_widen && line_width > self.width {
            self.width = line_width
        }

        self.lines.push(Box::new(line));
    }

    /// Adds a line showing a key value pair to the banner.
    ///
    /// # Arguments
    ///
    /// * `self` - The banner to add the line to.
    /// * `key` - The key name.
    /// * `value` - The value as text.
    pub fn add_key_value<'b>(&'b mut self, key: &'a str, value: &'a str) {
        let line = KeyValueLine::new(key, value, &self.style.text);

        // Check if banner needs to be widened
        let line_width = line.width();
        if self.auto_widen && line_width > self.width {
            self.width = line_width
        }

        self.lines.push(Box::new(line));
    }

    /// Assembles the banner.
    ///
    /// # Arguments
    ///
    /// * `self` - The banner to assemble.
    pub fn assemble(self: &Banner<'a>) -> String {
        let border_painter: BorderPainter =
            BorderPainter::new(&self.style.border, self.style.no_color_codes, self.width);

        let mut result: String;
        result = format!("{}\r\n", border_painter.top());
        for line in self.lines.iter() {
            let line_text = &(*line).fmt(self.style.no_color_codes);
            let line_width = (*line).width();
            // Add left border
            result.push_str(&border_painter.left());
            // Add line content
            result.push_str(&format!("{}", line_text)[..]);
            // Add whitespace to end
            result.push_str(&format!(
                "{}",
                (line_width as usize..self.width as usize)
                    .map(|_| " ")
                    .collect::<String>()
            ));
            // Add right border
            result.push_str(&border_painter.right());
            result.push_str("\r\n");
        }
        result.push_str(&format!("{}\r\n", border_painter.bottom())[..]);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::style::Color;
    use super::*;

    // #region Tests for color code suppression

    /// Tests that an banner can be rendered without color codes.
    #[test]
    fn test_assemble_empty_no_color_codes() {
        // Create a style with suppressed color codes
        let mut style: Style = Style::new();
        style.no_color_codes = true;

        let mut banner: Banner = Banner::new(&style);
        banner.width = 4;
        assert_eq!("┌────┐\r\n└────┘\r\n", banner.assemble());
    }

    /// Tests that an banner can be rendered without color codes.
    #[test]
    fn test_assemble_right_padding() {
        // Create a style with suppressed color codes
        let mut style: Style = Style::new();
        style.no_color_codes = true;

        let mut banner: Banner = Banner::new(&style);
        banner.width = 10;
        banner.add_text("Test");

        assert_eq!("┌──────────┐\r\n│Test      │\r\n└──────────┘\r\n", banner.assemble());
    }

    /// Tests that an banner can be rendered without color codes.
    #[test]
    fn test_assemble_auto_width() {
        // Create a style with suppressed color codes
        let mut style: Style = Style::new();
        style.no_color_codes = true;

        let mut banner: Banner = Banner::new(&style);
        banner.width = 2; // Banner declared with width 2
        banner.add_text("Test"); // but should auto-widen to 4

        assert_eq!("┌────┐\r\n│Test│\r\n└────┘\r\n", banner.assemble());
    }

    #[test]
    fn test_assemble_banner_no_color_codes() {
        // Create a style with suppressed color codes
        let mut style: Style = Style::new();
        style.no_color_codes = true;

        let mut banner: Banner = Banner::new(&style);
        banner.width = 12;
        banner.add_header("Header h1", HeaderLevel::H1);
        banner.add_header("Header h2", HeaderLevel::H2);
        banner.add_header("Header h3", HeaderLevel::H3);
        banner.add_text("Text");
        banner.add_key_value("Key", "Val");

        let expected = "┌────────────┐\r\n│Header h1   │\r\n│Header h2   │\r\n│Header h3   │\r\n│Text        │\r\n│Key: Val    │\r\n└────────────┘\r\n";
        assert_eq!(expected, banner.assemble());
    }

    // #endregion

    /// Tests that an empty banner is assembled correctly.
    #[test]
    fn test_assemble_empty() {
        // Create a style
        let mut style: Style = Style::new();
        style.border.color = Color::White;

        // Build the banner
        let mut banner: Banner = Banner::new(&style);
        banner.width = 4;

        let expected = "\u{1b}[37m┌────┐\u{1b}[0m\r\n\u{1b}[37m└────┘\u{1b}[0m\r\n";
        assert_eq!(expected, banner.assemble());
    }

    /// Verifies that a banner with a single text line is assembled correctly.
    #[test]
    fn test_assemble_simple() {
        // Create a style
        let mut style: Style = Style::new();
        style.no_color_codes = true;

        // Build the banner
        let mut banner: Banner = Banner::new(&style);
        banner.width = 16;
        banner.add_text("Hello!");

        let expected = "┌────────────────┐\r\n│Hello!          │\r\n└────────────────┘\r\n";
        assert_eq!(expected, banner.assemble());
    }

    /// Verifies that a banner with a single text line is assembled correctly.
    #[test]
    fn test_assemble_simple_colored() {
        // Create a style
        let mut style: Style = Style::new();
        style.border.color = Color::White;
        style.text.content_color = Color::Red;

        // Build a banner
        let mut banner: Banner = Banner::new(&style);
        banner.width = 16;

        // Add multiple lines of text
        banner.add_text("Hello, ");
        banner.add_text("World!");

        let expected = "\u{1b}[37m┌────────────────┐\u{1b}[0m\r\n\u{1b}[37m│\u{1b}[0m\u{1b}[31mHello, \u{1b}[0m         \u{1b}[37m│\u{1b}[0m\r\n\u{1b}[37m│\u{1b}[0m\u{1b}[31mWorld!\u{1b}[0m          \u{1b}[37m│\u{1b}[0m\r\n\u{1b}[37m└────────────────┘\u{1b}[0m\r\n";
        assert_eq!(expected, banner.assemble());
    }
}
