use crate::game::card_stack::Card;
use crate::game::color::Color;
use crate::game::game_command::GameCommand;
use crate::game::game_command::GameCommand::*;
use crate::game::jewelry_box::{JewelryBox, JEWELRIES};
use crate::game::token_stack::{Token, TokenStack};

use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;

const MAX_NUMBER_OF_HANDS: usize = 3;

#[derive(Clone)]
pub struct User {
    id: u8,
    hand: Vec<Card>,
    acquired_card: Vec<Card>,
    vp: u8,
    token_stack: TokenStack,
    rng: ThreadRng,
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
{}
        ",
            self.id, self.vp, self.hand, self.acquired_card, self.token_stack,
        )
    }
}
impl User {
    pub fn new(id: u8) -> User {
        let user = User {
            id: id,
            vp: 0,
            hand: vec![],
            acquired_card: vec![],
            token_stack: TokenStack::new(),
            rng: rand::thread_rng(),
        };

        user
    }
    pub fn read(&mut self) -> GameCommand {
        let random_value = self.rng.gen::<u8>() % 45;
        GameCommand::to_command(random_value)
    }
    pub fn get_id(&self) -> u8 {
        self.id
    }
    pub fn get_vp(&self) -> u8 {
        self.vp
    }
    pub fn add_vp(&mut self, point: u8) {
        self.vp += point;
    }
    pub fn get_number_of_tokens(&self, color: Color) -> u8 {
        self.token_stack.len(color)
    }
    pub fn get_number_of_hands(&self) -> u8 {
        self.hand.len() as u8
    }
    pub fn add_to_hands(&mut self, card: Card) {
        self.hand.push(card);
    }
    pub fn obtain(&mut self, card: Card) {
        self.vp += card.get_point();
        self.acquired_card.push(card);
    }
    pub fn is_over_capacity_of_hand(&self) -> bool {
        self.hand.len() >= MAX_NUMBER_OF_HANDS
    }
    pub fn can_get_token(&self) -> bool {
        self.token_stack.len_all() <= 10
    }
    pub fn add_token(&mut self, token: Token) {
        self.token_stack.add(token);
    }
    pub fn remove_token(&mut self, color: Color) {
        self.token_stack.remove(color);
    }
    pub fn pay(&mut self, card: &Card, board_token_stack: &mut TokenStack) {
        let jewelries = self.get_jewelries();
        let mut paid_tokens = vec![];

        for color in JEWELRIES.iter() {
            let cost = card.get_cost(*color);
            let number_of_token = self.get_number_of_tokens(*color);
            let jewelry = jewelries.get_jewelry(*color);
            paid_tokens.append(&mut self.pay_each_tokens(cost, number_of_token, jewelry, *color));
        }

        board_token_stack.addn(paid_tokens);
    }
    pub fn get_acquired_cards(&self) -> &Vec<Card> {
        &self.acquired_card
    }
    pub fn peek_card_in_hands(&self, order: u8) -> Option<&Card> {
        self.hand.get(order as usize)
    }
    pub fn uget_card_in_hands(&mut self, order: u8) -> Card {
        let card = self.hand.get(order as usize).unwrap();
        card.clone()
    }
    pub fn remove_card_in_hands(&mut self, order: u8) {
        self.hand.remove(order as usize);
    }
    pub fn get_jewelries(&self) -> JewelryBox {
        let mut jewelries = JewelryBox::new();

        for card in self.get_acquired_cards().iter() {
            jewelries.add_jewelry(card.get_color(), card.get_point());
        }

        jewelries
    }
    fn pay_each_tokens(
        &mut self,
        cost: u8,
        user_tokens: u8,
        user_jewelries: u8,
        color: Color,
    ) -> Vec<Token> {
        let mut paid_tokens = vec![];
        if user_jewelries >= cost {
            return paid_tokens;
        }

        let new_cost = cost - user_jewelries;

        // 宝石トークンを優先的に支払う
        if user_tokens == 0 {
            paid_tokens.append(&mut self.token_stack.removen(Color::Gold, new_cost));
        } else if user_tokens > new_cost {
            paid_tokens.append(&mut self.token_stack.removen(color, new_cost));
        } else {
            paid_tokens.append(&mut self.token_stack.removen(color, user_tokens));
            paid_tokens.append(
                &mut self
                    .token_stack
                    .removen(Color::Gold, new_cost - user_tokens),
            );
        }

        paid_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::game::card_stack::Card;
    use crate::game::color::Color::*;
    use crate::game::token_stack::{Token, TokenStack};

    #[test]
    fn test_new() {
        let user = User::new(1);
        assert_eq!(user.id, 1);
        assert_eq!(user.vp, 0);
        assert_eq!(user.hand.len(), 0);
        assert_eq!(user.acquired_card.len(), 0);
        assert_eq!(user.token_stack.len(Gold), 0);
    }

    #[test]
    fn test_get_jewelries() {
        let mut user = User::new(1);
        let cards = Card::load("json/test_card.json");
        for card in cards.into_iter() {
            user.acquired_card.push(card);
        }

        let jewelries = user.get_jewelries();
        assert_eq!(jewelries.get_jewelry(Black), 1)
    }

    #[test]
    fn test_pay_each_token() {
        let mut user = User::new(1);

        let tokens = user.pay_each_tokens(0, 0, 0, Black);
        assert_eq!(tokens, vec![]);

        let tokens = user.pay_each_tokens(2, 0, 2, Black);
        assert_eq!(tokens, vec![]);

        user.token_stack.addn(vec![Token::new(Black)]);
        let tokens = user.pay_each_tokens(2, 1, 1, Black);
        assert_eq!(tokens, vec![Token::new(Black)]);

        user.token_stack.addn(vec![Token::new(Gold)]);
        let tokens = user.pay_each_tokens(2, 0, 1, White);
        assert_eq!(tokens, vec![Token::new(Gold)]);

        user.token_stack
            .addn(vec![Token::new(Gold), Token::new(White)]);
        let tokens = user.pay_each_tokens(3, 1, 1, White);
        assert_eq!(tokens, vec![Token::new(White), Token::new(Gold)]);
    }
}

//     #[test]
//     fn test_pay() {
//        let mut user = User::new(1);
//        let cards = Card::load("json/test_card.json");
//        for card in cards.into_iter() {
//            user.acquired_card.push(card);
//        }
//        user.token_stack.insert(*color, vec![]);
//         user.pay();
//        }
//    }

//    pub fn pay(&mut self, card: &Card, token_stack: &mut HashMap<Color, Vec<Token>>) {
//        let jewelries = self.get_jewelries();
//
//        for color in  colors.iter() {
//            let cost = card.get_cost(*color);
//            let number_of_token = self.get_number_of_tokens(*color);
//            let jewelry = jewelries.get_jewelry(*color);
//            self.pay_every_token(cost, number_of_token, jewelry, *color, token_stack);
//        }
//    }
//
// use std::cell::RefCell;
// pub struct UserMock {
//     user: RefCell<Vec<User>>
// }
//
// impl UserMock {
//     pub fn get_number_of_tokens(&self, color: Color) -> u8 {
//        1
//     }
