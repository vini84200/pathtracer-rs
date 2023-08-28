use std::{borrow::BorrowMut, cell::RefCell};

use crate::{material::{Material, Diffuse}, color::ColorF32, camera::Camera, object::Object, world::Intersection};
use bvh::{bvh::BVH, aabb::Bounded, bounding_hierarchy::BHShape};
use nalgebra::{Vector3, Point3};
use rand::{Rng, distributions::Distribution, thread_rng};
use lazy_static::lazy_static;

pub type Point = Point3<f32>;


pub const ORIGIN: &Point = &Point::new(0.0, 0.0, 0.0);


pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32, color: ColorF32) -> Self {
        Self {
            center: Point::new(x, y, z),
            radius,
            material:Box::new(Diffuse::new(color)),

        }
    }
    pub fn new_with_material(x: f32, y: f32, z: f32, radius: f32, material: Box<dyn Material+Send+Sync>) -> Self {
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
    pub index: usize,
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

pub fn random_in_unit_sphere() -> Vector3<f32> {
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
        if t1 < 0.0 {
            return Some(t2);
        }
        if t2 < 0.0 {
            return Some(t1);
        }

        let t = if t1 < t2 { t1 } else { t2 };
    
        Some(t)
    }
    
}

impl Object for Sphere {
    fn surface_normal(&self, point: &Point) -> Vector3<f32> {
        (point - self.center).normalize()
    }

    fn material(&self) -> &Box<dyn Material + Send + Sync> {
        &self.material
    }
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3<f32>,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Plane {
    pub fn new(origin: Point, normal: Vector3<f32>, material: Box<dyn Material + Send + Sync>) -> Self {
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

    fn material(&self) -> &Box<dyn Material + Send  + Sync> {
        &self.material
    }
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self {
            a,
            b,
            c,
            index: 0,
        }
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let epsilon = 0.0001;
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -epsilon && a < epsilon {
            return None;
        }
        let f = 1.0 / a;
        let s = ray.origin - self.a;
        let u = f * s.dot(&h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if !(0.0..=1.0).contains(&v) {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > epsilon {
            return Some(t);
        } else {
            return None;
        }
    }
}

fn point_to_bvh_point(point: &Point) -> bvh::Point3 {
    bvh::Point3::new(point.x, point.y, point.z)
}

impl Bounded for Triangle {
    fn aabb(&self) -> bvh::aabb::AABB {
        let min_x = self.a.x.min(self.b.x.min(self.c.x));
        let min_y = self.a.y.min(self.b.y.min(self.c.y));
        let min_z = self.a.z.min(self.b.z.min(self.c.z));
        let max_x = self.a.x.max(self.b.x.max(self.c.x));
        let max_y = self.a.y.max(self.b.y.max(self.c.y));
        let max_z = self.a.z.max(self.b.z.max(self.c.z));
        bvh::aabb::AABB::with_bounds(
            point_to_bvh_point(&Point::new(min_x, min_y, min_z)),
            point_to_bvh_point(&Point::new(max_x, max_y, max_z)),
        )
    }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.index
    }
}


pub struct Mesh {
    triangles: Vec<Triangle>,
    aabb: Option<BVH>,
    material: Box<dyn Material + Send + Sync>,
}

impl Mesh {
    pub fn empty(material : Box<dyn Material + Send + Sync>) -> Self {
        Self {
            triangles: Vec::new(),
            aabb: None,
            material
        }
    }

    pub fn from_triangles(triangles: Vec<Triangle>, material : Box<dyn Material + Send + Sync>) -> Self {
        Self {
            triangles,
            aabb: None,
            material
        }
    }

    pub fn build_bvh(&mut self) {
        self.aabb = Some(BVH::build(&mut self.triangles));
    }

    pub fn from_obj_text( text: &str, material : Box<dyn Material + Send + Sync>) -> Self {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        for line in text.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let x = parts.next().unwrap().parse::<f32>().unwrap();
                    let y = parts.next().unwrap().parse::<f32>().unwrap();
                    let z = parts.next().unwrap().parse::<f32>().unwrap();
                    vertices.push(Point::new(x, y, z));
                },
                Some("f") => {
                    // Has to accept f v/vt/vn v/vt/vn v/vt/vn
                    // Or f v//vn v//vn v//vn
                    // Or f v v v
                    
                        let mut vertex = parts.next().unwrap().split('/');
                        let a = vertex.next().unwrap().parse::<usize>().unwrap() - 1;
                        let _ = vertex.next().unwrap().parse::<usize>().unwrap() - 1;
                        let _ = vertex.next().unwrap().parse::<usize>().unwrap() - 1;

                        let mut vertex = parts.next().unwrap().split('/');
                        let b = vertex.next().unwrap().parse::<usize>().unwrap() - 1;
                        let _ = vertex.next().unwrap().parse::<usize>().unwrap() - 1;
                        let _ = vertex.next().unwrap().parse::<usize>().unwrap() - 1;

                        let mut vertex = parts.next().unwrap().split('/');
                        let c = vertex.next().unwrap().parse::<usize>().unwrap() - 1;
                        let _ = vertex.next().unwrap().parse::<usize>().unwrap() - 1;
                        let _ = vertex.next().unwrap().parse::<usize>().unwrap() - 1;
                        println!("{} {} {} ", a, b, c);
                        triangles.push(Triangle::new(vertices[a], vertices[b], vertices[c]));
                },
                _ => {}
            }
        }
        Self::from_triangles(triangles, material)
    }

    pub fn from_obj(path: &std::path::Path, material : Box<dyn Material + Send + Sync>) -> Self {
        let text = std::fs::read_to_string(path).unwrap();
        Self::from_obj_text(&text, material)
    }

}

