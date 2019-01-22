use crate::card::Card;
use crate::color::Color;
use crate::jewelry_box::JewelryBox;
use crate::jewelries::JEWELRIES;
use crate::token::Token;

use std::collections::HashMap;
use std::fmt;

const MAX_NUMBER_OF_HANDS: usize = 3;

#[derive(Clone)]
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
    pub fn new(id: u8) -> User {
        let mut user = User {
            id: id,
            vp: 0,
            hand: vec![],
            acquired_card: vec![],
            token_stack: HashMap::new(),
        };

        use crate::color::Color::*;
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.iter() {
            user.token_stack.insert(*color, vec![]);
        }

        user
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
        let stack = self.token_stack.get(&color).unwrap();
        stack.len() as u8
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
    pub fn add_token(&mut self, token: Token) {
        let stack = self.token_stack.get_mut(&token.get_color()).unwrap();
        stack.push(token);
    }
    pub fn pay(&mut self, card: &Card, token_stack: &mut HashMap<Color, Vec<Token>>) {
        let jewelries = self.get_jewelries();
        let colors = [
            Color::Black,
            Color::White,
            Color::Red,
            Color::Blue,
            Color::Green,
        ];

        for color in  colors.iter() {
            let cost = card.get_cost(*color);
            let number_of_token = self.get_number_of_tokens(*color);
            let jewelry = jewelries.get_jewelry(*color);
            self.pay_every_token(cost, number_of_token, jewelry, *color, token_stack);
        }
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
    fn pay_every_token(
        &mut self,
        cost: u8,
        tokens: u8,
        jewelries: u8,
        color: Color,
        token_stack: &mut HashMap<Color, Vec<Token>>,
    ) {
        if jewelries >= cost {
            return;
        }
        let new_cost = cost - jewelries;
        if tokens > new_cost {
            self.sub_token(color, new_cost);
            let stack = token_stack.get_mut(&color).unwrap();
            for _ in 0..new_cost {
                stack.push(Token::new(color));
            }
        } else {
            self.sub_token(color, tokens);
            let stack = token_stack.get_mut(&color).unwrap();
            for _ in 0..tokens {
                stack.push(Token::new(color))
            }
            self.sub_token(Color::Gold, new_cost - tokens);
            let stack = token_stack.get_mut(&Color::Gold).unwrap();
            for _ in 0..new_cost - tokens {
                stack.push(Token::new(Color::Gold));
            }
        }
    }

    fn sub_token(&mut self, color: Color, cost: u8) {
        let stack = self.token_stack.get_mut(&color).unwrap();
        for _ in 0..cost {
            stack.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::card::Card;
    use crate::color::Color;

    #[test]
    fn test_new() {
        let user = User::new(1);
        assert_eq!(user.id, 1);
        assert_eq!(user.vp, 0);
        assert_eq!(user.hand.len(), 0);
        assert_eq!(user.acquired_card.len(), 0);
        assert_eq!(user.token_stack.get(&Color::Gold).unwrap().len(), 0);
    }
    
    #[test]
    fn test_get_jewelries() {
        let mut user = User::new(1);
        let cards = Card::load("json/test_card.json");
        for card in cards.into_iter() {
            user.acquired_card.push(card);
        }

        let jewelries = user.get_jewelries();
        assert_eq!(jewelries.get_jewelry(Color::Black), 1)

    }

   //  #[test]
   //  fn test_pay() {
   //      let user = User::new(1);
   //      user.pay();

   //      
   //  }
//    pub fn pay(&mut self, card: &Card, token_stack: &mut HashMap<Color, Vec<Token>>) {
//        let jewelries = self.get_jewelries();
//        let colors = [
//            Color::Black,
//            Color::White,
//            Color::Red,
//            Color::Blue,
//            Color::Green,
//        ];
//
//        for color in  colors.iter() {
//            let cost = card.get_cost(*color);
//            let number_of_token = self.get_number_of_tokens(*color);
//            let jewelry = jewelries.get_jewelry(*color);
//            self.pay_every_token(cost, number_of_token, jewelry, *color, token_stack);
//        }
//    }
//
}
// use std::cell::RefCell;
// pub struct UserMock {
//     user: RefCell<Vec<User>>
// }
//
// impl UserMock {
//     pub fn get_number_of_tokens(&self, color: Color) -> u8 {
//        1
//     }
// }
