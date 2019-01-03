use crate::card::Card;
use crate::color::Color;
use crate::level::Level;
use crate::token::Token;
use ndarray::Array2;
use std::fmt;

use std::collections::HashMap;
use std::fs::File;
// use std::io::Write;
use std::io::{BufRead, BufReader};

const LIMIT_OF_GETTING_SAME_TOKEN: u8 = 4;

#[derive(Debug)]
pub struct Board {
    board: Array2<Card>,
    level1_stack: Vec<Card>,
    level2_stack: Vec<Card>,
    level3_stack: Vec<Card>,
    token_stack: HashMap<Color, Vec<Token>>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: Array2::<Card>::default((3, 4)),
            level1_stack: vec![],
            level2_stack: vec![],
            level3_stack: vec![],
            token_stack: HashMap::new(),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
レベル3 残り残数: {:?}
レベル2 残り残数: {:?}
レベル1 残り残数: {:?}
        ",
            self.level3_stack.len(),
            self.level2_stack.len(),
            self.level1_stack.len()
        )
    }
}

impl Board {
    pub fn peek_card(&self, x: u8, y: u8) -> Option<&Card> {
        self.board.get((x as usize, y as usize))
    }
    pub fn get_card(&mut self, x: u8, y: u8) -> Option<Card> {
        match self.board.get_mut((x as usize, y as usize)) {
            Some(card) => {
                let card2 = card.clone();
                self.refill(x, y, 1);
                Some(card2)
            }
            None => None,
        }
    }
    pub fn get_stack_card(&mut self, level: Level) -> Option<Card> {
        match level {
            Level::One => {
                self.level1_stack.pop()
            }
            Level::Two => {
                self.level2_stack.pop()
            }
            Level::Three => {
                self.level3_stack.pop()
            }
        }
    }
    pub fn uget_card(&mut self, x: u8, y: u8) -> Card {
        let card = self.board.get_mut((x as usize, y as usize)).unwrap();
        let card2 = card.clone();
        self.refill(x, y, 1);
        card2
    }
    pub fn create(&mut self) {
        for result in BufReader::new(File::open("card.json").unwrap()).lines() {
            let l = result.unwrap();
            let card: Card = serde_json::from_str(&l).unwrap();

            match card {
                Card { level: 1, .. } => self.level1_stack.push(card),
                Card { level: 2, .. } => self.level2_stack.push(card),
                Card { level: 3, .. } => self.level3_stack.push(card),
                Card { level: _, .. } => unreachable!(),
            }
        }

        self.refill(0, 0, 3);
        self.refill(0, 1, 3);
        self.refill(0, 2, 3);
        self.refill(0, 3, 3);
        self.refill(1, 0, 2);
        self.refill(1, 1, 2);
        self.refill(1, 2, 2);
        self.refill(1, 3, 2);
        self.refill(2, 0, 3);
        self.refill(2, 1, 3);
        self.refill(2, 2, 3);
        self.refill(2, 3, 3);

        self.token_stack
            .insert(Color::Black, Token::create_stack(Color::Black));
        self.token_stack
            .insert(Color::White, Token::create_stack(Color::White));
        self.token_stack
            .insert(Color::Red, Token::create_stack(Color::Red));
        self.token_stack
            .insert(Color::Blue, Token::create_stack(Color::Blue));
        self.token_stack
            .insert(Color::Green, Token::create_stack(Color::Green));
        self.token_stack
            .insert(Color::Gold, Token::create_stack(Color::Gold));
    }
    pub fn get_token(&mut self, color: Color) -> Option<Token> {
        let stack = self.token_stack.get_mut(&color).unwrap();
        stack.pop()
    }
    pub fn uget_token(&mut self, color: Color) -> Token {
        let stack = self.token_stack.get_mut(&color).unwrap();
        stack.pop().unwrap()
    }
    pub fn can_get_token(&self, color: Color) -> bool {
        self.get_number_of_tokens(color) >= LIMIT_OF_GETTING_SAME_TOKEN
    }
    fn get_number_of_tokens(&self, color: Color) -> u8 {
        let stack = self.token_stack.get(&color).unwrap();
        stack.len() as u8
    }
    fn refill(&mut self, x: u8, y: u8, level: u8) {
        // TODO boader
        match level {
            1 => match self.level1_stack.pop() {
                Some(card) => self.board[[x as usize, y as usize]] = card,
                None => (),
            },
            2 => match self.level2_stack.pop() {
                Some(card) => self.board[[x as usize, y as usize]] = card,
                None => (),
            },
            3 => match self.level3_stack.pop() {
                Some(card) => self.board[[x as usize, y as usize]] = card,
                None => (),
            },
            _ => unreachable!(),
        }
    }

}
