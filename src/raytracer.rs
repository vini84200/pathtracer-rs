use std::f32::EPSILON;

use image::{DynamicImage, GenericImage};
use nalgebra::Vector3;

use crate::{geometry::{Point, Ray}, camera::Camera, world::World, color::ColorF32, material};
pub struct Pathtracer {
    width: u32,
    height: u32,
    image: DynamicImage,
    world: World,
    camera: Camera,
}

impl Pathtracer {
    pub fn new(width : u32, height : u32) -> Self {
        let camera = Camera::new(
            Point::new(0.0, 0.0, 0.0), 
            Vector3::new(0.0, 0.0, -1.0), 90.0,
            width, 
            height);
        Self {
            width,
            height,
            image: DynamicImage::new_rgba8(width, height),
            world: World::new(),
            camera
        }
    }

    pub fn resize(&mut self, width : u32, height : u32) {
        self.width = width;
        self.height = height;
        self.image = DynamicImage::new_rgba8(width, height);
    }

    pub fn render(&mut self) {

        let now = std::time::Instant::now();
        for y in 0..self.height {
            for x in 0..self.width {
                let r = Ray::new_prime(x, y, &self.camera);
                self.trace(r, x, y, 0);
            }
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {} ms ({:.2} fps)", elapsed.as_millis(), 1000.0 / elapsed.as_millis() as f32);
    }

    fn trace(&mut self, r: Ray, x: u32, y: u32, depth: u16) {
        if let Some(intersection) = self.world.intersect(&r) {
            let mut color = ColorF32::new(0.0, 0.0, 0.0);
            const SINGLE_SHOT_SAMPLES: i32 = 8;
            for _ in 0..SINGLE_SHOT_SAMPLES {
                color = color + self.object_color(&intersection, &r, 0);
            }
            color = color / SINGLE_SHOT_SAMPLES as f32;
            self.image.put_pixel(x, y, color.into());
        } else {
            let c = self.world.background_color(&r);
            self.image.put_pixel(x, y, c.into());
        }
    }

    pub fn present(&self) -> &image::DynamicImage {
        // ...

        &self.image
    }

    pub(crate) fn world(&mut self) -> &mut World{
        &mut self.world
    }
    const RAYS: i32 = 2;
    const MAX_DEPTH: u16 = 100;

    fn object_color(&self, intersection: &crate::world::Intersection<'_>, ray: &Ray, depth: u16) -> ColorF32 {
        let mut acc = ColorF32::new(0.0, 0.0, 0.0);
        // Diffuse
        {
            let mut diffuse = ColorF32::new(0.0, 0.0, 0.0);
            //
            if depth < Self::MAX_DEPTH {
                let random_ray = Ray::new_random(intersection.point, intersection.object.surface_normal(&intersection.point));
                let random_intersection = self.world.intersect(&random_ray);
                if let Some(random_intersection) = random_intersection {
                    diffuse = self.object_color(&random_intersection, &random_ray, depth + 1) * 0.5;
                } else {
                    diffuse = self.world.background_color(&random_ray);
                }
            } else {
                diffuse = self.world.background_color(ray);
            }
            acc = acc + diffuse;
        } 
        // Reflection

        // Refraction

        
        acc
    }

    pub(crate) fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

}

fn get_reflection_vector(normal: &Vector3<f32>, incident: &Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * incident.dot(&normal) * normal
}