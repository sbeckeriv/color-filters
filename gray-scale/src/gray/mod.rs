use image;
use image::GenericImage;
use std::fs::File;
use std::path::Path;

pub trait Gray {
    fn gray(&self, red: u8, blue: u8, green: u8) -> (u8,u8,u8);
}

pub fn make_gray<T: Gray>(grayer: T, file_name: &str, image: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Result<(), image::ImageError>{
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
pub use self::luma::Luma;
pub use self::standard::Standard;
pub use self::desaturation::Desaturation;
pub use self::decomposition::Decomposition;
pub use self::channel::Channel;
pub use self::layers::Layers;
mod layers;
mod channel;
mod decomposition;
mod desaturation;
mod standard;
mod luma;
