use crate::game::color::Color::{self, *};

pub const GEMS: [Color; 5] = [Black, White, Red, Blue, Green];
pub struct Gem {
    black: u8,
    white: u8,
    red: u8,
    blue: u8,
    green: u8,
}

impl Gem {
    pub fn new() -> Gem {
        Gem {
            black: 0,
            white: 0,
            red: 0,
            blue: 0,
            green: 0,
        }
    }
    pub fn get(&self, color: Color) -> u8 {
        match color {
            Black => self.black,
            White => self.white,
            Red => self.red,
            Blue => self.blue,
            Green => self.green,
            _ => unreachable!(),
        }
    }
    pub fn add(&mut self, color: Color, gems: u8) {
        match color {
            Black => self.black += gems,
            White => self.white += gems,
            Red => self.red += gems,
            Blue => self.blue += gems,
            Green => self.green += gems,
            _ => unreachable!(),
        }
    }
}
