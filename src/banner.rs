mod content;
mod rendering;
mod style;

use rendering::BorderPainter;
use content::{Line, TextLine};
use style::{Style, FormatLevel};

pub struct Banner<'a> {
    pub width: u8,
    style: &'a Style,
    lines: Vec<Box<dyn Line + 'a>>,
}

impl<'a> Banner<'a> {
    /// Creates a new banner with default values.
    pub fn new(style: &'a Style) -> Banner<'a> {
        return Banner {
            width: 50,
            style: style,
            lines: Vec::new()
        };
    }

    /// Adds a line of text to the banner.
    ///
    /// # Arguments
    /// 
    /// * `self` - The banner to add the line of text to.
    /// * `text` - The text to add.
    /// * `element_type` - The element formatting level.
    pub fn add_text_line<'b>(&'b mut self, text: String) {
        let tl = TextLine::new(text, &self.style.text);

        self.lines.push(Box::new(tl));
    }

    /// Prints the banner.
    ///
    /// # Arguments
    /// 
    /// * `self` - The banner to print.
    fn print(self: &Banner<'a>) {
        println!("{}", self.assemble());
    }

    /// Assembles the banner.
    ///
    /// # Arguments
    /// 
    /// * `self` - The banner to assemble.
    pub fn assemble(self: &Banner<'a>) -> String {
        let border_painter: BorderPainter = BorderPainter::new(
            &self.style.border, 
            self.style.no_color_codes, 
            self.width);

        let mut result: String;
        result = format!("{}\r\n", border_painter.top());
        for line in self.lines.iter() {
            let l = &(*line).fmt(self.style.no_color_codes);
            // Add left border
            result.push_str(&border_painter.left());
            // Add line content
            result.push_str(&format!("{}", l)[..]);
            // Add whitespace to end
            result.push_str(&format!(
                "{}",
                (l.len() + 1..self.width as usize - 1)
                    .map(|_| " ")
                    .collect::<String>()
            ));
            // Add right border
            result.push_str(&border_painter.right());
        }
        result.push_str(&format!("{}\r\n", border_painter.bottom())[..]);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::style::Color;

    /// Verifies that a default, empty banner is assembled correctly.
    #[test]
    fn test_assemble_empty() {
        // Create a style
        let mut style: Style = Style::new();
        style.no_color_codes = true;

        let mut banner: Banner = Banner::new(&style);
        banner.width = 4;
        let expected = format!(
            "{}",
            "┌──┐\r\n└──┘\r\n"
        );
        assert_eq!(expected, banner.assemble());
    }

    /// Verifies that a default, empty banner with color codes is assembled correctly.
    #[test]
    fn test_assemble_empty_colored() {
        // Create a style
        let mut style: Style = Style::new();
        style.border.color = Color::White;

        let mut banner: Banner = Banner::new(&style);
        banner.width = 4;
        let expected = format!(
            "{}",
            "\u{1b}[37m┌──┐\u{1b}[0m\r\n\u{1b}[37m└──┘\u{1b}[0m\r\n"
        );
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
        banner.add_text_line(String::from("Hello!"));

        // Build the expected output
        let expected = format!(
            "{}", "┌──────────────┐\r\n│Hello!        │└──────────────┘\r\n"
        );

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
        banner.add_text_line(String::from("Hello, "));
        banner.add_text_line(String::from("World!"));

        // Build the expected output
        let expected = format!(
            "{}", "\u{1b}[37m┌──────────────┐\u{1b}[0m\r\n\u{1b}[37m│\u{1b}[0m\u{1b}[31mHello, \u{1b}[0m\u{1b}[37m│\u{1b}[0m\u{1b}[37m│\u{1b}[0m\u{1b}[31mWorld!\u{1b}[0m\u{1b}[37m│\u{1b}[0m\u{1b}[37m└──────────────┘\u{1b}[0m\r\n"
        );

        assert_eq!(expected, banner.assemble());
    }
}
