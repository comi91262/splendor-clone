#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod card_stack;
pub mod color;
mod game;
pub mod jewelries;
pub mod jewelry_box;
pub mod level;
pub mod noble_tile;
pub mod token_stack;
pub mod user;

use crate::board::Board;
use crate::game::Game;
use crate::user::User;

use std::time::Instant;

fn main() {
    let mut game = Game::new();
    let mut board = Board::new();
    let mut user1 = User::new(1);
    let mut user2 = User::new(2);
    let mut user3 = User::new(3);
    let mut turn = 1;
    let mut max_duration = 0;
    let mut sum_duration = 0;
    let mut is_over = false;

    loop {
        let start = Instant::now();
        println!("{}手番目\n{}", turn, board);

        let command = game.look(1, &user1, &board);
        match game.eval(command, &mut user1, &mut board) {
            Ok(msg) => game.print(&msg, &user1),
            Err(error_msg) => is_over = true,
        }

        // let command = game.read();
        // match game.eval(command, &mut user2, &mut board) {
        //     Ok(msg) => game.print(&msg, &user2),
        //     Err(error_msg) => is_over = true,
        // }

        // let command = game.look(1, &user1, &board);
        // match game.eval(command, &mut user1, &mut board) {
        //     Ok(msg) => game.print(&msg, &user1),
        //     Err(error_msg) => is_over = true,
        // }

        let end = start.elapsed().subsec_nanos();
        if end > max_duration {
            max_duration = end;
        }
        sum_duration += end;

        if game.is_over(vec![&user1, &user2]) || is_over {
            break;
        }

        turn = turn + 1;
    }

    println!("\n\nゲーム終了:");
    println!("{}手番目\n{}", turn, board);
    game.print(&"", &user1);
    game.print(&"", &user2);
    println!("ターン経過最大: {}ns", max_duration);
    println!("ターン経過平均: {}ns", sum_duration / turn);
}
