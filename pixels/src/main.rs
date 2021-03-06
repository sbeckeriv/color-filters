extern crate image;
extern crate simple_parallel;
extern crate rusoto;
extern crate rustc_serialize;
extern crate curl;
extern crate time;
#[macro_use]
use rusoto::{AwsError, ProvideAwsCredentials, EnvironmentProvider, Region, DefaultCredentialsProvider};
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
use curl::easy::Easy;
fn timestamp () -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0 );
    mills
}
fn download(url: &str) -> String{
    let p =format!("/tmp/{}.jpg", timestamp());
    {
        let path = Path::new(&p);
        let display = path.display();
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = File::create(&path).unwrap();

        let mut easy = Easy::new();
        easy.url(&url).unwrap();
        easy.write_function(move |data| {
            Ok(file.write(data).unwrap())
        }).unwrap();
        easy.perform().unwrap();
    }
    p
}
fn main() {
    let file_url = env::args().nth(1).unwrap();
    let file = download(&file_url);
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
        //println!("{}", clean_name);

        let out_file = format!("{}.jpg",
                               file_name);

        let ref mut fout = File::create(&Path::new(&out_file)).unwrap();
        image::ImageRgb8(imgbuf.clone()).save(fout, image::JPEG);
        let path = Path::new(&out_file);
        let file = File::open(&path).unwrap();
        let mut reader = BufReader::new(file);
        let bytes: Vec<u8> = reader.bytes().map(|b| b.unwrap()).collect();
        let credentials = DefaultCredentialsProvider::new().unwrap();
        let mut s3 = s3::S3Client::new(credentials, Region::UsEast1);
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
        println!("https://s3.amazonaws.com/becker-rust-lambda/{}", clean_name);
    });
    std::fs::remove_file(&file);
}

fn filters(file: &str) ->Vec<Processor>{
    let mut processors = Vec::new();
    let mut v = Vec::new();
    v.push(vec![1.0,0.2,0.0]);
    v.push(vec![0.2,1.2,0.2]);
    v.push(vec![0.0,0.2,1.0]);
    processors.push(Processor::new(Box::new(FilterGrid::filter(v, 1.0/4.0)),
    format!("{}-blur-mot", file)));

    let mut v = Vec::new();
    v.push(vec![0.0,0.2,0.0]);
    v.push(vec![0.2,0.2,0.2]);
    v.push(vec![0.0,0.2,0.0]);
    processors.push(Processor::new(Box::new(FilterGrid::filter(v,1.0)),
    format!("{}-blur", file)));

    let mut v = Vec::new();
    v.push(vec![0.0-1.0,0.0-1.0,0.0-1.0]);
    v.push(vec![0.0-1.0,8.0,0.0-1.0]);
    v.push(vec![0.0-1.0,0.0-1.0,0.0-1.0]);
    processors.push(Processor::new(Box::new(FilterGrid::filter(v,1.0)),
    format!("{}-edges", file)));

    let mut v = Vec::new();
    v.push(vec![0.0-1.0,0.0-1.0,0.0-1.0]);
    v.push(vec![0.0-1.0,   15.0,0.0-2.0]);
    v.push(vec![0.0-1.0,0.0-2.0,0.0-2.0]);
    processors.push(Processor::new(Box::new(FilterGrid::filter(v,1.0/4.0)),
    format!("{}-sharpen", file)));

    let mut v = Vec::new();
    v.push(vec![1.0,1.0    ,1.0]);
    v.push(vec![1.0,0.0-7.0,1.0]);
    v.push(vec![1.0,1.0    ,1.0]);
    processors.push(Processor::new(Box::new(FilterGrid::filter(v,1.0)),
    format!("{}-edges-excessively", file)));

    let mut v = Vec::new();
    v.push(vec![0.0-2.0,0.0-1.0,0.0-0.0]);
    v.push(vec![0.0-1.0,0.0+1.0,0.0+1.0]);
    v.push(vec![0.0+0.0,0.0+1.0,0.0+2.0]);
    let mut f = FilterGrid::filter(v,1.0);
    f.bias = 0.0;
    processors.push(Processor::new(Box::new(f),
    format!("{}-boss", file)));

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
