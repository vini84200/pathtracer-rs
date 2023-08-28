use crate::color::ColorF32;


pub trait Material {
    fn color(&self) -> ColorF32;
    fn albedo(&self) -> f32 {
        1.0
    }
    fn is_reflective(&self) -> bool {
        false
    }
    fn emissivity(&self) -> ColorF32 {
        ColorF32::new(0.0, 0.0, 0.0)
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


pub struct Emmisive {
    color: ColorF32,
    intensity: f32,
}

impl Emmisive {
    pub fn new(color: crate::color::Color<f32>, intensity: f32) -> Self {
        Self {
            color,
            intensity,
        }
    }
}

impl Material for Emmisive {
    fn color(&self) -> ColorF32 {
        self.color
    }

    fn emissivity(&self) -> ColorF32 {
        self.color * self.intensity
    }
}
