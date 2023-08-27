use image::Pixel;


#[derive(Debug, Copy, Clone)]
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

impl From<ColorF32> for image::Rgba<u8> {
    fn from(val: ColorF32) -> Self {
        image::Rgba::from_slice(&[
            (val.r * 255.0).min(255.) as u8,
            (val.g * 255.0).min(255.) as u8,
            (val.b * 255.0).min(255.) as u8,
            (val.a * 255.0).min(255.) as u8,
        ]).to_owned()
    }
    
}

pub const RED : ColorF32 = ColorF32 { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
pub const GREEN : ColorF32 = ColorF32 { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
pub const BLUE : ColorF32 = ColorF32 { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
pub const WHITE : ColorF32 = ColorF32 { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
pub const BLACK : ColorF32 = ColorF32 { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };