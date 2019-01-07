use crate::color::Color;

pub struct JewelryBox {
    black: u8,
    white: u8,
    red: u8,
    blue: u8,
    green: u8,
}

impl JewelryBox {
    pub fn create() -> JewelryBox {
        JewelryBox {
            black: 0,
            white: 0,
            red: 0,
            blue: 0,
            green: 0,
        }
    }
    pub fn get_jewelry(&self, color: Color) -> u8 {
        match color {
            Color::Black => self.black,
            Color::White => self.white,
            Color::Red => self.red,
            Color::Blue => self.blue,
            Color::Green => self.green,
            _ => unreachable!(),
        }
    }
    pub fn add_jewelry(&mut self, color: Color) {
        match color {
            Color::Black => self.black += 1,
            Color::White => self.white += 1,
            Color::Red => self.red += 1,
            Color::Blue => self.blue += 1,
            Color::Green => self.green += 1,
            _ => unreachable!(),
        }
    }
}
