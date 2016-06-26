use bright::Bright;

pub struct ChannelZero{
    red: Option<u8>,
    blue: Option<u8>,
    green: Option<u8>
}

impl ChannelZero{
    pub fn new(gamma: u8) -> Self{
        ChannelZero{ red: Some(gamma), blue: Some(gamma), green: Some(gamma)}
    }
    pub fn red(gamma: u8) -> Self{
        ChannelZero{ red: Some(gamma), blue: None, green: None}
    }
    pub fn blue(gamma: u8) -> Self{
        ChannelZero{ red: None, blue: Some(gamma), green: None}
    }
    pub fn green(gamma: u8) -> Self{
        ChannelZero{ red: None, blue: None, green: Some(gamma)}
    }
}

impl Bright for ChannelZero{
    fn lighten(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let r = match self.red {
            Some(_) => {
                red as u16 + self.red.unwrap() as u16
            }
            _ => { 0}
        };
        let b = match self.blue {
            Some(_) => {
                blue as u16 + self.blue.unwrap() as u16
            }
            _ => { 0}
        };
        let g = match self.green {
            Some(_) => {
                green as u16 + self.green.unwrap() as u16
            }
            _ => { 0}
        };
        (r as u8, b as u8, g as u8)
    }
}

