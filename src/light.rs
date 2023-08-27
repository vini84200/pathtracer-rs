use nalgebra::Vector3;

use crate::{geometry::{Intersectable, Point}, color::ColorF32};

pub trait Light : Intersectable {
    fn position(&self) -> &Point;
    fn color(&self) -> &ColorF32;
    fn intensity(&self) -> f32;
    fn direction_from(&self, point: &Point) -> Vector3<f32>;
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