mod content;
mod rendering;
mod style;

use colored::Colorize;
use rendering::BorderPainter;
use content::{TextLine};
use style::{Style};

pub struct Banner {
    pub width: u8,
    style: Style,
    lines: Vec<TextLine>,
}

impl Banner {
    /// Creates a new banner with default values.
    pub fn new() -> Banner {
        return Banner {
            width: 50,
            style: Style::new(),
            lines: Vec::new()
        };
    }

    /// Adds a line of text to the banner.
    ///
    /// # Arguments
    /// 
    /// * `self` - The banner to add the line of text to.
    /// * `text` - The text to add.
    pub fn add_text_line(&mut self, text: String) {
        self.lines.push(TextLine::new(text));
    }

    /// Prints the banner.
    ///
    /// # Arguments
    /// 
    /// * `self` - The banner to print.
    fn print(self: &Banner) {
        println!("{}", self.assemble());
    }

    /// Assembles the banner.
    ///
    /// # Arguments
    /// 
    /// * `self` - The banner to assemble.
    pub fn assemble(self: &Banner) -> String {
        let border_painter: BorderPainter = BorderPainter::new(
            &self.style.border, 
            self.style.is_monochrome, 
            self.width);

        let mut result: String;
        result = format!("{}\r\n", border_painter.top());
        for line in self.lines.iter() {
            let l = &(*line).fmt(&self.style.text, self.style.is_monochrome);
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
        let mut banner: Banner = Banner::new();
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
        let mut banner: Banner = Banner::new();
        banner.width = 4;
        banner.style.is_monochrome = false;
        banner.style.border.color = Color::White;
        let expected = format!(
            "{}",
            "\u{1b}[37m┌──┐\u{1b}[0m\r\n\u{1b}[37m└──┘\u{1b}[0m\r\n"
        );
        assert_eq!(expected, banner.assemble());
    }

    /// Verifies that a banner with a single text line is assembled correctly.
    #[test]
    fn test_assemble_simple() {
        // Build the banner
        let mut banner: Banner = Banner::new();
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
        // Build the banner
        let mut banner: Banner = Banner::new();
        banner.style.is_monochrome = false;
        banner.style.border.color = Color::White;
        banner.style.text.content_color = Color::Red;
        banner.width = 16;
        banner.add_text_line(String::from("Hello!"));

        // Build the expected output
        let expected = format!(
            "{}", "\u{1b}[37m┌──────────────┐\u{1b}[0m\r\n\u{1b}[37m│\u{1b}[0m\u{1b}[31mHello!\u{1b}[0m\u{1b}[37m│\u{1b}[0m\u{1b}[37m└──────────────┘\u{1b}[0m\r\n"
        );

        assert_eq!(expected, banner.assemble());
    }
}
