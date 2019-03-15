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
use std::f32;

fn color<T: Hitable>(r: &Ray, s: &T, depth: u32) -> Vector3 {
    match s.hit(r, 0.001, f32::MAX) {
        Some(hr) => {
            if depth < 50 {
                let scatter = hr.material.scatter(r, &hr);
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

fn random_scene() -> HitableList {
    use random::random_in_unit_interval as RandUnit;
    let mut hitable_list = HitableList::new();
    
    hitable_list.add(
        Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian{ albedo: Vector3::new(0.5, 0.5, 0.5) })
        );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = RandUnit();
            let x_rand = RandUnit();
            let z_rand = RandUnit();
            let center = Vector3::new(a as f32 + 0.9 * x_rand, 0.2, b as f32 + 0.9 * z_rand);
            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitable_list.add(
                        Sphere::new(
                            center, 0.2, Material::Lambertian{
                                albedo: Vector3::new(RandUnit() * RandUnit(),
                                RandUnit() * RandUnit(),
                                RandUnit() * RandUnit())
                            }
                            )
                        );
                }
                else if choose_mat < 0.95 {
                    hitable_list.add(
                        Sphere::new(
                            center, 0.2, Material::Metal {
                                albedo: Vector3::new(0.5 * (1.0 + RandUnit()),
                                0.5 * (1.0 + RandUnit()),
                                0.5 * (1.0 + RandUnit())),
                                fuzz: 0.5 * RandUnit(),
                            }
                            )
                        );
                }
                else {
                    hitable_list.add(
                        Sphere::new(center, 0.2, Material::Dielectric { ri: 1.5 })
                        );
                }
            }
        }
    }
    hitable_list.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric{ri: 1.5}));
    hitable_list.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Material::Lambertian{albedo: Vector3::new(0.4, 0.2, 0.1)}));
    hitable_list.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Material::Metal{albedo: Vector3::new(0.7, 0.6, 0.5), fuzz: 0.0}));
    hitable_list
}

fn main() {

    let x_res = 200;
    let y_res = 100;
    let num_samples = 100;
    let colour_range = 255;
    
    // Rewrite as create_ppm
    println!("P3\n{} {}\n{}\n", x_res, y_res, colour_range);

    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, x_res as f32 / y_res as f32, aperture, dist_to_focus);

    let hitable_list = random_scene();

    for j in (0..y_res).rev() {
        for i in 0..x_res {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..num_samples {
                let u_rand = random::random_in_unit_interval();
                let v_rand = random::random_in_unit_interval();
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
