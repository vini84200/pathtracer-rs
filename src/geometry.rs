pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}