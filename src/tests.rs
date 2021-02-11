#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::colors::Color;

    #[test]
    fn test_colors_to_string() {
        assert_eq!(Color::Red.to_string(), "red");
        assert_eq!(Color::Blue.to_string(), "blue");
        assert_eq!(Color::Green.to_string(), "green");
        assert_eq!(Color::Yellow.to_string(), "yellow");
        assert_eq!(Color::Magenta.to_string(), "magenta");
        assert_eq!(Color::Cyan.to_string(), "cyan");
        assert_eq!(Color::White.to_string(), "white");
        assert_eq!(Color::Black.to_string(), "black");
    }
}