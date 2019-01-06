#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod card;
pub mod color;
mod game;
pub mod level;
pub mod noble_tile;
pub mod token;
pub mod user;

use crate::board::Board;
use crate::game::*;
use crate::user::User;

use std::process;

fn main() {
    let mut board = Board::create();
    let mut user = User::create();
    let mut rng = rand::thread_rng();
    let mut turn = 1;

    loop {
        println!("{}手番目\n{}", turn, board);
        let command = game::read(&mut rng);
        let result = game::eval(command, &mut user, &mut board, &mut rng);

        game::visit(&mut user, &mut board);
        if is_over(&user) {
            game::print(&"end", &user);
            process::exit(1);
        }
        game::print(&result, &user);

        turn = turn + 1;
    }
}
