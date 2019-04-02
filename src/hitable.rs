use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vector3;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

pub trait Hitable: HitableClone {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub trait HitableClone {
    fn clone_box(&self) -> Box<Hitable + Send>;
}

#[derive(Clone)]
pub struct HitableList {
    hitables: Vec<Box<dyn Hitable + Send>>,
}

impl<T> HitableClone for T
where
    T: 'static + Hitable + Clone + Send,
{
    fn clone_box(&self) -> Box<Hitable + Send> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hitable + Send> {
    fn clone(&self) -> Box<Hitable + Send> {
        self.clone_box()
    }
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            hitables: Vec::new(),
        }
    }

    pub fn add<T: Hitable + 'static + Send>(&mut self, item: T) {
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
                }
                None => (),
            }
        }
        temp_rec
    }
}
