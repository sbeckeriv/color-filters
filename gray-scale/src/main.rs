extern crate image;
extern crate simple_parallel;
use std::env;
use std::sync::Arc;
use std::path::Path;
mod gray;

use image::GenericImage;
use std::fs::File;
use gray::{Decomposition, Desaturation, Channel, Layers,
Luma, Standard, Gray};
fn main() {
    let file = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&file)).unwrap().to_rgb();
    let mut processors = Vec::new();
    processors.push(Processor::new(Box::new(Standard::new(0)),
    format!("{}_avg", file)));
       processors.push(Processor::new( Box::new(Desaturation::new()),
       format!("{}_desat", file)));

       processors.push(Processor::new( Box::new(Decomposition::max()),
       format!("{}_decomp_max", file)));

       processors.push(Processor::new( Box::new(Decomposition::min()),
       format!("{}_decomp_min", file)));

       processors.push(Processor::new(Box::new(Luma::new()),
       format!("{}_luma", file)));

       processors.push(Processor::new(Box::new(Channel::red()),
       format!("{}_red", file)));
       processors.push(Processor::new(Box::new(Channel::green()),
       format!("{}_green", file)));
       processors.push(Processor::new(Box::new(Channel::blue()),
       format!("{}_blue", file)));
       processors.push(Processor::new(Box::new(Layers::new(16)),
       format!("{}_layer_16", file)));

       processors.push(Processor::new(Box::new(Layers::new(160)),
       format!("{}_layer_160", file)));
    let mut pool = simple_parallel::Pool::new(5);
    pool.for_(processors.iter(), |processor| {
        let file_name =  &processor.file_base;
        let image= &img;
        let (image_x, image_y) = image.dimensions();
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let org_pixel = image.get_pixel(x, y);
            let (r,g,b) = processor.process.gray(org_pixel[0],
                                      org_pixel[1],
                                      org_pixel[2]);
            let color = r as u32 +g as u32 +b as u32;
            *pixel = image::Rgb([color as u8,
                                color as u8,
                                color as u8
            ]);
        }

        let out_file = format!("{}.jpg",
                               file_name);

        let ref mut fout = File::create(&Path::new(&out_file)).unwrap();
        image::ImageRgb8(imgbuf.clone()).save(fout, image::JPEG);
    });
    println!("done");
}

struct Processor{
    pub process: Box<Gray>,
    pub file_base: String
}
impl Processor{
    fn new(process: Box<Gray>, name: String) -> Self{
        Processor{ process: process, file_base: name}
    }
}
