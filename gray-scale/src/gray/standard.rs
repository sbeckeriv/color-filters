use gray::Gray;
pub struct Standard{
    gamma: i32
}

impl Standard{
    pub fn new(gamma: i32) -> Self{
        Standard{ gamma: gamma}
    }
}

impl Gray for Standard{
    fn gray(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let color = (red as u32 + green as u32 + blue as u32) as f32 /3.0;
        (color as u8, color as u8, color as u8)
    }
}

