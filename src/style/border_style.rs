use super::{BorderGlyphs, Color};

/**
 * Describes a border style.
 */
pub struct BorderStyle {
    /**
     * Describes the glyphs to use for the border.
     */
    pub glyphs: BorderGlyphs,

    /**
     * Specifies the border color.
     */
    pub color: Color,

    /**
     * Specifies whether the border is visible.
     * True to show the border or false to hide the border.
     */
    pub is_visible: bool
}

impl BorderStyle {
    pub fn new() -> BorderStyle {
        BorderStyle {
            glyphs: BorderGlyphs::new(),
            color: Color::White,
            is_visible: true
        }
    }
}