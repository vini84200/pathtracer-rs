use nalgebra::Vector3;

use crate::{geometry::{Intersectable, Point, ORIGIN}, color::ColorF32};

pub trait Light : Intersectable {
    fn position(&self) -> &Point;
    fn color(&self) -> &ColorF32;
    fn intensity(&self) -> f32;
    fn direction_from(&self, point: &Point) -> Vector3<f32>;
    fn attenuation(&self, distance_sqr: f32) -> f32 {
        1.0 / (1.0 + distance_sqr)
    }
}

pub struct PointLight {
    position: Point,
    color: ColorF32,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Point, color: ColorF32, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }
}

impl Intersectable for PointLight {
    fn intersect(&self, ray: &crate::geometry::Ray) -> Option<f32> {
        // If the ray direction is the same as the vector from the ray origin to the light position, 
        // the ray is parallel to the light vector and will never intersect the light.
        let v = self.position - ray.origin;
        let size = v.magnitude();
        let p = ray.point_at(size);
        if (p - self.position).norm_squared() < 0.0001 {
            Some(size)
        } else {
            None
        }
    }
}

impl Light for PointLight {
    fn position(&self) -> &Point {
        &self.position
    }

    fn color(&self) -> &ColorF32 {
        &self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn direction_from(&self, point: &Point) -> Vector3<f32> {
        (self.position - point).normalize()
    }
}

pub struct DirectionalLight {
    direction: Vector3<f32>,
    color: ColorF32,
    intensity: f32,
}

impl DirectionalLight {
    pub fn new(direction: Vector3<f32>, color: ColorF32, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }
}

impl Intersectable for DirectionalLight {
    fn intersect(&self, _ray: &crate::geometry::Ray) -> Option<f32> {
        // Directional lights are infinitely far away, so they never intersect with a ray.
        None
    }
}

impl Light for DirectionalLight {
    fn position(&self) -> &Point {
        ORIGIN
    }

    fn color(&self) -> &ColorF32 {
        &self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn direction_from(&self, _point: &Point) -> Vector3<f32> {
        -self.direction
    }
}