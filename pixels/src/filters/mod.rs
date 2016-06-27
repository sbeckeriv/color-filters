use image;
use image::GenericImage;
use std::fs::File;
use std::path::Path;

pub trait Filter: Send + Sync{
    fn filter(&self, image: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, x: u32, y: u32) -> (u8,u8,u8);
}
pub use self::filter_grid::FilterGrid;
mod filter_grid;
