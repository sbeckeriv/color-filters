use bright::Bright;

pub struct DuoTone{
    red: Option<u8>,
    blue: Option<u8>,
    green: Option<u8>
}

impl DuoTone{
    pub fn new() -> Self{
        DuoTone{ red: None, blue: None, green: None}
    }
}

impl Bright for DuoTone{
    fn lighten(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        match (green as f32) / 255.0{
            0.0 ... 0.33 => { (red, 0, 0) }
            0.33 ... 0.66 => { (red, blue, 0) }
            _  => { (0, blue, green)}
        }
    }
}

