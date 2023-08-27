use image::{DynamicImage, GenericImage, Rgba, Pixel};
use nalgebra::Vector3;

use crate::{color, geometry::{Sphere, Point, Ray, Intersectable}, camera::Camera};
pub struct Pathtracer {
    width: u32,
    height: u32,
    image: DynamicImage,
}

impl Pathtracer {
    pub fn new(width : u32, height : u32) -> Self {
        Self {
            width,
            height,
            image: DynamicImage::new_rgba8(width, height),
        }
    }

    pub fn resize(&mut self, width : u32, height : u32) {
        self.width = width;
        self.height = height;
        self.image = DynamicImage::new_rgba8(width, height);
    }

    pub fn render(&mut self) {

        let sphere = Sphere::new(0.0, 0.0, -7.0, 1.0, color::GREEN);
        let camera = Camera::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0), 90.0, self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let r = Ray::new_prime(x, y, &camera);
                self.trace(&sphere, r, x, y);

            }
        }
    }

    fn trace(&mut self, sphere: &Sphere, r: Ray, x: u32, y: u32) {
        if let Some(d) = sphere.intersect(&r) {
            let p = r.origin + (r.direction * d);
            let c = sphere.material.color();
            self.image.put_pixel(x, y, c.into());
        } else {
            let c = Rgba::from(color::BLACK);
            self.image.put_pixel(x, y, c);
        }
    }

    pub fn present(&self) -> &image::DynamicImage {
        // ...

        &self.image
    }
}