fn vector_into_bvh_vector(vector: &Vector3<f32>) -> bvh::Vector3 {
    bvh::Vector3::new(vector.x, vector.y, vector.z)
}

impl From<Ray> for bvh::ray::Ray {
    fn from(ray: Ray) -> Self {
        Self::new(point_to_bvh_point(&ray.origin), vector_into_bvh_vector(&ray.direction))
    }
}

impl From<&Ray> for bvh::ray::Ray {
    fn from(ray: &Ray) -> Self {
        Self::new(point_to_bvh_point(&ray.origin), vector_into_bvh_vector(&ray.direction))
    }
}

impl From<bvh::ray::Ray> for Ray {
    fn from(value: bvh::ray::Ray) -> Self {
        Self::new(Point::new(value.origin.x, value.origin.y, value.origin.z), Vector3::new(value.direction.x, value.direction.y, value.direction.z))
    }
}


impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        if let Some(bvh) = &self.aabb {
            let bvh_ray: bvh::ray::Ray = ray.into();
            let hit_aabbs = bvh.traverse(&bvh_ray, &self.triangles);
            let mut closest: Option<f32> = None;
            for hit_aabb in hit_aabbs {
                if let Some(distance) = hit_aabb.intersect(ray) {
                    if closest.is_none() || distance < closest.unwrap() {
                        closest = Some(distance);
                    }
                }
            }
            closest
        } else {
            panic!("Mesh has no BVH")
        }
    }
}

impl Object for Mesh {
    fn surface_normal(&self, point: &Point) -> Vector3<f32> {
        // Slow, transverse all triangles
        let mut closest: Option<&Triangle> = None;
        for triangle in &self.triangles {
            // Test if the point is inside the triangle
            let a = triangle.a - point;
            let b = triangle.b - point;
            let c = triangle.c - point;

            let u = b.cross(&c);
            let v = c.cross(&a);
            let w = a.cross(&b);

            if u.dot(&v) > 0.0 && u.dot(&w) > 0.0 {
                closest = Some(triangle);
                break;
            }
            
        }

        // Now we have the closest triangle, we can calculate the normal
        if let Some(triangle) = closest {
            (triangle.a - triangle.b).cross(&(triangle.a - triangle.c)).normalize()
        } else {
            // No triangle found, return a default normal
            Vector3::new(0.0, 1.0, 0.0)
        }
    }

    fn material(&self) -> &Box<dyn Material + Send + Sync> {
       &self.material
    }

    fn intersection<'a>(&'a self, r: &crate::geometry::Ray, b: &'a Box<dyn Object + Send + Sync>) -> Option<crate::world::Intersection> {
        // Fast intersection with normal

        if let Some(bvh) = &self.aabb {
            let bvh_ray: bvh::ray::Ray = r.into();
            let hit_aabbs = bvh.traverse(&bvh_ray, &self.triangles);
            let mut closest: Option<(&Triangle, f32)> = None;
            for hit_aabb in hit_aabbs {
                if let Some(distance) = hit_aabb.intersect(r) {
                    if closest.is_none() || distance < closest.unwrap().1 {
                        closest = Some((hit_aabb, distance));
                    }
                }
            }
            Some(
                Intersection {
                    distance: closest?.1,
                    point: r.point_at(closest.unwrap().1),
                    object: b,
                    normal: {
                        // Calculate the normal of the triangle
                        let triangle = closest?.0;
                        (triangle.a - triangle.b).cross(&(triangle.a - triangle.c)).normalize()
                    }
                }
            )
        } else {
            panic!("Mesh has no BVH")
        }

    }
}