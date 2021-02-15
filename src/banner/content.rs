mod text_line;

// Re-exports
pub use text_line::TextLine;

use super::style::ElementStyle;

/// Line traits render a line of content within a banner.
pub trait Line {
    fn fmt(self: &Self, style: &ElementStyle, is_monochrome: bool) -> String;
}