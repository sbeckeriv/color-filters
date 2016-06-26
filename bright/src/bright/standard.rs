use bright::Bright;
pub struct Standard{
    gamma: u8
}

impl Standard{
    pub fn new(gamma: u8) -> Self{
        Standard{ gamma: gamma}
    }
}

impl Bright for Standard{
    fn lighten(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let r = red as u16 + self.gamma as u16;
        let b = blue as u16 + self.gamma as u16;
        let g = green as u16 +  self.gamma as u16;
        (r as u8, b as u8, g as u8)
    }
}

