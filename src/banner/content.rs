mod text_line;

pub use text_line::TextLine;

pub trait Line {
    fn fmt(self: &Self) -> String;
}