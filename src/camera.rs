use crate::random;
use crate::ray::Ray;
use crate::vec3::Vector3;
use std::f32;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f32, aspect: f32, aperture :f32, focus_dist:f32) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = lookfrom;
        let w = (lookfrom - lookat).get_unit_vector();
        let u = (vup.cross(&w)).get_unit_vector();
        let v = w.cross(&u);

        Camera {
            origin,
            lower_left_corner: origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t:f32) -> Ray {
        let rd = self.lens_radius * random::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}
