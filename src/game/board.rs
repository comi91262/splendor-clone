use crate::game::card_stack::{Card, CardStack};
use crate::game::color::Color;
use crate::game::gem::{Gem, GEMS};
use crate::game::level::Level;
use crate::game::noble_tile::NobleTile;
use crate::game::token_stack::{Token, TokenStack};

use ndarray::Array2;
use std::fmt;

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
    card_stack: CardStack,
    token_stack: TokenStack,
    noble_tile: Vec<NobleTile>,
}

pub trait Board {
    fn new() -> Board;
    fn peek_card(&self, x: u8, y: u8) -> Option<&Card>;
    fn get_card(&mut self, x: u8, y: u8) -> Option<Card>;
    fn get_stack_card(&mut self, level: Level) -> Option<Card>;
    fn get_noble_tile(&mut self) -> &mut Vec<NobleTile>;
    fn get_required_cost(&self) -> Gem;
    fn get_number_of_tokens(&self, color: Color) -> u8;
    fn get_token_stack(&mut self) -> &mut TokenStack;
    fn uget_card(&mut self, x: u8, y: u8) -> Card;
    fn get_token(&mut self, color: Color) -> Option<Token>;
    fn uget_token(&mut self, color: Color) -> Token;
    fn can_get_token(&self, color: Color) -> bool;
    fn refill(&mut self, x: u8, y: u8);
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
貴族タイル: {:?}
----------------------------------------------------------------------------------------------
{}| {}| {}| {}  残り枚数: {}
{}| {}| {}| {}  残り枚数: {}
{}| {}| {}| {}  残り枚数: {}
----------------------------------------------------------------------------------------------
残りのトークン (黒, 白, 赤, 青, 緑, 金) = ({}, {}, {}, {}, {}, {})
        ",
            self.noble_tile,
            self.board[(0, 0)],
            self.board[(0, 1)],
            self.board[(0, 2)],
            self.board[(0, 3)],
            self.card_stack.len(Level::Three),
            self.board[(1, 0)],
            self.board[(1, 1)],
            self.board[(1, 2)],
            self.board[(1, 3)],
            self.card_stack.len(Level::Two),
            self.board[(2, 0)],
            self.board[(2, 1)],
            self.board[(2, 2)],
            self.board[(2, 3)],
            self.card_stack.len(Level::One),
            self.token_stack.len(Color::Black),
            self.token_stack.len(Color::White),
            self.token_stack.len(Color::Red),
            self.token_stack.len(Color::Blue),
            self.token_stack.len(Color::Green),
            self.token_stack.len(Color::Gold)
        )
    }
}


impl UseBoard for Board {
    fn new() -> Board {
        let mut board = Board {
            board: Array2::<Card>::default((3, 4)),
            card_stack: CardStack::new(),
            token_stack: TokenStack::new().fill(),
            noble_tile: NobleTile::create_stack(),
        };

        for (x, y) in COORDINATE.iter() {
            board.refill(*x, *y);
        }

        board
    }
     fn peek_card(&self, x: u8, y: u8) -> Option<&Card> {
        self.board.get((x as usize, y as usize))
    }
     fn get_card(&mut self, x: u8, y: u8) -> Option<Card> {
        match self.board.get_mut((x as usize, y as usize)) {
            Some(card) => {
                let card2 = card.clone();
                self.refill(x, y);
                Some(card2)
            }
            None => None,
        }
    }
     fn get_stack_card(&mut self, level: Level) -> Option<Card> {
        self.card_stack.get(level)
    }
     fn uget_card(&mut self, x: u8, y: u8) -> Card {
        let card = self.board.get_mut((x as usize, y as usize)).unwrap();
        let card2 = card.clone();
        self.refill(x, y);
        card2
    }
     fn get_token(&mut self, color: Color) -> Option<Token> {
        self.token_stack.remove(color)
    }
     fn uget_token(&mut self, color: Color) -> Token {
        self.token_stack.remove(color).unwrap()
    }
     fn get_token_stack(&mut self) -> &mut TokenStack {
        &mut self.token_stack
    }
     fn can_get_token(&self, color: Color) -> bool {
        self.get_number_of_tokens(color) >= LIMIT_OF_GETTING_SAME_TOKEN
    }
     fn get_noble_tile(&mut self) -> &mut Vec<NobleTile> {
        &mut self.noble_tile
    }
     fn get_required_cost(&self) -> Gem {
        let mut required_cost = Gem::new();
        for (x, y) in COORDINATE.iter() {
            if let Some(card) = self.peek_card(*x, *y) {
                for color in GEMS.iter() {
                    required_cost.add(*color, card.get_cost(*color));
                }
            }
        }
        required_cost
    }
    fn get_number_of_tokens(&self, color: Color) -> u8 {
        self.token_stack.len(color)
    }
    fn refill(&mut self, x: u8, y: u8) {
        let card = match x {
            0 => self.card_stack.get(Level::Three),
            1 => self.card_stack.get(Level::Two),
            2 => self.card_stack.get(Level::One),
            _ => unreachable!(),
        };

        match card {
            Some(card) => self.board[[x as usize, y as usize]] = card,
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    use crate::game::card_stack::Card;
    use crate::game::color::Color::{self, *};
    use crate::game::token_stack::{Token, TokenStack};
    use ndarray::Array2;

    // TOOD mock
    fn setup_board() {
        // let mut board = Board {
        //     board: Array2::<Card>::default((3, 4)),
        //     card_stack: CardStack::new(),
        //     token_stack: TokenStack::new().fill(),
        //     noble_tile: NobleTile::create_stack(),
        // };

        // for (x, y) in COORDINATE.iter() {
        //     board.refill(*x, *y);
        // }

        // board
    }
    //    #[test]
    //    fn test_get_required_cost() {
    //        let required_cost = Gem::new();
    //        for (x, y) in COORDINATE.iter() {
    //            if let Some(card) = self.peek_card(x, y) {
    //                for color in GEMS.iter() {
    //                    required_cost.add(*color, card.get_cost(*color));
    //                }
    //            }
    //        }
    //        required_cost
    //    }
    //     fn get_required_cost(&self) -> Gem {
    //        let required_cost = Gem::new();
    //        for (x, y) in COORDINATE.iter() {
    //            if let Some(card) = self.peek_card(x, y) {
    //                for color in GEMS.iter() {
    //                    required_cost.add(*color, card.get_cost(*color));
    //                }
    //            }
    //        }
    //        required_cost
    //    }
}
