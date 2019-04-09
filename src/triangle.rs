use crate::hitable::HitRecord;
use crate::hitable::Hitable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vector3;

#[derive(Clone)]
pub struct Triangle {
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    material: Material,
    // There are two-normals, we only store one
}

// Using two-sided triangles
impl Triangle {
    pub fn new(v1: Vector3, v2: Vector3, v3: Vector3, material: Material) -> Triangle {
        Triangle {
            v1,
            v2,
            v3,
            material,
        }
    }
}


impl Hitable for Triangle {
    // http://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/code/raytri_tam.pdf
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        // TODO This are recalculated everytime, could probably store them with the triangle
        // Edges
        let edge1 = self.v2 - self.v1;
        let edge2 = self.v3 - self.v1;

        // Start of determinant calculation
        let pvec = r.direction().cross(&edge2);

        let det = edge1.dot(&pvec);

        // Parallel ray test
        if det > -0.000001 && det < 0.000001 {
            return None;
        }

        let tvec = (r.origin() - self.v1);

        // (u,v) Barycentric coordinates
        let mut u = tvec.dot(&pvec);
        if det > 0.0 {
            if u < 0.0 || u > det {
                return None;
            }
        }
        else {
            if u >= 0.0 || u <= det {
                return None
            }
        }

        let qvec = tvec.cross(&edge1);
        let mut v = r.direction().dot(&qvec);
        if det > 0.0 {
            if v < 0.0 || u + v > det {
                return None
            }
        }
        else {
            if v >= 0.0 || u + v <= det {
                return None
            }
        }

        // Otherwise we have a hit
        let inv_det = 1.0 / det;
        let mut t = edge2.dot(&qvec);
        t *= inv_det;
        u *= inv_det;
        v *= inv_det;

        // Normal
        // TODO Could also calculate this once and store with the triangle
        let n = edge1.cross(&edge2).get_unit_vector();

        let p = r.point_at_parameter(t);

        if t > 0.00001 && t < t_max && t > t_min {

            return Some(HitRecord {
                t,
                p,
                normal: n,
                material: self.material.clone(),
            });
        }
        else {
            return None;
        }
    }
}
