use crate::color::Color;

pub struct JewelryBox {
    black: u8,
    white: u8,
    red: u8,
    blue: u8,
    green: u8,
}

impl JewelryBox {
    pub fn new() -> JewelryBox {
        JewelryBox {
            black: 0,
            white: 0,
            red: 0,
            blue: 0,
            green: 0,
        }
    }
    pub fn get_jewelry(&self, color: Color) -> u8 {
        use crate::color::Color::*;
        match color {
            Black => self.black,
            White => self.white,
            Red => self.red,
            Blue => self.blue,
            Green => self.green,
            _ => unreachable!(),
        }
    }
    pub fn set_jewelry(&mut self, color: Color, jewelies: u8) {
        use crate::color::Color::*;
        match color {
            Black => self.black = jewelies,
            White => self.white = jewelies,
            Red => self.red = jewelies,
            Blue => self.blue = jewelies,
            Green => self.green = jewelies,
            _ => unreachable!(),
        }
    }
    pub fn add_jewelry(&mut self, color: Color, jewelies: u8) {
        use crate::color::Color::*;
        match color {
            Black => self.black += jewelies,
            White => self.white += jewelies,
            Red => self.red += jewelies,
            Blue => self.blue += jewelies,
            Green => self.green += jewelies,
            _ => unreachable!(),
        }
    }
}
