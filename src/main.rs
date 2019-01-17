#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod card;
pub mod color;
mod game;
pub mod jewelry_box;
pub mod jewelries;
pub mod level;
pub mod noble_tile;
pub mod token;
pub mod user;

use crate::board::Board;
use crate::game::Game;
use crate::user::User;

use std::process;

fn main() {
    let mut game = Game::new();
    let mut board = Board::new();
    let mut user = User::new();
    let mut turn = 1;

    loop {
        println!("{}手番目\n{}", turn, board);

        game.look(1, &user, &board);
        let command = game.read();
        let result = game.eval(command, &mut user, &mut board);

        game.visit(&mut user, &mut board);
        if game.is_over(&user) {
            game.print(&"ゲーム終了しました", &user);
            process::exit(1);
        }
        game.print(&result, &user);

        turn = turn + 1;
    }
}
