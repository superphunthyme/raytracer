use std::ops::Add;
use std::ops::Mul;

pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    // pub fn new() {}
    // Use Option to allow for empty contructor
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{x, y, z}
    }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
    pub fn r(&self) -> f32 { self.x }
    pub fn g(&self) -> f32 { self.y }
    pub fn b(&self) -> f32 { self.z }

}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
