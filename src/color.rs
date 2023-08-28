use std::ops::{Mul, Add, Div};

use image::Pixel;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}


pub type ColorF32 = Color<f32>;
pub type ColorU8 = Color<u8>;

impl From<ColorU8> for image::Rgba<u8> {
    fn from(val: ColorU8) -> Self {
        image::Rgba([val.r, val.g, val.b, val.a])
    }
}

impl ColorF32 {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: 1.0,
        }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        assert!(t >= 0.0 && t <= 1.0);
        a * (1.0 - t) + b * t
    }

    pub fn pow(self, power: f32) -> Self {
        Self::new(self.r.powf(power), self.g.powf(power), self.b.powf(power))
    }

    pub fn exp(self) -> Self {
        Self::new(self.r.exp(), self.g.exp(), self.b.exp())
    }

}

impl Mul<f32> for ColorF32 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Self> for ColorF32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Div<f32> for ColorF32 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl Div<Self> for ColorF32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b)
    }
}

impl Mul<ColorF32> for f32 {
    type Output = ColorF32;

    fn mul(self, rhs: ColorF32) -> Self::Output {
        ColorF32::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }

    
}

impl Div<ColorF32> for f32 {
    type Output = ColorF32;

    fn div(self, rhs: ColorF32) -> Self::Output {
        ColorF32::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl Add<Self> for ColorF32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl From<ColorF32> for image::Rgba<u8> {
    fn from(val: ColorF32) -> Self {
        image::Rgba::from_slice(&[
            (val.r * 255.0).clamp(0., 255.) as u8,
            (val.g * 255.0).clamp(0., 255.) as u8,
            (val.b * 255.0).clamp(0., 255.) as u8,
            (val.a * 255.0).clamp(0., 255.) as u8,
        ]).to_owned()
    }
}

impl From<image::Rgba<u8>> for ColorF32 {
    fn from(val: image::Rgba<u8>) -> Self {
        Self {
            r: val[0] as f32 / 255.0,
            g: val[1] as f32 / 255.0,
            b: val[2] as f32 / 255.0,
            a: val[3] as f32 / 255.0,
        }
    }
}

impl From<image::Rgba<f32>> for ColorF32 {
    fn from(val: image::Rgba<f32>) -> Self {
        Self {
            r: val[0],
            g: val[1],
            b: val[2],
            a: val[3],
        }
    }
}
impl From<&image::Rgba<f32>> for ColorF32 {
    fn from(val: &image::Rgba<f32>) -> Self {
        Self {
            r: val[0],
            g: val[1],
            b: val[2],
            a: val[3],
        }
    }
}

impl From<ColorF32> for image::Rgba<f32> {
    fn from(val: ColorF32) -> Self {
        image::Rgba::from_slice(&[
            if val.r.is_normal() { val.r } else { 0.0 },
            if val.g.is_normal() { val.g } else { 0.0 },
            if val.b.is_normal() { val.b } else { 0.0 },
            if val.a.is_normal() { val.a } else { 0.0 },
        ]).to_owned()
    }
}


pub const RED : ColorF32 = ColorF32 { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
pub const GREEN : ColorF32 = ColorF32 { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
pub const BLUE : ColorF32 = ColorF32 { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
pub const WHITE : ColorF32 = ColorF32 { r: 0.98, g: 0.98, b: 0.98, a: 1.0 };
pub const GRAY : ColorF32 = ColorF32 { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
pub const BLACK : ColorF32 = ColorF32 { r: 0.01, g: 0.01, b: 0.01, a: 1.0 };
pub const ORANGE : ColorF32 = ColorF32 { r: 1.0, g: 0.5, b: 0.0, a: 1.0 };

#[cfg(test)]
mod tests {

    // Test that the color is converted to an image::Rgba<u8> correctly
    #[test]
    fn converts_to_rgba_u8() {
        let color = super::ColorF32::new(0.5, 0.5, 0.5);
        let rgba = image::Rgba::from(color);
        assert_eq!(rgba, image::Rgba([127, 127, 127, 255]));
    }
    #[test]
    fn converts_from_rgba_u8() {
        let rgba = image::Rgba([127, 127, 127, 255]);
        let color = super::ColorF32::from(rgba);
        assert_eq!(color, super::ColorF32::new(127.0 / 255.0, 127.0 / 255.0, 127.0 / 255.0));
    }

    #[test]
    fn converts_to_rgba_f32() {
        let color = super::ColorF32::new(0.5, 0.5, 0.5);
        let rgba = image::Rgba::from(color);
        assert_eq!(rgba, image::Rgba([127, 127, 127, 255]));
    }

    

}
