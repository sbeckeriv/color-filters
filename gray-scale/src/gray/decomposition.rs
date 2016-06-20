use gray::Gray;
use std::cmp;
enum Selector{
    Min,
    Max,
}

pub struct Decomposition{
    selector: Selector
}

impl Decomposition{
    pub fn max() -> Self{
        Decomposition{ selector: Selector::Max }
    }

    pub fn min() -> Self{
        Decomposition{ selector: Selector::Min }
    }
}

impl Gray for Decomposition{
    fn gray(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let base = match self.selector{
            Selector::Max => {
                cmp::max(cmp::max(red, green), blue)
            }
            Selector::Min => {
                cmp::min(cmp::min(red, green), blue)
            }
        };
        let color = base as f32 / 3.0;
        (color as u8, color as u8, color as u8)
    }
}
