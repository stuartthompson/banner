use super::colors::Color;

/**
 * Describes a banner border.
 */
pub struct BannerBorder {
    pub color: Color
    pub visible: bool
}

impl BannerBorder {
    /**
     * Creates a new banner border with default values.
     */
    pub fn new() -> BannerBorder {
        return BannerBorder {
            color: Color::White,
            visible: true
        };
    }

    /**
     * Creates a new banner border with a specific color.
     */
    pub fn new(
        color: Color
    ) -> BannerBorder {
        return BannerBorder {
            color: color,
            visible: true
        };
    }
}