use gray::Gray;
pub struct Layers{
    layers: u8
}

impl Layers{
    pub fn new(layers: u8) -> Self{
        Layers{ layers: layers}
    }
}

impl Gray for Layers{
    fn gray(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let factor = 255.0 / (self.layers as f32);
        let avg = (red as u32 + green as u32 + blue as u32) as f32 /9.0;
        let color  = ((avg / factor) + 0.5) * factor;
        (color as u8, color as u8, color as u8)
    }
}

