use crate::ray::Ray;
use crate::vec3::Vector3;

#[derive(Clone)]
pub struct AABB {
    min: Vector3,
    max: Vector3,
}

impl AABB {
    pub fn new(a: Vector3, b: Vector3) -> AABB {
        AABB { min: a, max: b }
    }

    pub fn min(&self) -> Vector3 {
        self.min
    }

    pub fn max(&self) -> Vector3 {
        self.max
    }

    pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        let inv_dx = 1.0 / r.direction().x();
        let mut t0x = (self.min().x() - r.origin().x()) * inv_dx;
        let mut t1x = (self.max.x() - r.origin().x()) * inv_dx;

        if inv_dx < 0.0 {
            std::mem::swap(&mut t0x, &mut t1x);
        }

        if t0x > t_min {
            t_min = t0x;
        }

        if t1x < t_min {
            t_max = t1x;
        }

        if t_max <= t_min {
            return false;
        }

        let inv_dy = 1.0 / r.direction().y();
        let mut t0y = (self.min().y() - r.origin().y()) * inv_dy;
        let mut t1y = (self.max.y() - r.origin().y()) * inv_dy;

        if inv_dy < 0.0 {
            std::mem::swap(&mut t0y, &mut t1y);
        }

        if t0y > t_min {
            t_min = t0y;
        }

        if t1y < t_min {
            t_max = t1y;
        }

        if t_max <= t_min {
            return false;
        }

        let inv_dz = 1.0 / r.direction().z();
        let mut t0z = (self.min().z() - r.origin().z()) * inv_dz;
        let mut t1z = (self.max.z() - r.origin().z()) * inv_dz;

        if inv_dz < 0.0 {
            std::mem::swap(&mut t0z, &mut t1z);
        }

        if t0z > t_min {
            t_min = t0z;
        }

        if t1z < t_min {
            t_max = t1z;
        }

        if t_max <= t_min {
            return false;
        }

        true
    }

    pub fn surrounding_box(&self, other: &AABB) -> AABB {
        AABB::new(
            Vector3::new(
                self.min.x().min(other.min.x()),
                self.min.y().min(other.min.y()),
                self.min.z().min(other.min.z()),
            ),
            Vector3::new(
                self.max.x().max(other.max.x()),
                self.max.y().max(other.max.y()),
                self.max.z().max(other.max.z()),
            ),
        )
    }
}
