use std::f32::consts::PI;

use crate::{object::Object, light::Light, color::{ColorF32, self}};

pub struct Intersection<'a> {
    pub distance: f32,
    pub point: nalgebra::Point3<f32>,
    pub object: &'a Box<dyn Object + Send+Sync>,
}

pub struct World {
    pub objects: Vec<Box<dyn Object + Send+Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object +Send+Sync>) {
        self.objects.push(object);
    }


    pub fn intersect(&self, r: &crate::geometry::Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;
        for object in &self.objects {
            if let Some(distance) = object.intersect(r) {
                if closest.is_none() || distance < closest.as_ref().unwrap().distance {
                    closest = Some(Intersection {
                        distance,
                        point: r.point_at(distance),
                        object
                    });
                }
            }
        }
        closest
    }
    const SUN_SIZE_DEGREES : f32 = 0.53;
    const SUN_INTENSITY: f32 = 0.4;

    const Br : f32 = 0.0025;
    const Bm : f32 = 0.0003;
    const g: f32 =  0.9800;

    pub fn background_color(&self, r: &crate::geometry::Ray) -> ColorF32 {
        let direction = r.direction.normalize();
        if direction.y < 0.0 {
            return color::BLACK;
        }
        let sun_direction = nalgebra::Vector3::new(0.3, -1.1,- 0.3);
        let sun_direction = sun_direction.normalize();
        // https://github.com/shff/opengl_sky
        let nitrogen : ColorF32 = ColorF32::new(0.650, 0.570, 0.475);
        let kr = Self::Br / ColorF32::pow(nitrogen, 4.0);
        let km = Self::Bm / ColorF32::pow(nitrogen, 0.84);
        let mu = direction.dot(&sun_direction);
        // // Draw a sun if the ray is pointing at the sun 
        // if mu > sun_size {
        //     return ColorF32::new(1.0, 1.0, 0.0) * Self::SUN_INTENSITY;
        // }
        let rayleigh = 3.0 / (8.0 * PI) * (1.0 + mu * mu);
        let g = Self::g;
        let mie = (kr + km * (1.0 - g * g) / (2.0 + g * g) / (1.0 + g * g - 2.0 * g * mu).powf(1.5)) / (Self::Br + Self::Bm);
        let day_extinction = 
            (-(-((direction.y + sun_direction.y * 4.0) *
            ((-direction.y * 16.0).exp() + 0.1) / 80.0) / Self::Br).exp() *
            ((-direction.y * 16.0).exp() + 0.1) * kr / Self::Br).exp() *
         (-direction.y * (-direction.y * 8.0 ).exp() * 4.0).exp() * 
         (-direction.y * 2.0).exp() * 4.0;

        let night_extinction =1.0 - (-sun_direction.y).exp() * 0.2;
        let night_extinction = ColorF32::new(night_extinction, night_extinction, night_extinction);
        let t = 0.5 * (direction.y + 1.0);
        let extinction = night_extinction * (1.0 - t) + day_extinction * t;
        extinction *rayleigh * mie * Self::SUN_INTENSITY

        // let white = ColorF32::new(1.0, 1.0, 1.0);
        // let blue = ColorF32::new(0.5, 0.7, 1.0);
        // let t = 0.5 * (direction.y + 1.0);
        // ColorF32::lerp(white, blue, t)
    }
}