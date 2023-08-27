use nalgebra::Vector3;

use crate::{geometry::{Intersectable, Point}, material::Material};


pub trait Object: Intersectable {
    fn surface_normal(&self, point: &Point) -> Vector3<f32>;
    fn material(&self) -> &Box<dyn Material>;
}