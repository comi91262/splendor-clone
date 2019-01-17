use crate::board::Board;
use crate::color::Color;
use crate::game::game_command::GameCommand;
use crate::jewelries::JEWELRIES;
use crate::jewelry_box::JewelryBox;
use crate::user::User;

use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;

mod game_command;

const VP_TO_END: u8 = 15;

pub struct Game {
    rng: ThreadRng,
    color_value: HashMap<Color, f32>,
}

pub struct ActionReward {
    pub action: GameCommand,
    pub reward: f32,
}

impl ActionReward {
    fn new(action: GameCommand, reward: f32) -> ActionReward {
        ActionReward {
            action: action,
            reward: reward,
        }
    }
}

impl fmt::Debug for ActionReward {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action: {} 報酬点: {}", self.action, self.reward)
    }
}

impl Game {
    pub fn new() -> Game {
        let mut color_value = HashMap::new();
        color_value.insert(Color::Black, 0.0);
        color_value.insert(Color::White, 0.0);
        color_value.insert(Color::Red, 0.0);
        color_value.insert(Color::Blue, 0.0);
        color_value.insert(Color::Green, 0.0);
        color_value.insert(Color::Gold, 0.0);

        Game {
            rng: rand::thread_rng(),
            color_value: color_value,
        }
    }
    pub fn read(&mut self) -> GameCommand {
        let random_value = self.rng.gen::<u8>() % 45;
        game_command::to_command(random_value)
    }

    pub fn eval(&mut self, input: GameCommand, user: &mut User, board: &mut Board) -> String {
        let output = self.eval_by_selection(input, user, board);

        match output {
            Ok(result) => {
                return result.to_string();
            }
            Err(error_msg) => {
                println!("{}", error_msg);
                let input = self.read();
                self.eval(input, user, board)
            }
        }
    }

    fn eval_by_selection(
        &self,
        input: GameCommand,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        use self::game_command::GameCommand::*;
        use self::game_command::*;

        match input {
            ReserveDevelopmentCard { x, y } => reserve_development_card(x, y, user, board),
            BuyDevelopmentCard { x, y } => buy_development_card(x, y, user, board),
            SelectTwoSameTokens(color) => select_two_same_tokens(color, user, board),
            SelectThreeTokens(color1, color2, color3) => {
                select_three_tokens(color1, color2, color3, user, board)
            }
            ReserveStackCard(level) => reserve_stack_card(level, user, board),
            BuyReservedCard(index) => buy_reserved_card(index, user, board),
        }
    }

    pub fn print(&self, result: &str, user: &User) -> () {
        println!("{}", result);
        println!("ユーザーステータス: {}", user);
    }

    pub fn is_over(&self, users: Vec<&User>) -> bool {
        let mut result = false;
        for user in users.iter() {
            if user.get_vp() >= VP_TO_END {
                result = true;
                println!("プレイヤー{}が勝利しました", user.get_id());
            }
        }
        result
    }

    pub fn look(&mut self, step: u8, user: &User, board: &Board) -> GameCommand {
        use self::game_command::GameCommand::*;
        use self::game_command::*;
        let mut action_rewards: Vec<ActionReward> = vec![];
        self.calc_color_value(user, board);

        for input in 0..45 {
            let command = to_command(input);
            let mut user = user.clone();
            let mut board = board.clone();
            match command {
                ReserveDevelopmentCard { x, y } => {
                    let output = reserve_development_card(x, y, &mut user, &mut board);
                    match output {
                        Ok(_) => action_rewards.push(ActionReward::new(
                            command,
                            *self.color_value.get(&Color::Gold).unwrap(),
                        )),
                        Err(_) => (),
                    };
                }
                BuyDevelopmentCard { x, y } => {
                    let output = buy_development_card(x, y, &mut user, &mut board);
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
                    let result = select_two_same_tokens(c, &mut user, &mut board);
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

                    let result = select_three_tokens(c1, c2, c3, &mut user, &mut board);

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
                    let result = reserve_stack_card(l, &mut user, &mut board);
                    match result {
                        Ok(_) => action_rewards.push(ActionReward::new(command, 0.0)),
                        Err(_) => (),
                    };
                }
                BuyReservedCard(index) => {
                    let output = buy_reserved_card(index, &mut user, &mut board);
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

        let mut max_value = 0.0;
        let mut command = GameCommand::ReserveDevelopmentCard { x: 0, y: 0 };

        for e in action_rewards.iter() {
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
}
