mod content;
mod style;

use colored::Colorize;
use content::{Line, TextLine};
use style::Style;

pub struct Banner {
    pub width: u8,
    style: Style,
    lines: Vec<Box<dyn Line>>,
}

impl Banner {
    /**
     * Creates a new banner with default values.
     */
    pub fn new() -> Banner {
        return Banner {
            width: 50,
            style: Style::new(),
            lines: Vec::new(),
        };
    }

    /**
     * Adds a line of text to the banner.
     */
    pub fn add_text_line(&mut self, line: TextLine) {
        self.lines.push(Box::new(line));
    }

    /**
     * Prints the banner.
     */
    pub fn print(self: &Banner) {
        println!("{}", self.assemble());
    }

    /**
     * Assembles the banner.
     */
    pub fn assemble(self: &Banner) -> String {
        let mut result: String;
        result = format!("{}\r\n", self.fmt_border_top());
        for line in self.lines.iter() {
            let l = &(*line).fmt();
            // Add left border
            result.push_str(&self.fmt_border_left());
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
            result.push_str(&self.fmt_border_right());
        }
        result.push_str(&format!("{}\r\n", self.fmt_border_bottom())[..]);

        result
    }

    /**
     * Formats the border top as a colored string.
     */
    pub fn fmt_border_top(self: &Banner) -> String {
        let style = &self.style.border;
        let mut result: String = format!(
            "{}{}{}",
            style.glyphs.top_left,
            (1..self.width - 1).map(|_| style.glyphs.top).collect::<String>(),
            style.glyphs.top_right.to_string()
        );
        // Apply color (if not monochrome styled)
        if !self.style.is_monochrome {
            result = result.color(style.color.to_string()).to_string();
        }
        result
    }

    /**
     * Formats the border bottom as a colored string.
     */
    fn fmt_border_bottom(self: &Banner) -> String {
        let style = &self.style.border;
        let mut result: String = format!(
            "{}{}{}",
            style.glyphs.bottom_left,
            (1..self.width - 1)
                .map(|_| style.glyphs.bottom)
                .collect::<String>(),
            style.glyphs.bottom_right
        );
        // Apply color (if not monochrome styled)
        if !self.style.is_monochrome {
            result = result.color(style.color.to_string()).to_string();
        }
        result
    }

    /**
     * Formats the border left-side as a colored string.
     */
    fn fmt_border_left(self: &Banner) -> String {
        let style = &self.style.border;
        let mut result: String = String::from(style.glyphs.left);
        // Apply color (if not monochrome styled)
        if !self.style.is_monochrome {
            result = result.color(style.color.to_string()).to_string();
        }
        result
    }

    /**
     * Formats the border right-side as a colored string.
     */
    fn fmt_border_right(self: &Banner) -> String {
        let style = &self.style.border;
        let mut result: String = String::from(style.glyphs.right);
        // Apply color (if not monochrome styled)
        if !self.style.is_monochrome {
            result = result.color(style.color.to_string()).to_string();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fmt_default_empty() {
        let mut banner: Banner = Banner::new();
        banner.width = 4;
        banner.style.is_monochrome = true;
        let expected = format!(
            "{}",
            "┌──┐\r\n└──┘\r\n"
        );
        assert_eq!(expected, banner.assemble());
    }

    #[test]
    fn test_fmt_single_text_line() {
        // Build the banner
        let mut banner: Banner = Banner::new();
        banner.width = 16;
        banner.add_text_line(content::TextLine::new(String::from("Hello!")));

        // Build the expected output
        let expected = format!(
            "{}", "┌──────────────┐\r\n│Hello!        │└──────────────┘\r\n"
        );

        assert_eq!(expected, banner.assemble());
    }
}
