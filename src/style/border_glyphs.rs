const DEFAULT_TOP_LEFT_CHAR: char = '┌';
const DEFAULT_TOP_RIGHT_CHAR: char = '┐';
const DEFAULT_BOTTOM_LEFT_CHAR: char = '└';
const DEFAULT_BOTTOM_RIGHT_CHAR: char = '┘';
const DEFAULT_TOP_CHAR: char = '─';
const DEFAULT_LEFT_CHAR: char = '│';
const DEFAULT_RIGHT_CHAR: char = '│';
const DEFAULT_BOTTOM_CHAR: char = '─';

/**
 * Describes the glyphs used to render a border.
 */
pub struct BorderGlyphs {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub top: char,
    pub left: char,
    pub right: char,
    pub bottom: char,
}


impl BorderGlyphs {
    /**
     * Creates a new border glyphs descriptor with default values.
     */
    pub fn new() -> BorderGlyphs {
        return BorderGlyphs {
            top_left: DEFAULT_TOP_LEFT_CHAR,
            top_right: DEFAULT_TOP_RIGHT_CHAR,
            bottom_left: DEFAULT_BOTTOM_LEFT_CHAR,
            bottom_right: DEFAULT_BOTTOM_RIGHT_CHAR,
            top: DEFAULT_TOP_CHAR,
            left: DEFAULT_LEFT_CHAR,
            right: DEFAULT_RIGHT_CHAR,
            bottom: DEFAULT_BOTTOM_CHAR,
        };
    }
}
