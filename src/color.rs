use std::ops::{Add, Mul};

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

//to-do: derive Copy
#[derive(Debug, Clone)]
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

impl Add for &Rgba {
    type Output = Rgba;

    fn add(self, rhs: Self) -> Self::Output {
        Rgba::color_a(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl Mul<f32> for &Rgba {
    type Output = Rgba;
    fn mul(self, rhs: f32) -> Self::Output {
        Rgba::color_a(self.r * rhs, self.g * rhs, self.b * rhs, self.a * rhs)
    }
}

impl Mul<&Rgba> for f32 {
    type Output = Rgba;
    fn mul(self, rhs: &Rgba) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::f32_compare;
    use std::{u16, u32, u8};

    impl PartialEq for Rgba {
        fn eq(&self, other: &Self) -> bool {
            let r = f32_compare(self.r, other.r);
            let g = f32_compare(self.g, other.g);
            let b = f32_compare(self.b, other.b);
            let a = f32_compare(self.a, other.a);
            r && g && b && a
        }
    }

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

    #[test]
    fn test_color_eq() {
        assert_eq!(Rgba::from(&Color::Blue), (&Color::Blue).into());
        assert_ne!(Rgba::from(&Color::Blue), (&Color::Red).into());
    }

    #[test]
    fn test_color_add() {
        let blue: Rgba = (&Color::Blue).into();
        let red: Rgba = (&Color::Red).into();
        assert_eq!(&blue + &red, Rgba::color_a(1.0, 0.0, 1.0, 2.0));
    }

    #[test]
    fn test_color_mul() {
        let orig = Rgba::color_a(1.0, 1.0, 1.0, 1.0);
        let left_mul = 2.0 * &orig;
        let right_mul = &orig * 0.2;
        assert_eq!(left_mul, Rgba::color_a(2.0, 2.0, 2.0, 2.0));
        assert_eq!(right_mul, Rgba::color_a(0.2, 0.2, 0.2, 0.2));
    }
}
