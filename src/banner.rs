mod border;

use colored::*;
use border::Border;

pub struct Banner {
    pub border: Border,
    pub lines: Vec<BannerLine>,
    pub width: u8
}

impl Banner {
    /**
     * Creates a new banner with default values.
     */
    pub fn new() -> Banner {
        return Banner {
            border: Border::new(),
            lines: Vec::new(),
            width: 50
        }
    }

    /**
     * Hides the banner border.
     */
    pub fn hide_border(&mut self) {
        self.border.is_visible = false;
    }

    /**
     * Prints the banner.
     */
    pub fn print(self: &Banner) {
        println!("{}", self.format());
    }

    /**
     * Formats the banner to a colored string.
     */
    pub fn format(self: &Banner) -> String {
        let top = self.border.fmt_top(self.width);
        let bottom = self.border.fmt_bottom(self.width);
        let left = self.border.fmt_left();
        let right = self.border.fmt_right();

        let mut result: String;
        
        result = format!("{}\r\n", top);
        for line in self.lines.iter() {
            result.push_str(&format!("{}", line.format(&left, &right, self.width))[..]);
        }
        result.push_str(&format!("{}\r\n", bottom)[..]);

        result
    }
}

pub struct BannerLine {
    parts: Vec<BannerPart>
}

impl BannerLine {
    pub fn build_key_value(
        key_text: &str, 
        key_color: &str, 
        value_text: &str, 
        value_color: &str
    ) -> BannerLine {
        return BannerLine { 
            parts: vec![
                BannerPart { text: String::from(key_text), color: String::from(key_color) },
                BannerPart { text: String::from(value_text), color: String::from(value_color) }
            ]
        }
    }

    pub fn format(self: &BannerLine, left: &String, right: &String, panel_width: u8) -> String {
        let mut result: String;
        let mut col: u8;
    
        // Print left edge plus one space (col: 2)
        result = format!("{} ", left);
        col = 1;
    
        // Print parts
        for part in self.parts.iter() {
            result.push_str(&format!("{}", part.text.color(&part.color[..]))[..]);
            col = col + part.text.len() as u8;
        }
    
        // Print remaining space
        result.push_str(&format!("{}", (col..panel_width).map(|_| " ").collect::<String>())[..]);
        result.push_str(&format!("{}", right)[..]);

        result
    }
}

pub struct BannerPart {
    text: String,
    color: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_default_empty() {
        let mut banner: Banner = Banner::new();
        banner.width = 4;
        let expected = 
            format!("{}\r\n{}\r\n", 
                "┌──┐".color(banner.border.color.to_string()),
                "└──┘".color(banner.border.color.to_string()));
        assert_eq!(expected, banner.format());
    }
}