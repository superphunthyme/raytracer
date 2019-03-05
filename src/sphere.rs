use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vector3;

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc: Vector3 = r.origin() - self.center;
    let a = r.direction().dot(&r.direction());
    let b = oc.dot(&r.direction());
    let c = oc.dot(&oc) - self.radius * self.radius;
    let discriminant = b*b - a*c;
    if discriminant > 0.0 {
        let mut temp = (-b - discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
            let t = temp;
            let p = r.point_at_parameter(t);
            return Some(HitRecord {
                t,
                p,
                normal: (p - self.center) / self.radius,
                material: self.material.clone(),
            });
        }
        temp = (-b + discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
            let t = temp;
            let p = r.point_at_parameter(t);
            return Some(HitRecord {
                t,
                p,
                normal: (p - self.center) / self.radius,
                material: self.material.clone(),
            });
        }
    }
    return None;
    }
}