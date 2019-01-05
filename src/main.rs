extern crate serde;
extern crate serde_json;
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
use crate::color::Color;
use crate::level::Level;
use crate::user::User;
use rand::Rng;
use std::process;

const _GUIDE: &'static str = "
1). 0, 0 カードの確保 2). 0, 1 カードの確保 3). 0, 2 カードの確保 4). 0, 3 カードの確保
5). 1, 0 カードの確保 6). 1, 1 カードの確保 7). 1, 2 カードの確保 8). 1, 3 カードの確保
9). 2, 0 カードの確保 10). 2, 1 カードの確保 11). 2, 2 カードの確保 12). 2, 3 カードの確保

13). 0, 0 カードの購入 14). 0, 1 カードの購入 15). 0, 2 カードの購入 16). 0, 3 カードの購入
17). 1, 0 カードの購入 18). 1, 1 カードの購入 19). 1, 2 カードの購入 20). 1, 3 カードの購入
21). 2, 0 カードの購入 22). 2, 1 カードの購入 23). 2, 2 カードの購入 24). 2, 3 カードの購入

25). 黒トークン3枚獲得 26). 白トークン3枚獲得 27). 赤トークン3枚獲得
28). 青トークン3枚獲得 29). 緑トークン3枚獲得

30). 黒,白,赤トークン獲得 31). 黒,白,青トークン獲得 32). 黒,白,緑トークン獲得 33). 黒,赤,青トークン獲得
34). 黒,赤,緑トークン獲得 35). 黒,青,緑トークン獲得 36). 白,赤,青トークン獲得 37). 白,赤,緑トークン獲得
38). 白,青,緑トークン獲得 39). 赤,青,緑トークン獲得

40). レベル1 カードの確保 41). レベル2 カードの確保 42). レベル3 カードの確保
";

fn _read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn eval(input: u8, user: &mut User, board: &mut Board) -> String {
    use crate::game::*;

    let output = match input {
        1 => reserve_development_card(0, 0, user, board),
        2 => reserve_development_card(0, 1, user, board),
        3 => reserve_development_card(0, 2, user, board),
        4 => reserve_development_card(0, 3, user, board),
        5 => reserve_development_card(1, 0, user, board),
        6 => reserve_development_card(1, 1, user, board),
        7 => reserve_development_card(1, 2, user, board),
        8 => reserve_development_card(1, 3, user, board),
        9 => reserve_development_card(2, 0, user, board),
        10 => reserve_development_card(2, 1, user, board),
        11 => reserve_development_card(2, 2, user, board),
        12 => reserve_development_card(2, 3, user, board),
        13 => buy_development_card(0, 0, user, board),
        14 => buy_development_card(0, 1, user, board),
        15 => buy_development_card(0, 2, user, board),
        16 => buy_development_card(0, 3, user, board),
        17 => buy_development_card(1, 0, user, board),
        18 => buy_development_card(1, 1, user, board),
        19 => buy_development_card(1, 2, user, board),
        20 => buy_development_card(1, 3, user, board),
        21 => buy_development_card(2, 0, user, board),
        22 => buy_development_card(2, 1, user, board),
        23 => buy_development_card(2, 2, user, board),
        24 => buy_development_card(2, 3, user, board),
        25 => select_two_same_tokens(Color::White, user, board),
        26 => select_two_same_tokens(Color::Black, user, board),
        27 => select_two_same_tokens(Color::Red, user, board),
        28 => select_two_same_tokens(Color::Blue, user, board),
        29 => select_two_same_tokens(Color::Green, user, board),
        30 => select_three_tokens(Color::Black, Color::White, Color::Red, user, board),
        31 => select_three_tokens(Color::Black, Color::White, Color::Blue, user, board),
        32 => select_three_tokens(Color::Black, Color::White, Color::Green, user, board),
        33 => select_three_tokens(Color::Black, Color::Red, Color::Blue, user, board),
        34 => select_three_tokens(Color::Black, Color::Red, Color::Green, user, board),
        35 => select_three_tokens(Color::Black, Color::Blue, Color::Green, user, board),
        36 => select_three_tokens(Color::White, Color::Red, Color::Blue, user, board),
        37 => select_three_tokens(Color::White, Color::Red, Color::Green, user, board),
        38 => select_three_tokens(Color::White, Color::Blue, Color::Green, user, board),
        39 => select_three_tokens(Color::Red, Color::Blue, Color::Green, user, board),
        40 => reserve_stack_card(Level::One, user, board),
        41 => reserve_stack_card(Level::Two, user, board),
        42 => reserve_stack_card(Level::Three, user, board),
        _ => String::from(""),
    };

    output
}

fn print(result: &str, user: &User) -> () {
    println!("結果: {}", result);
    println!("ユーザーステータス: {}", user);
}

fn is_over(user: &User) -> bool {
    // TODO magic number
    user.get_vp() >= 15
}

fn main() {
    let mut board = Board::create();
    let mut user = User::create();

    let mut rng = rand::thread_rng();

    for _ in 0..2 {
        println!("{}", board);
        let command = rng.gen::<u8>() % 42 + 1;
        let result = eval(command, &mut user, &mut board);
        if is_over(&user) {
            print(&"end", &user);
            process::exit(1);
        }
        print(&result, &user);
    }
}
