use crate::vec3::Vector3;

struct Ray
{
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction
        }
    }
    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}
