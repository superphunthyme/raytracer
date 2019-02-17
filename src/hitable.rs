use crate::ray::Ray;
use crate::vec3::Vector3;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
