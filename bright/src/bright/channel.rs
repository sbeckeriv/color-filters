use bright::Bright;

pub struct Channel{
    red: u8,
    blue: u8,
    green: u8
}

impl Channel{
    pub fn new(gamma: u8) -> Self{
        Channel{ red: gamma, blue: gamma, green: gamma}
    }
    pub fn red(gamma: u8) -> Self{
        Channel{ red: gamma, blue: 0, green: 0}
    }
    pub fn blue(gamma: u8) -> Self{
        Channel{ red: 0, blue: gamma, green: 0}
    }
    pub fn green(gamma: u8) -> Self{
        Channel{ red: 0, blue: 0, green: gamma}
    }
}

impl Bright for Channel{
    fn lighten(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let r = red as u16 + self.red as u16;
        let b = blue as u16 + self.blue as u16;
        let g = green as u16 +  self.green as u16;
        (r as u8, b as u8, g as u8)
    }
}

