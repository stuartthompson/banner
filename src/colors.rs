pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black
}

impl Color {
    pub fn to_string(&self) -> &str {
        match self {
            Color::Red => { "red" },
            Color::Blue => { "blue" },
            Color::Green => { "green" },
            Color::Yellow => { "yellow" },
            Color::Magenta => { "magenta" },
            Color::Cyan => { "cyan" },
            Color::White => { "white" },
            Color::Black => { "black" }
        }
    }
}