mod camera;
mod hitable;
mod material;
mod random;
mod ray;
mod sphere;
mod vec3;
use crate::vec3::Vector3;
use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::hitable::HitableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use rand::Rng;
use std::f32;

fn color<T: Hitable>(r: &Ray, s: &T, depth: u32) -> Vector3 {
    let hr =  s.hit(r, 0.0, f32::MAX);
    match hr {
        Some(_hr) => {
            if depth < 50 {
                let scatter = _hr.material.scatter(r, &_hr);
                if scatter.should_scatter {
                    scatter.color * color(&scatter.ray, s, depth + 1)
                }
                else {
                    Vector3::new(0.0, 0.0, 0.0)
                }
            }
            else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        }
        // Color background
        None => {
            let unit_direction = r.direction().get_unit_vector();
            // 0 < t < 1
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vector3::new(1.0, 1.0, 1.0) * (1.0 - t)  + Vector3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {

    let x_res = 200;
    let y_res = 100;
    let num_samples = 100;
    let colour_range = 255;
    
    // Rewrite as create_ppm
    println!("P3\n{} {}\n{}\n", x_res, y_res, colour_range);

    let lookfrom = Vector3::new(0.0, 0.0, 0.0);
    let lookat = Vector3::new(0.0, 0.0, -1.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.0;
    let cam = Camera::new(lookfrom, lookat, vup, 90.0, x_res as f32 / y_res as f32, aperture, dist_to_focus);

    let sphere1 = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Material::Metal{ albedo: Vector3::new(1.0, 0.0, 0.0), fuzz: 0.0});
    let sphere2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian{ albedo: Vector3::new(0.8, 0.8, 0.0)});
    let sphere3 = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Material::Lambertian{ albedo: Vector3::new(0.0, 0.0, 1.0)});
    let mut hitable_list = HitableList::new();
    hitable_list.add(sphere1);
    hitable_list.add(sphere2);
    hitable_list.add(sphere3);

    for j in (0..y_res).rev() {
        for i in 0..x_res {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..num_samples {
                let u_rand = rand::thread_rng().gen_range(0.0, 1.0);
                let v_rand = rand::thread_rng().gen_range(0.0, 1.0);
                let u = (i as f32 + u_rand) / x_res as f32;
                let v = (j as f32 + v_rand) / y_res as f32;
                let ray = cam.get_ray(u, v);
                col = col + color(&ray, &hitable_list, 0);
            }
            col = col / num_samples as f32;
            col = Vector3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let out_colour = Vector3::new((255.99 * col.r()).floor(), (255.99 * col.g()).floor(), (255.99 * col.b()).floor());
            println!("{}", out_colour);
        }
    }
}
