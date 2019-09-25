use std::f32;

use std::sync::{Arc, Mutex};
use std::thread;

extern crate clap;
use clap::{App, Arg};

use crate::aabb::AABB;
use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::hitable::HitableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vector3;

mod aabb;
mod bvh;
mod camera;
mod hitable;
mod image_out;
mod material;
mod random;
mod ray;
mod sphere;
mod triangle;
mod vec3;

fn color<T: Hitable>(r: &Ray, s: &T, depth: u32) -> Vector3 {
    match s.hit(r, 0.001, f32::MAX) {
        Some(hr) => {
            if depth < 50 {
                let scatter = hr.material.scatter(r, &hr);
                if scatter.should_scatter {
                    scatter.color * color(&scatter.ray, s, depth + 1)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                }
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        }
        // Color background
        None => {
            let unit_direction = r.direction().get_unit_vector();
            // 0 < t < 1
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn random_scene() -> HitableList {
    use random::random_in_unit_interval as RandUnit;
    let mut hitable_list = HitableList::new();

    hitable_list.add(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Vector3::new(0.5, 0.5, 0.5),
        },
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = RandUnit();
            let x_rand = RandUnit();
            let z_rand = RandUnit();
            let center = Vector3::new(a as f32 + 0.9 * x_rand, 0.2, b as f32 + 0.9 * z_rand);
            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitable_list.add(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian {
                            albedo: Vector3::new(
                                RandUnit() * RandUnit(),
                                RandUnit() * RandUnit(),
                                RandUnit() * RandUnit(),
                            ),
                        },
                    ));
                } else if choose_mat < 0.95 {
                    hitable_list.add(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            albedo: Vector3::new(
                                0.5 * (1.0 + RandUnit()),
                                0.5 * (1.0 + RandUnit()),
                                0.5 * (1.0 + RandUnit()),
                            ),
                            fuzz: 0.5 * RandUnit(),
                        },
                    ));
                } else {
                    hitable_list.add(Sphere::new(center, 0.2, Material::Dielectric { ri: 1.5 }));
                }
            }
        }
    }
    hitable_list.add(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { ri: 1.5 },
    ));
    hitable_list.add(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vector3::new(0.4, 0.2, 0.1),
        },
    ));
    hitable_list.add(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vector3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    ));
    hitable_list
}

fn main() {
    let matches = App::new("Raytracer")
        .about("Raytracer in Rust from Peter Shirley's Raytracing in One Weekend")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .help("Output file. If not specified, wrties to stdout."),
        )
        .arg(
            Arg::with_name("samples")
                .short("s")
                .long("samples")
                .takes_value(true)
                .help("Number of samples per pixel")
                .default_value("100"),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .takes_value(true)
                .help("Number of threads to run")
                .default_value("1"),
        )
        .arg(
            Arg::with_name("x_res")
                .short("x")
                .long("x_res")
                .takes_value(true)
                .help("Width of trace in pixels")
                .default_value("200"),
        )
        .arg(
            Arg::with_name("y_res")
                .short("y")
                .long("y_res")
                .takes_value(true)
                .help("Height of trace in pixels")
                .default_value("100"),
        )
        .get_matches();

    let x_res: u32 = matches.value_of("x_res").unwrap().parse().unwrap();
    let y_res: u32 = matches.value_of("y_res").unwrap().parse().unwrap();
    let num_samples: u32 = matches.value_of("samples").unwrap().parse().unwrap();
    let num_threads: u32 = matches.value_of("threads").unwrap().parse().unwrap();
    let output = matches.value_of("output");

    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        x_res as f32 / y_res as f32,
        aperture,
        dist_to_focus,
    );

    let hitable_list = random_scene();
    let bvh = BVHNode::new(hitable_list.hitables.clone());

    let mut thread_handles = Vec::new();
    let result = Arc::new(Mutex::new(Vec::new()));
    let samples_per_thread = num_samples / num_threads;

    for _ in 0..num_threads {
        let mut thread_output = Vec::new();
        let bvh_clone = bvh.clone();
        let result = Arc::clone(&result);
        // TODO Handle exceptions + num_samples not divisible by num_threads
        thread_handles.push(thread::spawn(move || {
            for j in (0..y_res).rev() {
                for i in 0..x_res {
                    let mut col = Vector3::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_thread {
                        let u_rand = random::random_in_unit_interval();
                        let v_rand = random::random_in_unit_interval();
                        let u = (i as f32 + u_rand) / x_res as f32;
                        let v = (j as f32 + v_rand) / y_res as f32;
                        let ray = cam.get_ray(u, v);
                        col = col + color(&ray, &bvh_clone, 0);
                    }
                    col = col / num_samples as f32;
                    thread_output.push(col);
                    // Do averaging in th thread
                    // Then the final average at the end
                }
            }
            let mut l_result = result.lock().unwrap();
            if l_result.len() == 0 {
                *l_result = thread_output;
            } else {
                *l_result = l_result
                    .iter()
                    .zip(thread_output.iter())
                    .map(|(a, b)| a + b)
                    .collect();
            }
        }));
    }

    for thread in thread_handles {
        thread.join().unwrap();
    }

    let mut output_buffer: Vec<u8> = Vec::with_capacity(x_res as usize * y_res as usize * 3);

    // Should buffer and only write to the file at the end
    for col in result.lock().unwrap().iter() {
        let out_colour = Vector3::new(
            (255.99 * col.r().sqrt()).floor(),
            (255.99 * col.g().sqrt()).floor(),
            (255.99 * col.b().sqrt()).floor(),
        );

        output_buffer.push(out_colour.r() as u8);
        output_buffer.push(out_colour.g() as u8);
        output_buffer.push(out_colour.b() as u8);
    }

    image_out::write_image(output, &output_buffer, x_res, y_res);
}
