use crate::card::Card;
use crate::color::Color;
use crate::token::Token;
use ndarray::Array2;

use std::collections::HashMap;
use std::fs::File;
// use std::io::Write;
use std::io::{BufRead, BufReader};

pub struct Board {
    board: Array2<Card>,
    level1_stack: Vec<Card>,
    token_stack: HashMap<Color, Vec<Token>>,
    black_token_stack: Vec<Token>,
    white_token_stack: Vec<Token>,
    red_token_stack: Vec<Token>,
    blue_token_stack: Vec<Token>,
    green_token_stack: Vec<Token>,
    gold_token_stack: Vec<Token>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: Array2::<Card>::default((3, 4)),
            level1_stack: vec![],
            token_stack: HashMap::new(),
            white_token_stack: vec![],
            black_token_stack: vec![],
            red_token_stack: vec![],
            blue_token_stack: vec![],
            green_token_stack: vec![],
            gold_token_stack: vec![],
        }
    }
}

impl Board {
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

    pub fn create(&mut self) {
        for result in BufReader::new(File::open("card.json").unwrap()).lines() {
            let l = result.unwrap();
            let card: Card = serde_json::from_str(&l).unwrap();

            match card {
                Card { level: 1, .. } => self.level1_stack.push(card),
                Card { level: _, .. } => (),
            }
        }

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

    pub fn drop_card(&mut self, x: u8, y: u8) {
        // TODO boader
        let card = self.level1_stack.pop().unwrap();
        self.board[[x as usize, y as usize]] = card;
    }

    pub fn get_token(&mut self, color: Color) -> Option<Token> {
        let mut stack = self.token_stack.get_mut(&color).unwrap();
        stack.pop()
    }

    fn refill(&mut self, x: u8, y: u8, level: u8) {
        match level {
            1 => {
                self.board[[x as usize, y as usize]] = self.level1_stack.pop().unwrap();
            }
            _ => unreachable!(),
        }
    }
}
