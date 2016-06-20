extern crate image;
use std::env;
use std::path::Path;
mod gray;
use gray::{Decomposition, Desaturation, Channel, Layers,
    Luma, Standard, make_gray};
fn main() {
    let file = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&file)).unwrap().to_rgb();
    let standard = Standard::new(0);
    let desaturation = Desaturation::new();
    let decomposition = Decomposition::max();
    let decomposition_min = Decomposition::min();
    let luma = Luma::new();
    let channel = Channel::red();
    let channel_b = Channel::blue();
    let channel_g = Channel::green();
    let layers = Layers::new(10);
    let layers_100 = Layers::new(100);
    make_gray(standard, &format!("{}_avg", file), &img);
    make_gray(luma, &format!("{}_luma", file), &img);
    make_gray(desaturation, &format!("{}_desat", file), &img);
    make_gray(decomposition, &format!("{}_decomp_max", file), &img);
    make_gray(decomposition_min, &format!("{}_decomp_min", file), &img);
    make_gray(channel, &format!("{}_channel_r", file), &img);
    make_gray(channel_b, &format!("{}_channel_b", file), &img);
    make_gray(channel_g, &format!("{}_channel_g", file), &img);
    make_gray(layers, &format!("{}_layers_10", file), &img);
    make_gray(layers_100, &format!("{}_layers_100", file), &img);
    println!("done");
}

