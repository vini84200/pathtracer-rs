use crate::color::ColorF32;


pub trait Material {
    fn color(&self) -> ColorF32;
    fn albedo(&self) -> f32 {
        1.0
    }
}

pub struct Diffuse {
    color: ColorF32,
}
impl Diffuse {
    pub fn new(color: crate::color::Color<f32>) -> Self {
        Self {
            color,
        }
    }
}

impl Material for Diffuse {
    fn color(&self) -> ColorF32 {
        self.color
    }
}