mod border;
mod content;
mod style;

use border::Border;
use content::{Line, TextLine};

pub struct Banner {
    pub border: Border,
    pub lines: Vec<Box<dyn Line>>,
    pub width: u8,
}

impl Banner {
    /**
     * Creates a new banner with default values.
     */
    pub fn new() -> Banner {
        return Banner {
            border: Border::new(),
            lines: Vec::new(),
            width: 50,
        };
    }

    /**
     * Hides the banner border.
     */
    pub fn hide_border(&mut self) {
        self.border.is_visible = false;
    }

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
        let top = self.border.fmt_top(self.width);
        let bottom = self.border.fmt_bottom(self.width);
        let left = self.border.fmt_left();
        let right = self.border.fmt_right();

        let mut result: String;
        result = format!("{}\r\n", top);
        for line in self.lines.iter() {
            let l = &(*line).fmt();
            // Add left border
            result.push_str(&left);
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
            result.push_str(&right);
        }
        result.push_str(&format!("{}\r\n", bottom)[..]);

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
        let expected = format!("{}", "\u{1b}[37m┌──┐\u{1b}[0m\r\n\u{1b}[37m└──┘\u{1b}[0m\r\n");
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
            "{}", "\u{1b}[37m┌──────────────┐\u{1b}[0m\r\n\u{1b}[37m│\u{1b}[0mHello!        \u{1b}[37m│\u{1b}[0m\u{1b}[37m└──────────────┘\u{1b}[0m\r\n"
        );

        assert_eq!(expected, banner.assemble());
    }
}
