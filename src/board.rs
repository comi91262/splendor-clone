use crate::card::Card;
use crate::color::Color;
use crate::level::Level;
use crate::noble_tile::NobleTile;
use crate::token::Token;

use ndarray::Array2;
use std::fmt;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LIMIT_OF_GETTING_SAME_TOKEN: u8 = 4;

const COORDINATE: [(u8, u8); 12] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (1, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 0),
    (2, 1),
    (2, 2),
    (2, 3),
];

#[derive(Clone)]
pub struct Board {
    board: Array2<Card>,
    stack: HashMap<Level, Vec<Card>>,
    token_stack: HashMap<Color, Vec<Token>>,
    noble_tile: Vec<NobleTile>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
{:?}
----------------------------------------------------------------------------------------------
{}| {}| {}| {}
{}| {}| {}| {}
{}| {}| {}| {}
----------------------------------------------------------------------------------------------
レベル3 残り残数: {}
レベル2 残り残数: {}
レベル1 残り残数: {}
黒トークン 残り枚数: {}
白トークン 残り枚数: {}
赤トークン 残り枚数: {}
青トークン 残り枚数: {}
緑トークン 残り枚数: {}
金トークン 残り枚数: {}
        ",
            self.noble_tile,
            self.board[(0, 0)],
            self.board[(0, 1)],
            self.board[(0, 2)],
            self.board[(0, 3)],
            self.board[(1, 0)],
            self.board[(1, 1)],
            self.board[(1, 2)],
            self.board[(1, 3)],
            self.board[(2, 0)],
            self.board[(2, 1)],
            self.board[(2, 2)],
            self.board[(2, 3)],
            self.stack.get(&Level::One).unwrap().len(),
            self.stack.get(&Level::Two).unwrap().len(),
            self.stack.get(&Level::Three).unwrap().len(),
            self.token_stack.get(&Color::Black).unwrap().len(),
            self.token_stack.get(&Color::White).unwrap().len(),
            self.token_stack.get(&Color::Red).unwrap().len(),
            self.token_stack.get(&Color::Blue).unwrap().len(),
            self.token_stack.get(&Color::Green).unwrap().len(),
            self.token_stack.get(&Color::Gold).unwrap().len(),
        )
    }
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            board: Array2::<Card>::default((3, 4)),
            stack: HashMap::new(),
            token_stack: HashMap::new(),
            noble_tile: vec![],
        };

        board.stack.insert(Level::One, vec![]);
        board.stack.insert(Level::Two, vec![]);
        board.stack.insert(Level::Three, vec![]);

        for result in BufReader::new(File::open("card.json").unwrap()).lines() {
            let l = result.unwrap();
            let card: Card = serde_json::from_str(&l).unwrap();

            match card {
                Card { level: 1, .. } => board.stack.get_mut(&Level::One).unwrap().push(card),
                Card { level: 2, .. } => board.stack.get_mut(&Level::Two).unwrap().push(card),
                Card { level: 3, .. } => board.stack.get_mut(&Level::Three).unwrap().push(card),
                Card { level: _, .. } => unreachable!(),
            }
        }
        for (x, y) in COORDINATE.iter() {
            board.refill(*x, *y);
        }

        use crate::color::Color::*;
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.iter() {
            board
                .token_stack
                .insert(*color, Token::create_stack(*color));
        }

        board.noble_tile = NobleTile::create_stack();

        board
    }
    pub fn peek_card(&self, x: u8, y: u8) -> Option<&Card> {
        self.board.get((x as usize, y as usize))
    }
    pub fn get_card(&mut self, x: u8, y: u8) -> Option<Card> {
        match self.board.get_mut((x as usize, y as usize)) {
            Some(card) => {
                let card2 = card.clone();
                self.refill(x, y);
                Some(card2)
            }
            None => None,
        }
    }
    pub fn get_stack_card(&mut self, level: Level) -> Option<Card> {
        self.stack.get_mut(&level).unwrap().pop()
    }
    pub fn uget_card(&mut self, x: u8, y: u8) -> Card {
        let card = self.board.get_mut((x as usize, y as usize)).unwrap();
        let card2 = card.clone();
        self.refill(x, y);
        card2
    }
    pub fn get_token(&mut self, color: Color) -> Option<Token> {
        let stack = self.token_stack.get_mut(&color).unwrap();
        stack.pop()
    }
    pub fn uget_token(&mut self, color: Color) -> Token {
        let stack = self.token_stack.get_mut(&color).unwrap();
        stack.pop().unwrap()
    }
    pub fn get_token_stack(&mut self) -> &mut HashMap<Color, Vec<Token>> {
        &mut self.token_stack
    }
    pub fn can_get_token(&self, color: Color) -> bool {
        self.get_number_of_tokens(color) >= LIMIT_OF_GETTING_SAME_TOKEN
    }
    pub fn get_noble_tile(&mut self) -> &mut Vec<NobleTile> {
        &mut self.noble_tile
    }
    fn get_number_of_tokens(&self, color: Color) -> u8 {
        let stack = self.token_stack.get(&color).unwrap();
        stack.len() as u8
    }
    fn refill(&mut self, x: u8, y: u8) {
        let stack = match x {
            0 => self.stack.get_mut(&Level::Three).unwrap(),
            1 => self.stack.get_mut(&Level::Two).unwrap(),
            2 => self.stack.get_mut(&Level::One).unwrap(),
            _ => unreachable!(),
        };

        match stack.pop() {
            Some(card) => self.board[[x as usize, y as usize]] = card,
            None => (),
        }
    }
}
