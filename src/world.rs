use crate::{object::Object, light::Light, color::{self, Color, ColorF32}};

pub struct Intersection<'a> {
    pub distance: f32,
    pub point: nalgebra::Point3<f32>,
    pub object: &'a Box<dyn Object>,
}

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
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

    pub(crate) fn background_color(&self, r: crate::geometry::Ray) -> image::Rgba<u8> {
        let direction = r.direction.normalize();
        let t = 0.5 * (direction.y + 1.0);
            let white = ColorF32::new(1.0, 1.0, 1.0);
        let blue = ColorF32::new(0.5, 0.7, 1.0);
        ColorF32::lerp(white, blue, t).into()
    }
}