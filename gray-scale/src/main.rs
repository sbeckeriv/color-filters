extern crate image;
use std::env;
use std::path::Path;
mod gray;
use gray::{Desaturation, Luma, Standard,  make_gray};
fn main() {
    let file = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&file)).unwrap().to_rgb();
    let standard = Standard::new(0);
    let desaturation = Desaturation::new();
    let luma = Luma::new();
    make_gray(standard, &format!("{}_avg", file), &img);
    make_gray(luma, &format!("{}_luma", file), &img);
    make_gray(desaturation, &format!("{}_desat", file), &img);
    println!("done");
}

