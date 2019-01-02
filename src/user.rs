use crate::card::Card;
use crate::color::Color;
use crate::token::Token;
use std::collections::HashMap;

const MAX_NUMBER_OF_HANDS: usize = 3;

#[derive(Debug)]
pub struct User {
    id: u8,
    hand: Vec<Card>,
    acquired_card: Vec<Card>,
    vp: u8,
    token_stack: HashMap<Color, Vec<Token>>,
    black_token: u8,
    white_token: u8,
    red_token: u8,
    blue_token: u8,
    green_token: u8,
    gold_token: u8,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            vp: 0,
            hand: vec![],
            acquired_card: vec![],
            token_stack: HashMap::new(),
            black_token: 0,
            white_token: 0,
            red_token: 0,
            blue_token: 0,
            green_token: 0,
            gold_token: 0,
        }
    }
}

impl User {
    pub fn create(&mut self) {
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

    fn _get_vp(&self) -> u8 {
        self.vp
    }
    fn _set_vp(&mut self, vp: u8) {
        self.vp = vp
    }
    fn _get_id(&self) -> u8 {
        self.id
    }
    pub fn get_tokens(&self) -> (u8, u8, u8, u8, u8) {
        (
            self.black_token,
            self.white_token,
            self.red_token,
            self.blue_token,
            self.green_token,
        )
    }
    pub fn add_to_hands(&mut self, card: Card) {
        self.hand.push(card);
    }
    pub fn obtain(&mut self, card: Card) {
        self.acquired_card.push(card);
    }
    pub fn is_over_capacity_of_hand(&self) -> bool {
        self.hand.len() > MAX_NUMBER_OF_HANDS
    }
    pub fn add_token(&mut self, token: Token) {
        let stack = self.token_stack.get_mut(&token.get_color()).unwrap();
        stack.push(token);
    }
}
