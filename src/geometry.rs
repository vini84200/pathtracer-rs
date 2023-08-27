use crate::{material::{Material, Diffuse}, color::ColorF32, camera::Camera};
use nalgebra::{Vector3, Point3};

pub type Point = Point3<f32>;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32, color: ColorF32) -> Self {
        Self {
            center: Point::new(x, y, z),
            radius,
            material:Box::new(Diffuse::new(color)),

        }
    }
    pub fn new_with_material(x: f32, y: f32, z: f32, radius: f32, material: Box<dyn Material>) -> Self {
        Self {
            center: Point::new(x, y, z),
            radius,
            material,

        }
    }

}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3<f32>) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn new_prime (x : u32, y : u32, camera : &Camera) -> Self {
        let fov_adjustment = (camera.fov().to_radians() / 2.0).tan();
        let aspect_ratio = (camera.width() as f32) / (camera.height() as f32);
        let sensor_x = ((((x as f32 + 0.5) / camera.width() as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f32 + 0.5) / camera.height() as f32) * 2.0) * fov_adjustment;

        let direction = Vector3::new(sensor_x, sensor_y, -1.0).normalize();

        Self::new(camera.origin().to_owned(), direction)
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let ctor = self.center- ray.origin;
        let v = ctor.dot(&ray.direction);
        let discriminant = (self.radius * self.radius) - (ctor.dot(&ctor) - v * v);
        if discriminant < 0.0 {
            return None;
        }
        let d = discriminant.sqrt();
        let t1 = v - d;
        let t2 = v + d;
        if t1 < 0.0 && t2 < 0.0 {
            return None;
        }

        let t = if t1 < t2 { t1 } else { t2 };
    
        Some(t)
    }
    
}