use super::Line;

pub struct TextLine {
    text: String
}

impl TextLine {
    pub fn new(text: String) -> TextLine {
        TextLine { text: text }
    }
}

impl Line for TextLine {
    fn fmt(self: &Self) -> String {
        format!("{}", self.text)
    }
}