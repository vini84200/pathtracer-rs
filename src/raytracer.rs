use std::f32::EPSILON;

use image::{DynamicImage, GenericImage, GenericImageView, Rgba32FImage};
use nalgebra::Vector3;

use crate::{geometry::{Point, Ray, self}, camera::Camera, world::World, color::{ColorF32, self}, material};
pub struct Pathtracer {
    width: u32,
    height: u32,
    image: Rgba32FImage,
    world: World,
    camera: Camera,
    samples: u64,
    started: std::time::Instant,
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
            image: Rgba32FImage::new(width, height),
            world: World::new(),
            camera,
            samples: 0,
            started: std::time::Instant::now(),
        }
    }

    pub fn resize(&mut self, width : u32, height : u32) {
        self.width = width;
        self.height = height;
        self.image = Rgba32FImage::new(width, height);
        self.samples = 0;
    }

    pub fn render(&mut self) {

        let now = std::time::Instant::now();
        for y in 0..self.height {
            for x in 0..self.width {
                let r = Ray::new_prime(x, y, &self.camera);
                self.trace(r, x, y, 0);
            }
        }
        self.samples += Self::SINGLE_SHOT_SAMPLES as u64;
        let elapsed = now.elapsed();
        let totalElapsed = self.started.elapsed();
        println!("Elapsed: {:?} (fps: {}, {:?}) Samples: {}", totalElapsed, 1.0 / (elapsed.as_secs_f32() + EPSILON), elapsed, self.samples);
    }
    const SINGLE_SHOT_SAMPLES: i32 = 8;

    fn trace(&mut self, r: Ray, x: u32, y: u32, depth: u16) {
        let color = if let Some(intersection) = self.world.intersect(&r) {
            let mut color = ColorF32::new(0.0, 0.0, 0.0);
            for _ in 0..Self::SINGLE_SHOT_SAMPLES {
                color = color + self.object_color(&intersection, &r, depth);
            }
            color = color / Self::SINGLE_SHOT_SAMPLES as f32;
            color
        } else {
           self.world.background_color(&r)
        };
        // Load the color from the current pixel
        if self.samples > 0 {
            let orig = self.image.get_pixel(x, y);
            let orig : ColorF32 = orig.into();
            let new = orig * (self.samples as f32 / (self.samples as f32 + Self::SINGLE_SHOT_SAMPLES as f32)) + color * (Self::SINGLE_SHOT_SAMPLES as f32 / (self.samples as f32 + Self::SINGLE_SHOT_SAMPLES as f32));
            self.image.put_pixel(x, y, new.into());
        } else {
            self.image.put_pixel(x, y, color.into());
        }

    }

    pub fn present(&self) ->image::DynamicImage { 
        // ...
        DynamicImage::ImageRgba32F(self.image.clone())
    }

    pub(crate) fn world(&mut self) -> &mut World{
        &mut self.world
    }
    const MAX_DEPTH: u16 = 4;

    fn object_color(&self, intersection: &crate::world::Intersection<'_>, ray: &Ray, depth: u16) -> ColorF32 {
        let mut acc = intersection.object.material().emissivity();
        // Diffuse
        {
            let diffuse : ColorF32;
            //
            if depth < Self::MAX_DEPTH {
                let direction = geometry::random_in_hemi_lamberian(intersection.object.surface_normal(&intersection.point));
                let random_ray = Ray::new_with_eps(intersection.point, direction, 0.001);
                
                let random_intersection = self.world.intersect(&random_ray);
                if let Some(random_intersection) = random_intersection {
                    diffuse = self.object_color(&random_intersection, &random_ray, depth + 1) * intersection.object.material().color();
                } else {
                    diffuse = self.world.background_color(&random_ray) * intersection.object.material().color();
                }
            } else {
                diffuse = color::BLACK;
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