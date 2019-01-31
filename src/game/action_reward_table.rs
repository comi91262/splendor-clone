use std::collections::HashMap;

use crate::game::board::Board;
use crate::game::color::Color;
use crate::game::color::Color::*;
use crate::game::game_command::GameCommand;
use crate::game::game_command::GameCommand::*;
use crate::game::gem::{Gem, GEMS};
use crate::game::user::User;

mod action_reward;
mod color_value;

pub struct ActionReward {
    action: GameCommand,
    reward: f32,
}
pub struct ColorValue(HashMap<Color, f32>);
pub struct ActionRewardTable(Vec<ActionReward>);

impl ActionRewardTable {
    pub fn new() -> ActionRewardTable {
        ActionRewardTable(vec![])
    }

    pub fn look(&mut self, step: u8, users: &mut Vec<User>, board: &mut Board) -> GameCommand {
        let mut user = User::new(1);
        let color_value = self.calc_color_value(&user, board);

        self.estimate(1, &mut user, board, &color_value);
        self.choice()
    }

    fn calc_color_value(&mut self, user: &User, board: &Board) -> ColorValue {
        let mut color_value = ColorValue::new();

        let required_cost = board.get_required_cost();
        let owned = user.get_owned_gems();

        // 基礎点 = 0.3
        // α = 1 - 所持宝石数 / 盤面の必要な宝石数
        for color in GEMS.iter() {
            color_value.set(
                *color,
                0.3 * (1.0 - owned.get(*color) as f32 / required_cost.get(*color) as f32),
            );
        }
        color_value.set_gold_value();
        color_value
    }

    pub fn estimate(
        &mut self,
        step: u8,
        user: &mut User,
        board: &mut Board,
        color_value: &ColorValue,
    ) {
        for input in 0..45 {
            let command = GameCommand::to_command(input);
            match command {
                ReserveDevelopmentCard { x, y } => {
                    let output = GameCommand::reserve_development_card(x, y, user, board);
                    match output {
                        Ok(_) => self
                            .0
                            .push(ActionReward::new(command, color_value.get(Gold))),
                        Err(_) => (),
                    };
                }
                BuyDevelopmentCard { x, y } => {
                    let output = GameCommand::buy_development_card(x, y, user, board);
                    match output {
                        Ok(_) => match user.get_acquired_cards().as_slice().last() {
                            Some(card) => self.0.push(ActionReward::new(
                                command,
                                card.get_point() as f32 + color_value.get(card.get_color()),
                            )),
                            None => (),
                        },
                        Err(_) => (),
                    };
                }
                SelectTwoSameTokens(c) => {
                    let result = GameCommand::select_two_same_tokens(c, user, board);
                    match result {
                        Ok(_) => self
                            .0
                            .push(ActionReward::new(command, 2.0 * color_value.get(c))),
                        Err(_) => (),
                    };
                }
                SelectThreeTokens(c1, c2, c3) => {
                    let t1 = user.get_number_of_tokens(c1);
                    let t2 = user.get_number_of_tokens(c2);
                    let t3 = user.get_number_of_tokens(c3);

                    let result = GameCommand::select_three_tokens(c1, c2, c3, user, board);

                    let t1 = user.get_number_of_tokens(c1) - t1;
                    let t2 = user.get_number_of_tokens(c2) - t2;
                    let t3 = user.get_number_of_tokens(c3) - t3;

                    let mut total = 0.0;

                    if t1 > 0 {
                        total += color_value.get(c1);
                    }
                    if t2 > 0 {
                        total += color_value.get(c2);
                    }
                    if t3 > 0 {
                        total += color_value.get(c3);
                    }

                    match result {
                        Ok(_) => self.0.push(ActionReward::new(command, total)),
                        Err(_) => (),
                    };
                }
                ReserveStackCard(l) => {
                    let result = GameCommand::reserve_stack_card(l, user, board);
                    match result {
                        Ok(_) => self.0.push(ActionReward::new(command, 0.0)),
                        Err(_) => (),
                    };
                }
                BuyReservedCard(index) => {
                    let output = GameCommand::buy_reserved_card(index, user, board);
                    match output {
                        Ok(_) => match user.get_acquired_cards().as_slice().last() {
                            Some(card) => self.0.push(ActionReward::new(
                                command,
                                card.get_point() as f32 + color_value.get(card.get_color()),
                            )),
                            None => (),
                        },
                        Err(_) => (),
                    };
                }
            }
        }
    }

    fn choice(&self) -> GameCommand {
        let mut max_value = 0.0;
        let mut command = GameCommand::ReserveDevelopmentCard { x: 0, y: 0 };

        for e in self.0.iter() {
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
    use super::ColorValue;
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
        // let mut users = game.copy_users();
        let mut user = User::new(1);
        let mut table = ActionRewardTable::new();

        table.calc_color_value(&mut user, &mut board);
    }

    #[test]
    fn test_estimate() {
        let mut game = Game::new();
        let mut board = game.copy_board();
        // let mut users = game.copy_users();
        let mut user = User::new(1);
        let mut table = ActionRewardTable::new();
        let mut color_value = ColorValue::new();

        table.estimate(1, &mut user, &mut board, &color_value);
    }

    #[test]
    fn test_choise() {
        let mut game = Game::new();
        let mut board = game.copy_board();
        let mut users = game.copy_users();
        let mut table = ActionRewardTable::new();

        table.choice();
    }
}
