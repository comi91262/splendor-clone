#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod card;
pub mod color;
mod game;
pub mod jewelries;
pub mod jewelry_box;
pub mod level;
pub mod noble_tile;
pub mod token;
pub mod user;

use crate::board::Board;
use crate::game::Game;
use crate::user::User;

fn main() {
    let mut game = Game::new();
    let mut board = Board::new();
    let mut user1 = User::new(1);
    let mut user2 = User::new(2);
    let mut turn = 1;

    loop {
        println!("{}手番目\n{}", turn, board);

        let command = game.look(1, &user1, &board);
        let result = game.eval(command, &mut user1, &mut board);
        game.visit(&mut user1, &mut board);
        game.is_over(&user1);
        game.print(&result, &user1);

        let command = game.read();
        let result = game.eval(command, &mut user2, &mut board);
        game.visit(&mut user2, &mut board);
        game.is_over(&user2);
        game.print(&result, &user2);

        turn = turn + 1;
    }
}
