use crate::card::Card;
use ndarray::Array2;

use std::fs::File;
// use std::io::Write;
use std::io::{BufRead, BufReader};

pub struct Board {
    board: Array2<Card>,
    level1_stack: Vec<Card>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: Array2::<Card>::default((3, 4)),
            level1_stack: vec![],
        }
    }
}

impl Board {
    pub fn get_card(&self, x: u8, y: u8) -> &Card {
        &self.board[[x as usize, y as usize]]
    }

    pub fn create(&mut self) {
        for result in BufReader::new(File::open("card.json").unwrap()).lines() {
            let l = result.unwrap();
            let card: Card = serde_json::from_str(&l).unwrap();

            match card {
                Card { level: 1, .. } => self.level1_stack.push(card),
                Card { level: _, .. } => (),
            }
        }
    }

    pub fn drop_card(&mut self, x: u8, y: u8) {
        // TODO boader
        let card = self.level1_stack.pop().unwrap();
        self.board[[x as usize, y as usize]] = card;
    }
}
