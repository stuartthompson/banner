mod text_line;

// Re-exports
pub use text_line::TextLine;

/// Line traits render a line of content within a banner.
pub trait Line {
    fn fmt(self: &Self, is_monochrome: bool) -> String;
}