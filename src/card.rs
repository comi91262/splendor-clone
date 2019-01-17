use crate::color::Color;
use crate::user::User;
use crate::jewelries::JEWELRIES;

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

impl fmt::Display for Card {
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
        let jewelries = user.get_jewelries();

        let mut gold_token = user.get_number_of_tokens(Color::Gold) as i8;

        for color in JEWELRIES.iter() {
            let token = user.get_number_of_tokens(*color);
            let jewelry = jewelries.get_jewelry(*color);
            let cost = self.get_cost(*color);
            self.estimate_gold_token(token + jewelry, cost, &mut gold_token);
        }

        gold_token >= 0
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_cost(&self, color: Color) -> u8 {
        use crate::color::Color::*;
        match color {
            Black => self.cost_black,
            White => self.cost_white,
            Red => self.cost_red,
            Blue => self.cost_blue,
            Green => self.cost_green,
            Gold => unreachable!(),
        }
    }

    fn estimate_gold_token(&self, user_token: u8, card_cost: u8, gold_token: &mut i8) {
        if user_token < card_cost {
            let diff = card_cost - user_token;
            *gold_token -= diff as i8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn test_estimate_gold_token() {
        let card: Card = Default::default();
        let mut gold = 0;
        card.estimate_gold_token(0, 0, &mut gold);
        assert_eq!(gold, 0);

        let mut gold = 0;
        card.estimate_gold_token(0, 2, &mut gold);
        assert_eq!(gold, -2);

        let mut gold = 2;
        card.estimate_gold_token(1, 2, &mut gold);
        assert_eq!(gold, 1);
    }

}
