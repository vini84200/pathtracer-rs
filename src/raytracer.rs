use std::f32::EPSILON;

use image::{DynamicImage, Rgba32FImage};
use nalgebra::Vector3;
use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::{geometry::{Point, Ray}, camera::Camera, world::World , color::{ColorF32, self} };
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
        if self.samples == 0 {
            println!("Rendering {}x{} image", self.width, self.height);
            self.started = std::time::Instant::now();
        }
        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         let r = Ray::new_prime(x, y, &self.camera);
        //         self.trace(r, x, y, 0)
        //     }
        // }
        let color = (0..self.height).par_bridge().map(|y| {
            (0..self.width).par_bridge().map(|x| {
                let r = Ray::new_prime(x, y, &self.camera);
                (x, y, self.trace(r, x, y, 0))
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        for y in 0..self.height {
            for x in 0..self.width {
                let (x, y, color) = &color[y as usize][x as usize];
                self.image.put_pixel(*x, *y, (*color).into());
            }
        }
        self.samples += Self::SINGLE_SHOT_SAMPLES as u64;
        let elapsed = now.elapsed();
        let total_elapsed = self.started.elapsed();
        println!("Elapsed: {:?} (fps: {}, {:?}) {} samples ({} ms/sa)", total_elapsed, 1.0 / (elapsed.as_secs_f32() + EPSILON), elapsed, self.samples, total_elapsed.as_millis() as f32 / self.samples as f32);


    }
    const SINGLE_SHOT_SAMPLES: i32 = 64;

    fn trace(&self, r: Ray, x: u32, y: u32, depth: u16) -> ColorF32 {
        let color = if let Some(intersection) = self.world.intersect(&r) {
            let intersection =Box::new( intersection);
            let color = (0..Self::SINGLE_SHOT_SAMPLES).map(|_| {
                self.ray_color(&intersection, &r, depth)
                // }).reduce(||->ColorF32 {ColorF32::new(0.0,0.0,0.0)}, |x, y| x + y);
                }).reduce(|x, y| x + y).unwrap();
            color/(Self::SINGLE_SHOT_SAMPLES as f32)
        } else {
           self.world.background_color(&r)
        };
        // Load the color from the current pixel
        if self.samples > 0 {
            let orig = self.image.get_pixel(x, y);
            let orig : ColorF32 = orig.into();
            orig * (self.samples as f32 / (self.samples as f32 + Self::SINGLE_SHOT_SAMPLES as f32)) + color * (Self::SINGLE_SHOT_SAMPLES as f32 / (self.samples as f32 + Self::SINGLE_SHOT_SAMPLES as f32))
        } else {
            color
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

    fn ray_color(&self, intersection: &crate::world::Intersection<'_>, ray: &Ray, depth: u16) -> ColorF32 {
        let emisivty  = intersection.object.material().emissivity();
        let material = intersection.object.material();
        // Diffuse
        let diffuse = if depth < Self::MAX_DEPTH {
            let scatter = material.scatter(ray, intersection);
            if let Some(scatter) = scatter {
                let random_ray = scatter.ray;
                
                let random_intersection = self.world.intersect(&random_ray);
                if let Some(random_intersection) = random_intersection {
                    self.ray_color(&random_intersection, &random_ray, depth + 1) * scatter.attenuation
                } else {
                    self.world.background_color(&random_ray) * scatter.attenuation
                }
            } else {
                // No scatter, so no color
                color::BLACK
            }
        } else {
            // Max depth reached, don't recurse
            material.color() * 0.1
            
        };
        // Reflection

        // Refraction

        
        emisivty + diffuse
    }

    pub(crate) fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

}
