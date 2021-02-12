mod color;
mod border_glyphs;
mod border_style;
mod element_style;

pub use color::Color;
pub use border_glyphs::BorderGlyphs;
pub use border_style::BorderStyle;
pub use element_style::ElementStyle;

/**
 * Defines a banner style.
 */
pub struct Style {
    /**
     * Flag indicating if this style prints a monochrome banner.
     * Setting this flag to true will ignore all color code information in this style.
     */
    pub is_monochrome: bool,

    /**
     * The border color.
     */
    pub border: BorderStyle,

    /**
     * Defines the style for H1 elements.
     */
    pub h1: ElementStyle,

    /**
     * Defines the style for H2 elements.
     */
    pub h2: ElementStyle,

    /**
     * Defines the style for H3 elements.
     */
    pub h3: ElementStyle,

    /**
     * Defines the style for text elements.
     */
    pub text: ElementStyle
}

impl Style {
    pub fn new() -> Style {
        Style {
            is_monochrome: true,
            border: BorderStyle::new(),
            h1: ElementStyle::new(),
            h2: ElementStyle::new(),
            h3: ElementStyle::new(),
            text: ElementStyle::new()
        }
    }
}