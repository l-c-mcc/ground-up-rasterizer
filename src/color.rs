#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    White,
    Custom(f32, f32, f32),
}

impl From<&Color> for Rgba {
    fn from(color: &Color) -> Self {
        match color {
            Color::Black => Rgba::color(0.0, 0.0, 0.0),
            Color::Red => Rgba::color(1.0, 0.0, 0.0),
            Color::Green => Rgba::color(0.0, 1.0, 0.0),
            Color::Blue => Rgba::color(0.0, 0.0, 1.0),
            Color::White => Rgba::color(1.0, 1.0, 1.0),
            Color::Custom(r, g, b) => Rgba::color(*r, *g, *b),
        }
    }
}

pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn color(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn color_a(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<&Rgba> for u32 {
    fn from(rgba: &Rgba) -> Self {
        fn to_8_bytes(c: f32) -> u32 {
            (c * 255.0) as u32
        }
        let mut color: u32 = to_8_bytes(rgba.r);
        color <<= 8;
        color |= to_8_bytes(rgba.g);
        color <<= 8;
        color |= to_8_bytes(rgba.b);
        color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{u16, u32, u8};

    #[test]
    fn test_into_u32() {
        let black_rgba = u32::from(&Rgba::color(1.0, 1.0, 1.0));
        let black_u32 = u32::MAX >> 8;
        assert_eq!(black_rgba, black_u32);

        let white_rgba = u32::from(&Rgba::color(0.0, 0.0, 0.0));
        let white_u32 = 0;
        assert_eq!(white_rgba, white_u32);

        let green_rgba = u32::from(&Rgba::color(0.0, 1.0, 0.0));
        let green_u32 = (u16::MAX as u32) ^ (u8::MAX as u32);
        assert_eq!(green_rgba, green_u32);
    }
}
