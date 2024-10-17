use crate::math::OrdFloat;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    White,
    Custom(f32, f32, f32, f32),
}

impl From<&Color> for Rgba {
    fn from(color: &Color) -> Self {
        match color {
            Color::Black => Rgba::color(0.0, 0.0, 0.0),
            Color::Red => Rgba::color(1.0, 0.0, 0.0),
            Color::Green => Rgba::color(0.0, 1.0, 0.0),
            Color::Blue => Rgba::color(0.0, 0.0, 1.0),
            Color::White => Rgba::color(1.0, 1.0, 1.0),
            Color::Custom(r, g, b, a) => Rgba::color_a(*r, *g, *b, *a),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Rgba {
    pub r: OrdFloat,
    pub g: OrdFloat,
    pub b: OrdFloat,
    pub a: OrdFloat,
}

impl Rgba {
    pub fn color(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: OrdFloat(r),
            g: OrdFloat(g),
            b: OrdFloat(b),
            a: OrdFloat(1.0),
        }
    }

    pub fn color_a(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: OrdFloat(r),
            g: OrdFloat(g),
            b: OrdFloat(b),
            a: OrdFloat(a),
        }
    }

    // to-do: rethink trait impls
    //blend on top of self
    pub fn over_blend(&mut self, mut over: Rgba) {
        let under_multiplier = OrdFloat(1.0) - over.a;
        self.r *= under_multiplier;
        self.g *= under_multiplier;
        self.b *= under_multiplier;
        over.r *= over.a;
        over.g *= over.a;
        over.b *= over.a;
        *self += over;
    }
}

impl From<&Rgba> for u32 {
    fn from(rgba: &Rgba) -> Self {
        fn to_8_bytes(c: f32) -> u32 {
            (c * 255.0) as u32
        }
        let mut color: u32 = to_8_bytes(rgba.r.0);
        color <<= 8;
        color |= to_8_bytes(rgba.g.0);
        color <<= 8;
        color |= to_8_bytes(rgba.b.0);
        color
    }
}

impl Add for &Rgba {
    type Output = Rgba;

    fn add(self, rhs: Self) -> Self::Output {
        Rgba::color_a(
            self.r.0 + rhs.r.0,
            self.g.0 + rhs.g.0,
            self.b.0 + rhs.b.0,
            self.a.0 + rhs.a.0,
        )
    }
}

impl AddAssign for Rgba {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Sub for &Rgba {
    type Output = Rgba;
    fn sub(self, rhs: Self) -> Self::Output {
        Rgba::color_a(
            self.r.0 - rhs.r.0,
            self.g.0 - rhs.g.0,
            self.b.0 - rhs.b.0,
            self.a.0 - rhs.a.0,
        )
    }
}

impl Mul<f32> for &Rgba {
    type Output = Rgba;
    fn mul(self, rhs: f32) -> Self::Output {
        Rgba::color_a(
            self.r.0 * rhs,
            self.g.0 * rhs,
            self.b.0 * rhs,
            self.a.0 * rhs,
        )
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
