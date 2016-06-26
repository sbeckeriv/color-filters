extern crate image;
extern crate simple_parallel;
extern crate rusoto;
extern crate rustc_serialize;
#[macro_use]
use rusoto::{AwsError, EnvironmentProvider, Region};
use rusoto::s3;
use rustc_serialize::base64::{ToBase64, STANDARD};

mod bright;

use std::env;
use std::sync::Arc;
use std::path::Path;
use image::GenericImage;
use bright::{Standard, Bright, Channel, ChannelZero,
DuoTone};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() {
    let file = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&file)).unwrap().to_rgb();
    let mut processors = Vec::new();
    processors.push(Processor::new(Box::new(Standard::new(0)),
    format!("{}-0", file)));
    processors.push(Processor::new(Box::new(Standard::new(100)),
    format!("{}-100", file)));
    processors.push(Processor::new(Box::new(Standard::new(0-100)),
    format!("{}-neg-100", file)));
    processors.push(Processor::new(Box::new(Channel::red(10)),
    format!("{}-red-10", file)));
    processors.push(Processor::new(Box::new(Channel::blue(10)),
    format!("{}-blue-10", file)));
    processors.push(Processor::new(Box::new(Channel::green(10)),
    format!("{}-green-10", file)));

    processors.push(Processor::new(Box::new(ChannelZero::red(0)),
    format!("{}-red-zero-10", file)));
    processors.push(Processor::new(Box::new(ChannelZero::blue(0)),
    format!("{}-blue-zero-10", file)));
    processors.push(Processor::new(Box::new(ChannelZero::green(0)),
    format!("{}-green-zero-10", file)));

    processors.push(Processor::new(Box::new(DuoTone::new()),
    format!("{}-duotone", file)));

    let mut pool = simple_parallel::Pool::new(5);
    pool.for_(processors.iter(), |processor| {
        let file_name =  &processor.file_base;
        let image= &img;
        let (image_x, image_y) = image.dimensions();
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let org_pixel = image.get_pixel(x, y);
            let (r,b,g) = processor.process.lighten(org_pixel[0],
                                                 org_pixel[1],
                                                 org_pixel[2]);
            *pixel = image::Rgb([r as u8,
                                b as u8,
                                g as u8
            ]);
        }
        let name = file_name.replace(".","-");
        let clean_name = format!("{}.jpg", name);

        let out_file = format!("{}.jpg",
                               file_name);

        let ref mut fout = File::create(&Path::new(&out_file)).unwrap();
        image::ImageRgb8(imgbuf.clone()).save(fout, image::JPEG);
        let path = Path::new(&out_file);
        let file = File::open(&path).unwrap();
        let mut reader = BufReader::new(file);
        let bytes: Vec<u8> = reader.bytes().map(|b| b.unwrap()).collect();

        let mut s3 = s3::S3Client::new(EnvironmentProvider::new(), Region::UsEast1);
 		let mut request = s3::PutObjectRequest::default();
        request.key = clean_name.to_string();
        request.bucket = "becker-rust-lambda".to_string();
        request.body = Some(&bytes);
        request.content_type = Some("image/jpeg".to_string());
        let mut met = s3::Metadata::new();
        met.insert("Content-Type".to_string(),"image/jpeg".to_string());
        met.insert("becker".to_string(),"image/jpeg".to_string());
        request.metadata = Some(met);
        request.acl = Some(s3::CannedAcl::PublicRead);
		s3.put_object(&request);
    });
}

struct Processor{
    pub process: Box<Bright>,
    pub file_base: String
}
impl Processor{
    fn new(process: Box<Bright>, name: String) -> Self{
        Processor{ process: process, file_base: name}
    }
}
