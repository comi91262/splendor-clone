use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::game::color::Color;
use crate::game::color::Color::*;
use crate::game::gem::{Gem, GEMS};
use serde_derive::{Deserialize, Serialize};

const MAX_NUMBER_OF_TILES: u8 = 4;

#[derive(Serialize, Deserialize, Clone)]
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

        for result in BufReader::new(File::open("data/noble_tile.json").unwrap()).lines() {
            let l = result.unwrap();
            let tile: NobleTile = serde_json::from_str(&l).unwrap();
            stack.push(tile);
        }

        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        stack.shuffle(&mut rng);

        for _ in 0..10 - MAX_NUMBER_OF_TILES {
            stack.pop();
        }
        stack
    }
    pub fn get_point(&self) -> u8 {
        self.point
    }
    pub fn can_visit(&self, gems: &Gem) -> bool {
        for color in GEMS.iter() {
            if self.get_bonus(*color) > gems.get(*color) {
                return false;
            }
        }

        return true;
    }
    fn get_bonus(&self, color: Color) -> u8 {
        match color {
            Black => self.black_bonus,
            White => self.white_bonus,
            Red => self.red_bonus,
            Blue => self.blue_bonus,
            Green => self.green_bonus,
            _ => unreachable!(),
        }
    }
}
