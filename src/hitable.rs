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

pub struct HitableList {
    hitables: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            hitables: Vec::new(),
        }
    }

    pub fn add<T: Hitable + 'static>(&mut self, item: T) {
        self.hitables.push(Box::new(item));
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in self.hitables.iter() {
            match hitable.hit(r, t_min, closest_so_far) {
                Some(hr) => {
                    closest_so_far = hr.t;
                    temp_rec = Some(hr);
                },
                None => ()
            }
        }
        temp_rec
    }
}
