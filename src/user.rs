use crate::card::Card;
use crate::color::Color;
use crate::token::Token;

use std::collections::HashMap;
use std::fmt;

const MAX_NUMBER_OF_HANDS: usize = 3;

pub struct User {
    id: u8,
    hand: Vec<Card>,
    acquired_card: Vec<Card>,
    vp: u8,
    token_stack: HashMap<Color, Vec<Token>>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
ID: {} 勝利点: {}
手札: {:?}
取得したカード:
{:?}
取得したトークン:
{:?}
        ",
            self.id, self.vp, self.hand, self.acquired_card, self.token_stack,
        )
    }
}
impl User {
    pub fn create() -> User {
        let mut user = User {
            id: 1,
            vp: 0,
            hand: vec![],
            acquired_card: vec![],
            token_stack: HashMap::new(),
        };
        user.token_stack.insert(Color::Black, vec![]);
        user.token_stack.insert(Color::White, vec![]);
        user.token_stack.insert(Color::Red, vec![]);
        user.token_stack.insert(Color::Blue, vec![]);
        user.token_stack.insert(Color::Green, vec![]);
        user.token_stack.insert(Color::Gold, vec![]);

        user
    }

    pub fn get_vp(&self) -> u8 {
        self.vp
    }
    pub fn add_vp(&mut self, point: u8) {
        self.vp += point;
    }
    fn _set_vp(&mut self, vp: u8) {
        self.vp = vp
    }
    fn _get_id(&self) -> u8 {
        self.id
    }
    pub fn get_number_of_tokens(&self, color: Color) -> u8 {
        let stack = self.token_stack.get(&color).unwrap();
        stack.len() as u8
    }
    pub fn add_to_hands(&mut self, card: Card) {
        self.hand.push(card);
    }
    pub fn obtain(&mut self, card: Card) {
        self.vp += card.get_point();
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
        self.pay_every_token(card, Color::Black);
        self.pay_every_token(card, Color::White);
        self.pay_every_token(card, Color::Red);
        self.pay_every_token(card, Color::Blue);
        self.pay_every_token(card, Color::Green);
    }
    pub fn get_acquired_cards(&self) -> &Vec<Card> {
        &self.acquired_card
    }
    fn pay_every_token(&mut self, card: &Card, color: Color) {
        let cost = card.get_cost(color);
        let token = self.get_number_of_tokens(color);
        if token > cost {
            self.sub_token(color, cost);
        } else {
            self.sub_token(color, token);
            self.sub_token(Color::Gold, cost - token);
        }
    }

    fn sub_token(&mut self, color: Color, cost: u8) {
        let stack = self.token_stack.get_mut(&color).unwrap();
        for _ in 0..cost {
            stack.pop();
        }
    }
}
