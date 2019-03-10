use crate::hitable::HitRecord;
use crate::random;
use crate::ray::Ray;
use crate::vec3::Vector3;

fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - (v.dot(n) * 2.0) * *n
}

fn refract(v: &Vector3, n: &Vector3, ni_over_nt: f32) -> Option<Vector3> {
    let uv = v.get_unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    }
    else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Clone)]
pub enum Material {
    Dielectric {
        ri: f32,
    },
    Lambertian {
        albedo: Vector3,
    },
    Metal {
        albedo: Vector3,
        fuzz: f32
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
            Material::Dielectric {ri} => {
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
            Material::Metal {albedo, fuzz} => {
                let reflected = reflect(&r_in.direction(), &rec.normal);
                let scattered = Ray::new(rec.p, reflected + random::random_in_unit_sphere() * f32::min(*fuzz, 1.0));
                let should_scatter = scattered.direction().dot(&rec.normal) > 0.0;
                ScatterRecord {
                    color: *albedo,
                    ray: scattered,
                    should_scatter,
                }
            }
        }
    }
}
