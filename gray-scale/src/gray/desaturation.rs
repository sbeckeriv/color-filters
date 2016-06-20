use gray::Gray;
use std::cmp;
pub struct Desaturation;

impl Desaturation{
    pub fn new() -> Self{
        Desaturation{}
    }
}

impl Gray for Desaturation{
    fn gray(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let max = cmp::max(cmp::max(red, green), blue);
        let min = cmp::min(cmp::min(red, green), blue);
        let color =  (min as u16 + max as u16)  as f32 /2.0;
        (color as u8, color as u8, color as u8)
    }
}
