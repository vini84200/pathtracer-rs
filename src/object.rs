use nalgebra::Vector3;

use crate::{geometry::{Intersectable, Point}, material::Material, world::Intersection};


pub trait Object: Intersectable {
    fn surface_normal(&self, point: &Point) -> Vector3<f32>;
    fn material(&self) -> &Box<dyn Material + Send + Sync>;
    fn intersection<'a>(&'a self, r: &crate::geometry::Ray, b: &'a Box<dyn Object + Send + Sync>) -> Option<Intersection> {
        if let Some(distance) = self.intersect(r) {
            Some(Intersection {
                distance,
                point: r.point_at(distance),
                object: b,
                normal: self.surface_normal(&r.point_at(distance)),
            })
        } else {None}
    }
}