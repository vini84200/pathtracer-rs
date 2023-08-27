use std::f32::EPSILON;

use image::{DynamicImage, GenericImage};
use nalgebra::Vector3;

use crate::{geometry::{Point, Ray}, camera::Camera, world::World, color::ColorF32};
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
                self.trace(r, x, y);

            }
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {} ms ({:.2} fps)", elapsed.as_millis(), 1000.0 / elapsed.as_millis() as f32);
    }

    fn trace(&mut self, r: Ray, x: u32, y: u32) {
        if let Some(intersection) = self.world.intersect(&r) {
            let color = self.object_color(intersection, x, y);
            self.image.put_pixel(x, y, color.into());
        } else {
            let c = self.world.background_color(r);
            self.image.put_pixel(x, y, c);
        }
    }

    pub fn present(&self) -> &image::DynamicImage {
        // ...

        &self.image
    }

    pub(crate) fn world(&mut self) -> &mut World{
        &mut self.world
    }

    fn object_color(&self, intersection: crate::world::Intersection<'_>, _x: u32, _y: u32) -> ColorF32 {
        let mut acc = ColorF32::new(0.0, 0.0, 0.0);
        for light in &self.world.lights {
            let direction_to_light = light.direction_from(&intersection.point).normalize();
            let shadow_ray = Ray::new(intersection.point+direction_to_light*0.0001, direction_to_light);
            let shadow_intersection = self.world.intersect(&shadow_ray);
            if let Some(blocking) = shadow_intersection {
                if blocking.distance < direction_to_light.magnitude()  {
                    continue;
                }
            }
            let light_intensity = light.intensity() / light.attenuation(direction_to_light.magnitude_squared() + EPSILON);
            let light_reflected = intersection.object.material().albedo() / std::f32::consts::PI;
            let light_color = light.color();

            let diffuse_factor = direction_to_light.normalize().dot(&intersection.object.surface_normal(&intersection.point));
            let diffuse_color = intersection.object.material().color();
            let diffuse = *light_color * light_intensity * diffuse_factor * diffuse_color;
            let specular = ColorF32::new(0.0, 0.0, 0.0);
            let color = (diffuse + specular) * light_reflected;
            acc = acc + color
        }
        acc
    }

    pub(crate) fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

}