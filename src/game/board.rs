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

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            board: Array2::<Card>::default((3, 4)),
            card_stack: CardStack::new(Some("data/card.json")),
            token_stack: TokenStack::new().fill(),
            noble_tile: NobleTile::create_stack(),
        };

        for (x, y) in COORDINATE.iter() {
            board.refill(*x, *y);
        }

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
        self.card_stack.get(level)
    }
    pub fn uget_card(&mut self, x: u8, y: u8) -> Card {
        let card = self.board.get_mut((x as usize, y as usize)).unwrap();
        let card2 = card.clone();
        self.refill(x, y);
        card2
    }
    pub fn get_token(&mut self, color: Color) -> Option<Token> {
        self.token_stack.remove(color)
    }
    pub fn uget_token(&mut self, color: Color) -> Token {
        self.token_stack.remove(color).unwrap()
    }
    pub fn get_token_stack(&mut self) -> &mut TokenStack {
        &mut self.token_stack
    }
    pub fn can_get_token(&self, color: Color) -> bool {
        self.get_number_of_tokens(color) >= LIMIT_OF_GETTING_SAME_TOKEN
    }
    pub fn get_noble_tile(&mut self) -> &mut Vec<NobleTile> {
        &mut self.noble_tile
    }
    pub fn get_required_cost(&self) -> Gem {
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
    use super::COORDINATE;
    use crate::game::card_stack::{Card, CardStack};
    use crate::game::color::Color::{self, *};
    use crate::game::gem::{Gem, GEMS};
    use crate::game::level::Level;
    use crate::game::token_stack::{Token, TokenStack};
    use ndarray::Array2;

    // TOOD mock
    struct TestBoard {
        board: Array2<Card>,
        card_stack: CardStack,
    }
    impl TestBoard {
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
        fn peek_card(&self, x: u8, y: u8) -> Option<&Card> {
            self.board.get((x as usize, y as usize))
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

    fn setup_board() -> TestBoard {
        let mut board = TestBoard {
            board: Array2::<Card>::default((3, 4)),
            card_stack: CardStack::new(Some("data/test_card.json")),
        };
        for (x, y) in COORDINATE.iter() {
            board.refill(*x, *y);
        }

        board
    }
    #[test]
    fn test_get_required_cost() {
        let board = setup_board();
        assert_eq!(1, board.get_required_cost().get(Color::White));
        assert_eq!(1, board.get_required_cost().get(Color::Red));
        assert_eq!(1, board.get_required_cost().get(Color::Green));
    }
}
