use crate::color::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn is_available(
        &self,
        black_token: u8,
        white_token: u8,
        red_token: u8,
        blue_token: u8,
        green_token: u8,
    ) -> bool {
        if self.cost_black != black_token {
            return false;
        }

        if self.cost_white != white_token {
            return false;
        }

        if self.cost_red != red_token {
            return false;
        }

        if self.cost_blue != blue_token {
            return false;
        }

        if self.cost_green != green_token {
            return false;
        }

        return true;
    }
}
