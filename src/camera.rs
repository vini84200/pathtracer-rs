use nalgebra::Vector3;

use crate::geometry::Point;

pub struct Camera {
    origin: Point,
    direction: Vector3<f32>,

    fov: f32,
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(origin: Point, direction: Vector3<f32>, fov: f32, width: u32, height: u32) -> Self {
        Self {
            origin,
            direction,
            fov,
            width,
            height
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3<f32> {
        &self.direction
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }


}