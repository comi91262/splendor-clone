use crate::card::Card;

#[derive(Debug)]
pub struct User {
    id: u8,
    hand: Vec<Card>,
    acquired_card: Vec<Card>,
    vp: u8,
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
    fn _get_vp(&self) -> u8 {
        self.vp
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
    pub fn obtain(&mut self, card: Card) {
        self.acquired_card.push(card);
    }
    fn _set_vp(&mut self, vp: u8) {
        self.vp = vp
    }
}
