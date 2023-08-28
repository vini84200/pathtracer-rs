use std::{borrow::BorrowMut, cell::RefCell};

use crate::{material::{Material, Diffuse}, color::ColorF32, camera::Camera, object::Object};
use nalgebra::{Vector3, Point3};
use rand::{Rng, distributions::Distribution, thread_rng};
use lazy_static::lazy_static;

pub type Point = Point3<f32>;


pub const ORIGIN: &Point = &Point::new(0.0, 0.0, 0.0);


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
        let camera_direction = camera.direction().to_owned();
        let camera_origin = camera.origin().to_owned();
        let fov = camera.fov().to_radians();
        let width = camera.width();
        let height = camera.height();

        let aspect_ratio = width as f32 / height as f32;
        let sensor_x = (((x as f32 + 0.5) / width as f32) * 2.0 - 1.0) * aspect_ratio * (fov / 2.0).tan();
        let sensor_y = (1.0 - ((y as f32 + 0.5) / height as f32) * 2.0) * (fov / 2.0).tan();

        let direction = Vector3::new(sensor_x, sensor_y, -1.0).normalize();
        
        // Now we have a direction vector, but it's in camera space. We need to transform it into world space.
        // We can do this by rotating the vector by the camera's rotation matrix.


        Self {
            origin: camera_origin,
            direction,
        }
    }

    pub(crate) fn point_at(&self, distance: f32) -> nalgebra::OPoint<f32, nalgebra::Const<3>> {
        self.origin + (self.direction * distance)
    }

    pub(crate) fn new_random_hemi(point: nalgebra::OPoint<f32, nalgebra::Const<3>>, normal: Vector3<f32>) -> Ray {
        let mut rng = rand::thread_rng();
        let mut direction = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(-1.0..1.0));
        if direction.dot(&normal) < 0.0 {
            direction = -direction;
        }
        Ray {
            origin: point,
            direction: direction.normalize(),
        }
    }

    pub(crate) fn new_with_eps(point: Point, direction: Vector3<f32>, eps: f32) -> Ray {
        Self { origin: point + (direction * eps), direction }
    }
}

lazy_static! {
    pub static ref DISTRIBUTION : rand::distributions::Uniform<f32> = rand::distributions::Uniform::new(-1.0, 1.0);
}

thread_local! {
    pub static RNG : RefCell<rand_pcg::Pcg64> = RefCell::new(rand_pcg::Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7));
}

fn random_in_unit_sphere() -> Vector3<f32> {
    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        // let mut rng = thread_rng();
        let mut a = Vector3::new(rng.sample(*DISTRIBUTION), rng.sample(*DISTRIBUTION), rng.sample(*DISTRIBUTION));
        while a.magnitude_squared() >= 1.0 {
            a = Vector3::new(rng.sample(*DISTRIBUTION), rng.sample(*DISTRIBUTION), rng.sample(*DISTRIBUTION));
        }
        a
    })
}

fn random_in_hemisphere(normal: Vector3<f32>) -> Vector3<f32> {
    let a = random_in_unit_sphere().normalize();
    if a.dot(&normal) > 0.0 {
        a
    } else {
        -a
    }
}

pub fn random_lambertian(normal: Vector3<f32>) -> Vector3<f32> {
    let a = random_in_unit_sphere().normalize();
    a + normal
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

impl Object for Sphere {
    fn surface_normal(&self, point: &Point) -> Vector3<f32> {
        (point - self.center).normalize()
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3<f32>,
    pub material: Box<dyn Material>,
}

impl Plane {
    pub fn new(origin: Point, normal: Vector3<f32>, material: Box<dyn Material>) -> Self {
        Self {
            origin,
            normal,
            material,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        let denom = self.normal.dot(&ray.direction);
        let t = (self.origin - ray.origin).dot(&self.normal) / denom;
        if denom == 0.0 {
            // ray is parallel to plane
            // we consider this to be no intersection 
            return None;
        }
        if t < 0.0 {
            // plane is behind ray
            return None;
        }
        Some(t)

    }
}

impl Object for Plane {
    fn surface_normal(&self, _point: &Point) -> Vector3<f32> {
        self.normal
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}