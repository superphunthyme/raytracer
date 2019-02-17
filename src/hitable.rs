use crate::ray::Ray;
use crate::vec3::Vector3;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord<'a> {
    t: f32,
    p: Vector3,
    normal: Vector3,
    mat: &'a Material,
}

pub trait Hitable {
    // TODO Returning a bool is not particularly ideiomatic?
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> bool;
}
