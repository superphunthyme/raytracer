use crate::hitable::HitRecord;
use crate::random;
use crate::ray::Ray;
use crate::vec3::Vector3;

#[derive(Clone)]
pub enum Material {
    Dielectric {
    },
    Lambertian {
        albedo: Vector3,
    },
    Metal {
    },
}

pub struct ScatterRecord {
    pub color: Vector3,
    pub ray: Ray,
    pub should_scatter: bool,
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterRecord {
        match self {
            // FIXME This is wrong, just a placeholder for now
            Material::Dielectric {} => {
                let target = rec.p + rec.normal + random::random_in_unit_sphere();
                ScatterRecord {
                    color: Vector3::new(0.0, 0.0, 0.0),
                    ray: Ray::new(rec.p, target - rec.p),
                    should_scatter: true,
                }
            }
            Material::Lambertian { albedo } => {
                let target = rec.p + rec.normal + random::random_in_unit_sphere();
                ScatterRecord {
                    color: *albedo,
                    ray: Ray::new(rec.p, target - rec.p),
                    should_scatter: true,
                }
            }
            // FIXME This is wrong, just a placeholder for now
            Material::Metal {} => {
                let target = rec.p + rec.normal + random::random_in_unit_sphere();
                ScatterRecord {
                    color: Vector3::new(0.0, 0.0, 0.0),
                    ray: Ray::new(rec.p, target - rec.p),
                    should_scatter: true,
                }
            }
        }
    }
}
