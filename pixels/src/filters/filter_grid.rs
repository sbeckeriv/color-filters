use filters::Filter;
use image;
use std::cmp;

pub struct FilterGrid{
    pub filter: Vec<Vec<f32>>,
    pub factor: f32,
    pub bias: f32,
}

impl FilterGrid{
    pub fn new() -> Self{
        let mut v = Vec::new();
        v.push(vec![0.0,0.0,0.0]);
        v.push(vec![0.0,1.0,0.0]);
        v.push(vec![0.0,0.0,0.0]);
        FilterGrid{ filter: v, bias: 0.0, factor: 1.0 }
    }

    pub fn filter(v: Vec<Vec<f32>>, factor: f32) -> Self{
        FilterGrid{ filter: v, bias: 0.0, factor: factor}
    }
}

impl Filter for FilterGrid{
    fn filter(&self, image: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, x: u32, y: u32) -> (u8,u8,u8){
        let filter_width = self.filter[0].len();
        let filter_height = self.filter.len();
        let (w, h) = image.dimensions();
        let mut r = 0.0;
        let mut b = 0.0;
        let mut g = 0.0;
        let y_offset = (filter_height as f32 / 2.0) as u32;
        let x_offset = (filter_width as f32 / 2.0) as u32;
        for filterY in (0..(filter_height) as u32){
            for filterX in (0..(filter_width) as u32){
                let imageX = (w+x - x_offset + filterX ) % w;
                let imageY = (h+y - y_offset + filterY ) % h;
                let org_pixel = image.get_pixel(imageX, imageY);
                let filter_y: &Vec<f32> = self.filter.get(filterY as usize).unwrap();
                let filter_spot: &f32 = filter_y.get(filterX as usize).unwrap();
                r += org_pixel[0] as f32 * *filter_spot as f32;
                b += org_pixel[1] as f32 * *filter_spot as f32;
                g += org_pixel[2] as f32 * *filter_spot as f32;

            }
        }
        r = (self.factor * r + self.bias);
        b = (self.factor * b + self.bias);
        g = (self.factor * g + self.bias);
        if r< 0.0{
            r=0.0;
        }else if r>255.0 {
            r=255.0;
        }
        if b< 0.0{
            b=0.0;
        }else if b>255.0 {
            b=255.0;
        }
        if g< 0.0{
            g=0.0;
        }else if g>255.0 {
            g=255.0;
        }
        (r as u8, b as u8, g as u8)
    }
}

