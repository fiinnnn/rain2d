#[derive(Debug, PartialEq, Copy, Clone)]
/// Color representation
pub struct Color {
    /// Red component
    pub r: u8,

    /// Green component
    pub g: u8,

    /// Blue component
    pub b: u8,

    /// Alpha component
    pub a: u8
}

/// r: 0, g: 0, b: 0, a: 0
pub const NONE: Color = Color { r: 0, g: 0, b: 0, a: 0 };

/// r: 255, g: 255, b: 255, a: 255
pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

/// r: 0, g: 0, b: 0, a: 255
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };

/// r: 255, g: 0, b: 0, a: 255
pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };

/// r: 0, g: 255, b: 0, a: 255
pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };

/// r: 0, g: 0, b: 255, a: 255
pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };

/// r: 255, g: 255, b: 0, a: 255
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 255 };

/// r: 0, g: 255, b: 255, a: 255
pub const CYAN: Color = Color { r: 0, g: 255, b: 255, a: 255 };

/// r: 255, g: 0, b: 255, a: 255
pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };

impl Color {
    /// Creates a color from rgb values, alpha defaults to 255
    ///
    /// ### Example
    /// ```
    ///# use rain2d::core::Color;
    /// let color = Color::rgb(255, 120, 120);
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 0xff }
    }

    /// Creates a color from rgba values
    ///
    /// ### Example
    /// ```
    ///# use rain2d::core::Color;
    /// let color = Color::rgba(255, 120, 120, 125);
    /// ```
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let (a, r, g, b) = (self.a as u32, self.r as u32, self.g as u32, self.b as u32);
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for Color {
    fn from(n: u32) -> Self {
        Self {
            a: (n >> 24) as u8,
            r: (n >> 16) as u8,
            g: (n >> 8) as u8,
            b: n as u8,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_from_rgb() {
        let color = Color::rgb(128, 255, 50);
        assert_eq!(color, Color { r: 128, g: 255, b: 50, a: 255 });
    }

    #[test]
    fn test_color_from_rgba() {
        let color = Color::rgba(128, 255, 50, 150);
        assert_eq!(color, Color { r: 128, g: 255, b: 50, a: 150 });
    }

    #[test]
    fn test_color_to_u32() {
        let color: u32 = Color { r: 124, g: 58, b: 231, a: 255}.into();
        assert_eq!(color, 0xFF7C_3AE7);
    }

    #[test]
    fn test_u32_to_color() {
        let color: Color = 0x237C_FF7E.into();
        assert_eq!(color, Color { r: 124, g: 255, b: 126, a: 35 });
    }
}