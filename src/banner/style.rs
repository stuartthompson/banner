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
     * Used to suppress color codes.
     * Setting this flag to true will omit color codes from output.
     */
    pub no_color_codes: bool,

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

pub enum FormatLevel {
    H1,
    H2,
    H3,
    Text
}

impl Style {
    /// Returns a new Style.
    pub fn new() -> Style {
        Style {
            no_color_codes: false,
            border: BorderStyle::new(),
            h1: ElementStyle::new(),
            h2: ElementStyle::new(),
            h3: ElementStyle::new(),
            text: ElementStyle::new()
        }
    }

    /// Returns the element style for a specific format level.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The parent style to retrieve the element style from.
    /// * `level` - The format level used to identify the requested element style.
    pub fn element_style(
        self: &Style,
        level: &FormatLevel
    ) -> &ElementStyle {
        match level {
            FormatLevel::H1 => &self.h1,
            FormatLevel::H2 => &self.h2,
            FormatLevel::H3 => &self.h3,
            FormatLevel::Text => &self.text
        }
    }
}