use crate::aabb::AABB;
use crate::hitable::HitRecord;
use crate::hitable::Hitable;
use crate::random;
use crate::ray::Ray;
use std::cmp::PartialOrd;

// TODO Look into typedefs in Rust, especially for the Box<dyn Hitable + Send>
// TODO Enum for axis
//fn box_compare(axis: u32) -> Box<FnMut(Box<dyn Hitable + Send>, Box<dyn Hitable + Send>) -> Ordering> {
//    Box::new()
//}

#[derive(Clone)]
pub struct BVHNode {
    left: Box<dyn Hitable + Send>,
    right: Box<dyn Hitable + Send>,
    pub bbox: AABB,
}

impl BVHNode {
    pub fn new(mut l: Vec<Box<dyn Hitable + Send>>) -> BVHNode {
        let axis = 3 * random::random_in_unit_interval() as u32;
        let n = l.len();
        // TODO Enum for axis
        // TODO Handle unexpected cases, cleanuo
        // The Hitables are soreted by the minimum coordinate in the given axis.
        match axis {
            0 => l.sort_by(|a, b| {
                a.bounding_box()
                    .unwrap()
                    .min()
                    .x()
                    .partial_cmp(&b.bounding_box().unwrap().min().x())
                    .unwrap()
            }),
            1 => l.sort_by(|a, b| {
                a.bounding_box()
                    .unwrap()
                    .min()
                    .y()
                    .partial_cmp(&b.bounding_box().unwrap().min().y())
                    .unwrap()
            }),
            2 => l.sort_by(|a, b| {
                a.bounding_box()
                    .unwrap()
                    .min()
                    .z()
                    .partial_cmp(&b.bounding_box().unwrap().min().z())
                    .unwrap()
            }),
            _ => panic!("Rng generated a bad value."),
        }

        let left;
        let right;
        match n {
            1 => {
                right = l[0].clone();
                left = l[0].clone();
            }
            2 => {
                left = l[0].clone();
                right = l[1].clone();
            }
            _ => {
                left = Box::new(BVHNode::new(l[0..(n / 2)].to_vec()));
                right = Box::new(BVHNode::new(l[(n / 2)..n].to_vec()));
            }
        }

        let box_left = left.bounding_box();
        let box_right = right.bounding_box();

        if !(box_left.is_some() && box_right.is_some()) {
            panic!("Failure in bounding bvh construction")
        }

        BVHNode {
            left,
            right,
            bbox: box_left.unwrap().surrounding_box(&box_right.unwrap()),
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);

            if hit_left.is_some() && hit_right.is_some() {
                let left_rec = hit_left.unwrap();
                let right_rec = hit_right.unwrap();
                if left_rec.t < right_rec.t {
                    Some(left_rec)
                } else {
                    Some(right_rec)
                }
            } else if hit_left.is_some() {
                hit_left
            } else if hit_right.is_some() {
                hit_right
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
