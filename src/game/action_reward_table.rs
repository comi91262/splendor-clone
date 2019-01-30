use std::collections::HashMap;

use crate::game::board::Board;
use crate::game::color::Color;
use crate::game::color::Color::*;
use crate::game::game_command::GameCommand;
use crate::game::game_command::GameCommand::*;
use crate::game::jewelry_box::{JewelryBox, JEWELRIES};
use crate::game::user::User;

mod action_reward;

pub struct ActionReward {
    action: GameCommand,
    reward: f32,
}

pub struct ActionRewardTable {
    entity: Vec<ActionReward>, // TODO to const
    color_value: HashMap<Color, f32>,
}

impl ActionRewardTable {
    pub fn new() -> ActionRewardTable {
        let mut color_value = HashMap::new();
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.into_iter() {
            color_value.insert(&color, 0.0);
        }

        ActionRewardTable {
            entity: vec![],
            color_value: color_value,
        }
    }

    pub fn look(&mut self, step: u8, users: &Vec<User>, board: &Board) -> GameCommand {
        self.calc_color_value(users, board);
        self.estimate(users, board);
        self.choice()
    }

    fn calc_color_value(&mut self, user: &User, board: &Board) {
        let mut required_cost = JewelryBox::new();
        let mut owned = JewelryBox::new();

        // 基礎点 = 0.3
        // α = 1 - 所持宝石数 / 盤面の必要な宝石数
        for row in 0..3 {
            for col in 0..4 {
                if let Some(card) = board.peek_card(row, col) {
                    for color in JEWELRIES.iter() {
                        required_cost.add_jewelry(*color, card.get_cost(*color));
                    }
                }
            }
        }

        for card in user.get_acquired_cards().iter() {
            for color in JEWELRIES.iter() {
                owned.add_jewelry(*color, card.get_cost(*color));
            }
        }

        let mut max_color_value = 0.0;
        for color in JEWELRIES.iter() {
            let color_value = self.color_value.get_mut(color).unwrap();
            *color_value = 0.3
                * (1.0
                    - owned.get_jewelry(*color) as f32 / required_cost.get_jewelry(*color) as f32);

            if max_color_value <= *color_value {
                max_color_value = *color_value;
            }
        }

        let gold_color_value = self.color_value.get_mut(&Color::Gold).unwrap();
        *gold_color_value = max_color_value;
    }

    pub fn estimate(&self, step: u8, user: &User, board: &Board) {
        let mut action_rewards: Vec<ActionReward> = vec![];

        for input in 0..45 {
            let command = GameCommand::to_command(input);
            let mut user = user.clone();
            let mut board = board.clone();
            match command {
                ReserveDevelopmentCard { x, y } => {
                    let output = GameCommand::reserve_development_card(x, y, &mut user, &mut board);
                    match output {
                        Ok(_) => action_rewards.push(ActionReward::new(
                            command,
                            *self.color_value.get(&Color::Gold).unwrap(),
                        )),
                        Err(_) => (),
                    };
                }
                BuyDevelopmentCard { x, y } => {
                    let output = GameCommand::buy_development_card(x, y, &mut user, &mut board);
                    match output {
                        Ok(_) => match user.get_acquired_cards().as_slice().last() {
                            Some(card) => action_rewards.push(ActionReward::new(
                                command,
                                card.get_point() as f32
                                    + self.color_value.get(&card.get_color()).unwrap(),
                            )),
                            None => (),
                        },
                        Err(_) => (),
                    };
                }
                SelectTwoSameTokens(c) => {
                    let result = GameCommand::select_two_same_tokens(c, &mut user, &mut board);
                    match result {
                        Ok(_) => action_rewards.push(ActionReward::new(
                            command,
                            2.0 * *self.color_value.get(&c).unwrap(),
                        )),
                        Err(_) => (),
                    };
                }
                SelectThreeTokens(c1, c2, c3) => {
                    let t1 = user.get_number_of_tokens(c1);
                    let t2 = user.get_number_of_tokens(c2);
                    let t3 = user.get_number_of_tokens(c3);

                    let result =
                        GameCommand::select_three_tokens(c1, c2, c3, &mut user, &mut board);

                    let t1 = user.get_number_of_tokens(c1) - t1;
                    let t2 = user.get_number_of_tokens(c2) - t2;
                    let t3 = user.get_number_of_tokens(c3) - t3;

                    let mut total = 0.0;

                    if t1 > 0 {
                        total += self.color_value.get(&c1).unwrap();
                    }
                    if t2 > 0 {
                        total += self.color_value.get(&c2).unwrap();
                    }
                    if t3 > 0 {
                        total += self.color_value.get(&c3).unwrap();
                    }

                    match result {
                        Ok(_) => action_rewards.push(ActionReward::new(command, total)),
                        Err(_) => (),
                    };
                }
                ReserveStackCard(l) => {
                    let result = GameCommand::reserve_stack_card(l, &mut user, &mut board);
                    match result {
                        Ok(_) => action_rewards.push(ActionReward::new(command, 0.0)),
                        Err(_) => (),
                    };
                }
                BuyReservedCard(index) => {
                    let output = GameCommand::buy_reserved_card(index, &mut user, &mut board);
                    match output {
                        Ok(_) => match user.get_acquired_cards().as_slice().last() {
                            Some(card) => action_rewards.push(ActionReward::new(
                                command,
                                card.get_point() as f32
                                    + self.color_value.get(&card.get_color()).unwrap(),
                            )),
                            None => (),
                        },
                        Err(_) => (),
                    };
                }
            }
        }

        self.entity = action_rewards;
    }

    fn choice(&self) -> GameCommand {
        let mut max_value = 0.0;
        let mut command = GameCommand::ReserveDevelopmentCard { x: 0, y: 0 };

        for e in self.entity.iter() {
            // println!("{:?}", e);
            match e {
                ActionReward { action, reward } => {
                    if *reward > max_value {
                        command = action.clone();
                        max_value = *reward;
                    }
                }
            }
        }

        command
    }
}

#[cfg(test)]
mod tests {
    use super::ActionRewardTable;
    use crate::game::board::Board;
    use crate::game::user::User;
    use crate::game::Game;

    use crate::game::card_stack::Card;
    use crate::game::color::Color::*;
    use crate::game::token_stack::{Token, TokenStack};

    #[test]
    fn test_calc_color_value() {
        let mut game = Game::new();
        let mut board = game.copy_board();
        let mut users = game.copy_users();
        let mut table = ActionRewardTable::new();

        table.calc_color_value(users, board);
    }

    #[test]
    fn test_estimate() {
        let mut game = Game::new();
        let mut board = game.copy_board();
        let mut users = game.copy_users();
        let mut table = ActionRewardTable::new();

        table.estimate();
    }

    #[test]
    fn test_choise() {
        let mut game = Game::new();
        let mut board = game.copy_board();
        let mut users = game.copy_users();
        let mut table = ActionRewardTable::new();

        table.choise();
    }
}
