use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vector3;

      #[derive(Clone)]
pub enum Material {
    Metal {
    },
    Lambertian {
    },
    TexturedLambertian {
    },
    Dielectric {
    }
}
