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
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scattering>;
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
        let direction = geometry::random_lambertian(intersection.normal);
        let direction = if direction.magnitude_squared() < 0.0001 {
            intersection.normal
        } else {
            direction
        };
        let random_ray = Ray::new_with_eps(intersection.point, direction, 0.001);
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

    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scattering> {
        None
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
        let reflected = reflect(ray.direction, intersection.normal);
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

pub struct Dielectric {
    color: ColorF32,
    fuzz: f32,
    refraction_index: f32,
}

fn refract(v: Vector3<f32>, normal: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let cos_theta = (-uv).dot(&normal).min(1.0);
    let r_out_perp = ni_over_nt * (uv + cos_theta * normal);
    let r_out_parallel = (-((1.0 - r_out_perp.magnitude_squared()).abs().sqrt())) * normal;
    Some(r_out_perp + r_out_parallel)
}

impl Dielectric {
    pub fn new(color: crate::color::Color<f32>, fuzz: f32, refraction_index: f32) -> Self {
        Self {
            color,
            fuzz,
            refraction_index,
        }
    }
}

impl Material for Dielectric {
    fn color(&self) -> ColorF32 {
        self.color
    }

    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scattering> {
        let attenuation = self.color;
        let front_face = ray.direction.dot(&intersection.normal) < 0.0;
        let refraction_ratio = if front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let normal = if front_face {
            intersection.normal
        } else {
            -intersection.normal
        };
        let refracted = refract(ray.direction, normal, refraction_ratio);
        let refracted_fuzz = refracted? + self.fuzz * geometry::random_in_unit_sphere();
        let scattered = Ray::new_with_eps(intersection.point, refracted_fuzz, 0.1);
        Some(
            Scattering {
                ray: scattered,
                attenuation,
            }
        )
    }
}