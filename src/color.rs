pub struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    _a: f32,
}

impl RGBA {
    pub fn color(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, _a: 1.0 }
    }
}

impl From<&RGBA> for u32 {
    fn from(rgba: &RGBA) -> Self {
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

/*
impl Into<u32> for &RGBA {
    fn into(self) -> u32 {
        fn to_8_bytes(c: f32) -> u32 {
            (c * 255.0) as u32
        }
        let mut color: u32 = to_8_bytes(self.0);
        color <<= 8;
        color |= to_8_bytes(self.1);
        color <<= 8;
        color |= to_8_bytes(self.2);
        color
    }
}
*/

#[cfg(test)]
mod tests {
    use std::{u16, u32, u8};
    use super::*;

    #[test]
    fn test_into_u32() {
        let black_rgba = u32::from(&RGBA::color(1.0, 1.0, 1.0));
        let black_u32 = u32::MAX >> 8;
        assert_eq!(black_rgba, black_u32);

        let white_rgba = u32::from(&RGBA::color(0.0, 0.0, 0.0));
        let white_u32 = 0;
        assert_eq!(white_rgba, white_u32);

        let green_rgba = u32::from(&RGBA::color(0.0, 1.0, 0.0));
        let green_u32 = (u16::MAX as u32) ^ (u8::MAX as u32);
        assert_eq!(green_rgba, green_u32);
    }
}