use std::io::{stdout, Write};

use std::path::Path;

extern crate image;

pub fn write_image(output_path: Option<&str>, image: &Vec<u8>, imgx: u32, imgy: u32) {
    match output_path {
        Some(x) => {
            let output_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
                create_image(image, imgx, imgy);
            output_image.save(Path::new(x)).unwrap();
        }

        None => {
            write_image_stdout(image, imgx, imgy);
        }
    }
}

pub fn create_image(
    image: &Vec<u8>,
    imgx: u32,
    imgy: u32,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    image::ImageBuffer::from_vec(imgx, imgy, image.to_vec()).unwrap()
}

// Write image to stdout using the ppm 3 format, with subpixel values 0-255
fn write_image_stdout(image: &Vec<u8>, imgx: u32, imgy: u32) {
    let mut std_writer = Box::new(stdout());

    match write!(std_writer, "P3\n{} {}\n{}\n\n", imgx, imgy, 255) {
        Err(e) => panic!("Failed write: {}", e),
        Ok(_) => (),
    }

    // TODO Make this less akward
    for chunk in image.chunks(3) {
        for c in chunk {
            match write!(std_writer, "{} ", c) {
                Err(e) => panic!("Failed write: {}", e),
                Ok(_) => (),
            }
        }
        match write!(std_writer, "\n",) {
            Err(e) => panic!("Failed write: {}", e),
            Ok(_) => (),
        }
    }
}
