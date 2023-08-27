use nalgebra::Vector3;

use crate::geometry::Point;

pub struct Camera {
    origin: Point,
    direction: Vector3<f32>,

    fov: f32,
    width: u32,
    height: u32,

    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
}

impl Camera {
    pub fn new(origin: Point, direction: Vector3<f32>, fov: f32, width: u32, height: u32) -> Self {
        Self {
            origin,
            direction,
            fov,
            width,
            height,

            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,

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

    pub(crate) fn move_forward(&mut self, arg: bool)  {
        self.move_forward = arg;
    }

    pub(crate) fn move_backward(&mut self, arg: bool)  {
        self.move_backward = arg;
    }

    pub(crate) fn move_left(&mut self, arg: bool)  {
        self.move_left = arg;
    }

    pub(crate) fn move_right(&mut self, arg: bool)  {
        self.move_right = arg;
    }

    pub fn update(&mut self, delta_time: f32) {
        let direction = self.direction.to_owned();
        let mut origin = self.origin.to_owned();

        if self.move_forward {
            origin += direction * delta_time;
        }
        if self.move_backward {
            origin -= direction * delta_time;
        }
        if self.move_left {
            origin -= direction.cross(&Vector3::y()).normalize() * delta_time;
        }
        if self.move_right {
            origin += direction.cross(&Vector3::y()).normalize() * delta_time;
        }

        self.origin = origin;
        self.direction = direction;
    }

    pub(crate) fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        let mut direction = self.direction.to_owned();
        let origin = self.origin.to_owned();

        direction += Vector3::new(delta_y, delta_x, 0.0) * 0.01;
        direction.normalize_mut();


        self.origin = origin;
        self.direction = direction;
    }



}