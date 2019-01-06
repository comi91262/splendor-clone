use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::color::Color;
use crate::user::User;

#[derive(Serialize, Deserialize)]
pub struct NobleTile {
    point: u8,
    black_bonus: u8,
    white_bonus: u8,
    red_bonus: u8,
    blue_bonus: u8,
    green_bonus: u8,
}

impl fmt::Debug for NobleTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}点 {} {} {} {} {}",
            self.point,
            self.black_bonus,
            self.white_bonus,
            self.red_bonus,
            self.blue_bonus,
            self.green_bonus,
        )
    }
}

impl fmt::Display for NobleTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}点 {} {} {} {} {}",
            self.point,
            self.black_bonus,
            self.white_bonus,
            self.red_bonus,
            self.blue_bonus,
            self.green_bonus,
        )
    }
}

impl NobleTile {
    pub fn create_stack() -> Vec<NobleTile> {
        let mut stack = vec![];

        for result in BufReader::new(File::open("noble_tile.json").unwrap()).lines() {
            let l = result.unwrap();
            let tile: NobleTile = serde_json::from_str(&l).unwrap();
            stack.push(tile);
        }

        stack
    }
    pub fn get_point(&self) -> u8 {
        self.point
    }
    pub fn can_visit(&self, user: &User) -> bool {
        struct JewelryBox {
            black: u8,
            white: u8,
            red: u8,
            blue: u8,
            green: u8,
        };

        let mut jewelries = JewelryBox {
            black: 0,
            white: 0,
            red: 0,
            blue: 0,
            green: 0,
        };
        for card in user.get_acquired_cards().iter() {
            match card.get_color() {
                Color::Black => jewelries.black += 1,
                Color::White => jewelries.white += 1,
                Color::Red => jewelries.red += 1,
                Color::Blue => jewelries.blue += 1,
                Color::Green => jewelries.green += 1,
                Color::Gold => (),
            }
        }

        jewelries.black >= self.black_bonus
            && jewelries.white >= self.white_bonus
            && jewelries.red >= self.red_bonus
            && jewelries.blue >= self.blue_bonus
            && jewelries.green >= self.green_bonus
    }
}
