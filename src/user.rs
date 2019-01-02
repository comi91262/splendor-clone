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
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            vp: 0,
            hand: vec![],
            acquired_card: vec![],
            token_stack: HashMap::new(),
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
    pub fn get_number_of_tokens(&self, color: &Color) -> u8 {
        let stack = self.token_stack.get(color).unwrap();
        stack.len() as u8
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
    pub fn pay(&mut self, card: &Card) {
        self.pay_every_token(card, &Color::Black);
        self.pay_every_token(card, &Color::White);
        self.pay_every_token(card, &Color::Red);
        self.pay_every_token(card, &Color::Blue);
        self.pay_every_token(card, &Color::Green);
    }

    fn pay_every_token(&mut self, card: &Card, color: &Color) {
        let cost = card.get_cost(&color);
        let token = self.get_number_of_tokens(&color);
        if token > cost {
            self.sub_token(color, cost);
        } else {
            self.sub_token(color, token);
            self.sub_token(&Color::Gold, cost - token);
        }
    }

    fn sub_token(&mut self, color: &Color, cost: u8) {
        let stack = self.token_stack.get_mut(color).unwrap();
        for _ in 0..cost {
            stack.pop();
        }
    }
}
