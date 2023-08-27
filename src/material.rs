use crate::color::ColorF32;


pub trait Material {
    fn color(&self) -> ColorF32;
    fn albedo(&self) -> f32 {
        1.0
    }
    fn is_reflective(&self) -> bool {
        false
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

pub struct Reflective {
    color: ColorF32,
    albedo: f32,
}

impl Reflective {
    pub fn new(color: crate::color::Color<f32>, albedo: f32) -> Self {
        Self {
            color,
            albedo,
        }
    }
}

impl Material for Reflective {
    fn color(&self) -> ColorF32 {
        self.color
    }

    fn albedo(&self) -> f32 {
        self.albedo
    }

    fn is_reflective(&self) -> bool {
        true
    }
}