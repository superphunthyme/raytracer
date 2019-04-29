use crate::hitable::HitRecord;
use crate::hitable::Hitable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vector3;
use std::f32;
use std::fmt;

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
        // TODO Lookup better solution for floaiting point comparisons
        if det > -0.000001 && det < 0.000001 {
            return None;
        }

        let tvec = r.origin() - self.v1;

        // (u,v) Barycentric coordinates
        let u = tvec.dot(&pvec);
        if det > 0.0 {
            if u < 0.0 || u > det {
                return None;
            }
        } else {
            if u >= 0.0 || u <= det {
                return None;
            }
        }

        let qvec = tvec.cross(&edge1);
        let v = r.direction().dot(&qvec);
        if det > 0.0 {
            if v < 0.0 || u + v > det {
                return None;
            }
        } else {
            if v >= 0.0 || u + v <= det {
                return None;
            }
        }

        // Otherwise we have a hit
        let inv_det = 1.0 / det;
        let mut t = edge2.dot(&qvec);
        t *= inv_det;

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
        } else {
            return None;
        }
    }
}

impl fmt::Display for Triangle {
    // TODO Add material
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v1: {}, v2: {}, v3: {}", self.v1, self.v2, self.v3)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // TODO properly define front/back
    #[test]
    fn intersect_front() {
        let r: Ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        // Triagnle sitting on z=2, containing (2,2,2)
        let t: Triangle = Triangle::new(
            Vector3::new(2.0, 3.0, 2.0),
            Vector3::new(3.0, 1.0, 2.0),
            Vector3::new(1.0, 1.0, 2.0),
            Material::Lambertian {
                albedo: Vector3::new(0.0, 0.0, 0.0),
            },
        );
        let expected_t = 2.0;

        match t.hit(&r, 0.0, f32::MAX) {
            Some(hr) => {
                // Intersection point should be at (2,2,2), corresponding to t=2
                // TODO Lookup better solution for floaiting point comparisons
                assert!(
                    hr.t > expected_t - 0.000001,
                    "Triangle {} should be hit by ray {} at t={}. t is {}. ",
                    t,
                    r,
                    expected_t,
                    hr.t
                );
                assert!(
                    hr.t < expected_t + 0.000001,
                    "Triangle {} should be hit by ray {} at t={}. t is {}. ",
                    t,
                    r,
                    expected_t,
                    hr.t
                );
            }
            None => panic!(
                "Triangle {} should be hit by ray {} at t={} ",
                t, r, expected_t
            ),
        }
    }

    // Same trianle as interset_front, but hit from the opposite direction
    #[test]
    fn intersect_back() {
        let r: Ray = Ray::new(Vector3::new(0.0, 0.0, 4.0), Vector3::new(1.0, 1.0, -1.0));
        // Triagnle sitting on z=2, containing (2,2,2)
        let t: Triangle = Triangle::new(
            Vector3::new(2.0, 3.0, 2.0),
            Vector3::new(3.0, 1.0, 2.0),
            Vector3::new(1.0, 1.0, 2.0),
            Material::Lambertian {
                albedo: Vector3::new(0.0, 0.0, 0.0),
            },
        );
        let expected_t = 2.0;

        match t.hit(&r, 0.0, f32::MAX) {
            Some(hr) => {
                // Intersection point should be at (2,2,2), corresponding to t=2
                // TODO Lookup better solution for floaiting point comparisons
                assert!(
                    hr.t > expected_t - 0.000001,
                    "Triangle {} should be hit by ray {} at t={}. t is {}. ",
                    t,
                    r,
                    expected_t,
                    hr.t
                );
                assert!(
                    hr.t < expected_t + 0.000001,
                    "Triangle {} should be hit by ray {} at t={}. t is {}. ",
                    t,
                    r,
                    expected_t,
                    hr.t
                );
            }
            None => panic!(
                "Triangle {} should be hit by ray {} at t={} ",
                t, r, expected_t
            ),
        }
    }

    #[test]
    fn not_instersect() {
        let r: Ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        // Triangle sitting on z=2, not containing (2,2,2)
        // TODO At the moment, this test doesn't really cover enough to be useful
        let t: Triangle = Triangle::new(
            Vector3::new(4.0, 3.0, 2.0),
            Vector3::new(5.0, 1.0, 2.0),
            Vector3::new(3.0, 1.0, 2.0),
            Material::Lambertian {
                albedo: Vector3::new(0.0, 0.0, 0.0),
            },
        );

        match t.hit(&r, 0.0, f32::MAX) {
            Some(hr) => panic!(
                "Triangle {} should not be hit by ray {} . Intersection point found at {}.",
                t,
                r,
                r.point_at_parameter(hr.t)
            ),
            None => (),
        }
    }
}
