pub trait Gray {
    fn gray(&self, red: u8, blue: u8, green: u8) -> (u8,u8,u8);
}

pub struct Luma{
    coefficients: (f32, f32,f32)
}

impl Luma{
    pub fn new() -> Self{
        Luma{ coefficients: ( 0.2126, 0.7152, 0.0722)    }
    }

    pub fn common() -> Self{
        Luma{ coefficients: ( 0.3, 0.59, 0.11)    }
    }

    pub fn modern() -> Self{
        Luma{ coefficients: ( 0.299, 0.587, 0.114)    }
    }
}

impl Gray for Luma{
    fn gray(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let r = self.coefficients.0 * red as f32;
        let g = self.coefficients.1 * blue as f32;
        let b = self.coefficients.2 * green as f32;
        (r as u8, g as u8, b as u8)
    }
}

pub struct Standard{
    gamma: i32
}

impl Standard{
    pub fn new(gamma: i32) -> Self{
        Standard{ gamma: gamma}
    }
}
use std::u8::MAX;
impl Gray for Standard{
    fn gray(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let color = (red as u32 + green as u32 + blue as u32) as f32 /3.0;
        (color as u8, color as u8, color as u8)
    }
}

