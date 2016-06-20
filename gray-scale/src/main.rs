extern crate image;
use image::GenericImage;
use std::fs::File;
use std::env;
use std::path::Path;
mod gray;
use gray::{Luma, Standard, Gray};
fn main() {
    let file = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&file)).unwrap().to_rgb();
    let standard = Standard::new(0);
    let luma = Luma::new();
    make_gray(standard, &format!("{}_avg", file), &img);
    make_gray(luma, &format!("{}_luma", file), &img);
    println!("done");
}

fn make_gray<T: Gray>(grayer: T, file_name: &str, image: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) -> Result<(), image::ImageError>{
    let (image_x, image_y) = image.dimensions();
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let org_pixel = image.get_pixel(x, y);
        let (r,g,b) = grayer.gray(org_pixel[0],
                                    org_pixel[1],
                                    org_pixel[2]);
        let color = r as u32 +g as u32 +b as u32;
        *pixel = image::Rgb([color as u8,
                            color as u8,
                            color as u8
        ]);
    }

    let ppm_file = format!("{}.ppm",
                           file_name);

    let ref mut fout = File::create(&Path::new(&ppm_file)).unwrap();
    image::ImageRgb8(imgbuf.clone()).save(fout, image::PPM)
}
