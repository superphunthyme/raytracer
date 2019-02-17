use crate::vec3::Vector3;
use rand::Rng;

pub fn random_in_unit_disk() -> Vector3 {
    let mut p;
    loop {
        let x = rand::thread_rng().gen_range(0.0, 1.0);
        let y = rand::thread_rng().gen_range(0.0, 1.0);
        p = Vector3::new(x, y, 1.0) * 2.0 - Vector3::new(1.0, 1.0, 0.0);
        if p.dot(&p) >= 1.0 {
            break;
        }
    }
    p
}

pub fn random_in_unit_sphere() -> Vector3 {
    let mut p;
    loop {
        let x = rand::thread_rng().gen_range(0.0, 1.0);
        let y = rand::thread_rng().gen_range(0.0, 1.0);
        let z = rand::thread_rng().gen_range(0.0, 1.0);
        p = Vector3::new(x, y, z) * 2.0 - Vector3::new(1.0, 1.0, 1.0);
        if p.dot(&p) >= 1.0 {
            break;
        }
    }
    p
}
