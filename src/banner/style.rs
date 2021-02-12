mod color;

pub use color::Color;

/**
 * Defines a banner style.
 */
pub struct Style {
    /**
     * Flag indicating if this style prints a monochrome banner.
     * Setting this flag to true will ignore all color code information in this style.
     */
    is_monochrome: bool,

    /**
     * Defines the style for H1 elements.
     */
    h1: ElementStyle,

    /**
     * Defines the style for H2 elements.
     */
    h2: ElementStyle,

    /**
     * Defines the style for H3 elements.
     */
    h3: ElementStyle,

    /**
     * Defines the style for text elements.
     */
    text: ElementStyle
}

const DEFAULT_UNDERLINE_CHAR: char = '~';

/**
 * Defines style information for a particular element.
 */
pub struct ElementStyle {
    content_color: Color,
    is_underlined: bool,
    underline_char: char,
    underline_color: Color
}
