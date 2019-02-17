mod camera;
mod hitable;
mod vec3;
mod random;
mod ray;
mod material;
use crate::vec3::Vector3;

fn main() {

    let x_res = 200;
    let y_res = 100;
    let colour_range = 255;
    
    // Rewrite as create_ppm
    println!("P3\n{} {}\n{}\n", x_res, y_res, colour_range);

    for j in (0..y_res).rev() {
        for i in 0..x_res {
            let r = i as f32 / x_res as f32 * 255.99;
            let g = j as f32 / y_res as f32 * 255.99;
            let b: f32 = 0.2 * 255.99;

            let out_colour = Vector3::new(r.floor(), g.floor(), b.floor());
            println!("{}", out_colour);
        }
    }
}
