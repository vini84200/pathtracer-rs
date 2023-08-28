use nalgebra::Vector3;

use crate::{color::ColorF32, geometry::{Intersectable, Ray, self}, world::Intersection};

pub struct Scattering {
    pub ray: Ray,
    pub attenuation: ColorF32,
}

pub trait Material {
    fn color(&self) -> ColorF32;
    fn is_reflective(&self) -> bool {
        false
    }
    fn emissivity(&self) -> ColorF32 {
        ColorF32::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scattering> {
        None
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
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scattering> {
        let direction = geometry::random_lambertian(intersection.object.surface_normal(&intersection.point));
        let direction = if direction.magnitude_squared() < 0.0001 {
            intersection.object.surface_normal(&intersection.point)
        } else {
            direction
        };
        let random_ray = Ray::new_with_eps(intersection.point, direction, 0.00001);
        Some(
            Scattering {
                ray: random_ray,
                attenuation: self.color,
            }
        )
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

pub fn reflect (v : Vector3<f32>, n : Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

pub struct Metal {
    color: ColorF32,
    fuzz: f32,
}

impl Metal {
    pub fn new(color: crate::color::Color<f32>, fuzz: f32) -> Self {
        Self {
            color,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn color(&self) -> ColorF32 {
        self.color
    }

    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scattering> {
        let reflected = reflect(ray.direction, intersection.object.surface_normal(&intersection.point));
        let random_ray = Ray::new_with_eps(intersection.point, reflected + self.fuzz * geometry::random_in_unit_sphere(), 0.001);
        if random_ray.direction.dot(&intersection.object.surface_normal(&intersection.point)) > 0.0 {
            Some(
                Scattering {
                    ray: random_ray,
                    attenuation: self.color,
                }
            )
        } else {
            None
        }
    }
    
}