use gray::Gray;
use std::cmp;
enum Color{
    Red,
    Blue,
    Green,
}

pub struct Channel{
    selector: Color
}

impl Channel{
    pub fn red() -> Self{
        Channel{ selector: Color::Red }
    }

    pub fn blue() -> Self{
        Channel{ selector: Color::Blue}
    }

    pub fn green() -> Self{
        Channel{ selector: Color::Green }
    }

}

impl Gray for Channel{
    fn gray(&self, red: u8, blue: u8, green: u8)-> (u8, u8, u8){
        let color = match self.selector{
            Color::Red => {
                red
            }
            Color::Blue => {
                blue
            }
            Color::Green => {
                green
            }
        };
        (color as u8, color as u8, color as u8)
    }
}
