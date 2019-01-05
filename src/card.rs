use crate::color::Color;
use crate::user::User;

use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub level: u8,
    pub color: Color,
    pub point: u8,
    pub cost_black: u8,
    pub cost_white: u8,
    pub cost_red: u8,
    pub cost_blue: u8,
    pub cost_green: u8,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Lv: {} {} {}点 {} {} {} {} {}",
            self.level,
            self.color,
            self.point,
            self.cost_black,
            self.cost_white,
            self.cost_red,
            self.cost_blue,
            self.cost_green,
        )
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            level: 0,
            color: Color::Black,
            point: 0,
            cost_black: 0,
            cost_white: 0,
            cost_red: 0,
            cost_blue: 0,
            cost_green: 0,
        }
    }
}

impl Card {
    pub fn get_point(&self) -> u8 {
        self.point
    }
    pub fn is_available(&self, user: &User) -> bool {
        let black_token = user.get_number_of_tokens(Color::Black);
        let white_token = user.get_number_of_tokens(Color::White);
        let red_token = user.get_number_of_tokens(Color::Red);
        let blue_token = user.get_number_of_tokens(Color::Blue);
        let green_token = user.get_number_of_tokens(Color::Green);
        let mut gold_token = user.get_number_of_tokens(Color::Gold) as i8;

        self.estimate_gold_token(black_token, self.cost_black, &mut gold_token);
        self.estimate_gold_token(white_token, self.cost_white, &mut gold_token);
        self.estimate_gold_token(red_token, self.cost_red, &mut gold_token);
        self.estimate_gold_token(blue_token, self.cost_blue, &mut gold_token);
        self.estimate_gold_token(green_token, self.cost_green, &mut gold_token);

        gold_token > 0
    }

    pub fn get_cost(&self, color: Color) -> u8 {
        match color {
            Color::Black => self.cost_black,
            Color::White => self.cost_white,
            Color::Red => self.cost_red,
            Color::Blue => self.cost_blue,
            Color::Green => self.cost_green,
            Color::Gold => unreachable!(),
        }
    }

    fn estimate_gold_token(&self, user_token: u8, card_cost: u8, gold_token: &mut i8) {
        if user_token < card_cost {
            let diff = card_cost - user_token;
            *gold_token -= diff as i8;
        }
    }
}
