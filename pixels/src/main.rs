extern crate image;
extern crate simple_parallel;
extern crate rusoto;
extern crate rustc_serialize;
#[macro_use]
use rusoto::{AwsError, EnvironmentProvider, Region};
use rusoto::s3;
use rustc_serialize::base64::{ToBase64, STANDARD};

mod filters;

use std::env;
use std::sync::Arc;
use std::path::Path;
use image::GenericImage;
use filters::{Filter, FilterGrid};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let file = env::args().nth(1).unwrap();
    let image = image::open(&Path::new(&file)).unwrap().to_rgb();
    let mut processors = filters(&file);
    let mut pool = simple_parallel::Pool::new(5);
    pool.for_(processors.iter(), |processor| {
        let file_name =  &processor.file_base;
        let (image_x, image_y) = image.dimensions();
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let (r,b,g) = processor.process.filter(&image, x, y);
            *pixel = image::Rgb([r, b, g]);
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

fn filters(file: &str) ->Vec<Processor>{
    let mut processors = Vec::new();
    processors.push(Processor::new(Box::new(FilterGrid::new()),
    format!("{}-org", file)));

    let mut v = Vec::new();
    v.push(vec![0.0,0.2,0.0]);
    v.push(vec![0.2,0.2,0.2]);
    v.push(vec![0.0,0.2,0.0]);
    processors.push(Processor::new(Box::new(FilterGrid::filter(v)),
    format!("{}-2", file)));
    let mut v = Vec::new();
    v.push(vec![0.0-1.0,0.0-1.0,0.0-1.0]);
    v.push(vec![0.0-1.0,8.0,0.0-1.0]);
    v.push(vec![0.0-1.0,0.0-1.0,0.0-1.0]);
    processors.push(Processor::new(Box::new(FilterGrid::filter(v)),
    format!("{}-edges", file)));
    processors
}

struct Processor{
    pub process: Box<Filter>,
    pub file_base: String
}
impl Processor{
    fn new(process: Box<Filter>, name: String) -> Self{
        Processor{ process: process, file_base: name}
    }
}
