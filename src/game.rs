use self::action_reward_table::ActionRewardList;
use self::board::Board;
use self::game_command::GameCommand;
use self::user::User;

use std::time::Instant;

pub mod action_reward_table;
pub mod board;
pub mod card_stack;
pub mod color;
mod game_command;
pub mod jewelry_box;
pub mod level;
pub mod noble_tile;
pub mod token_stack;
pub mod user;

const VP_TO_END: u8 = 15;
const MAX_NUMBER_OF_TRIALS: u8 = 100;

pub struct Game {
    board: Board,
    users: Vec<User>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            users: vec![User::new(0), User::new(1)],
        }
    }
    pub fn play(&mut self) {
        let mut turn = 1;
        let mut max_duration = 0;
        let mut sum_duration = 0;
        let mut is_over = false;

        let _ = ActionRewardList::look(1, self.users[0], self.board);
        loop {
            let start = Instant::now();
            println!("{}手番目\n{}", turn, self.board);

            for user in self.users.iter_mut() {
                let command = user.read();
                match Game::eval(command, user, &mut self.board) {
                    Ok(msg) => Game::print(&msg, &user),
                    Err(error_msg) => is_over = true,
                }
            }

            let end = start.elapsed().subsec_nanos();
            if end > max_duration {
                max_duration = end;
            }
            sum_duration += end;

            if Game::is_over(&self.users) || is_over {
                break;
            }

            turn = turn + 1;
        }

        println!("\n\nゲーム終了:");
        println!("{}手番目\n{}", turn, self.board);
        for user in self.users.iter() {
            Game::print(&"", &user);
        }
        println!("ターン経過最大: {}ns", max_duration);
        println!("ターン経過平均: {}ns", sum_duration / turn);
    }

    pub fn eval(input: GameCommand, user: &mut User, board: &mut Board) -> Result<String, String> {
        let mut input = input;
        for _ in 0..MAX_NUMBER_OF_TRIALS {
            match Game::eval_by_selection(input, user, board) {
                Ok(result) => {
                    return Ok(result.to_string());
                }
                Err(error_msg) => {
                    println!("{}", error_msg);
                    input = user.read();
                    continue;
                }
            }
        }

        Err("降参を選ばれました".to_string())
    }

    fn eval_by_selection(
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

    pub fn print(result: &str, user: &User) -> () {
        println!("{}", result);
        println!("ユーザーステータス: {}", user);
    }

    pub fn is_over(users: &Vec<User>) -> bool {
        let mut result = false;
        for user in users.iter() {
            if user.get_vp() >= VP_TO_END {
                result = true;
                println!("プレイヤー{}が勝利しました", user.get_id());
            }
        }
        result
    }
}
