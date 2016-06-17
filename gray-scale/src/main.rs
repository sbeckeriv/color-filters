extern crate image;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::Path;
fn main() {
    let file = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&file)).unwrap().to_rgb();

    let (image_x, image_y) = img.dimensions();
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let org_pixel = img.get_pixel(x, y);
        let r = 0.2126 * org_pixel[0] as f32;
        let g = 0.7152 * org_pixel[1] as f32;
        let b = 0.0722 * org_pixel[2] as f32;
        let color = r+g+b;
        *pixel = image::Rgb([color as u8,
                            color as u8,
                            color as u8
        ]);
    }

    let ppm_file = format!("{}_gray.ppm",
                           file);

    let ref mut fout = File::create(&Path::new(&ppm_file)).unwrap();
    let _ = image::ImageRgb8(imgbuf.clone()).save(fout, image::PPM);
    println!("Hello, world! {}", ppm_file);
}
