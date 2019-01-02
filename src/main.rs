extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

mod board;
pub mod card;
pub mod color;
pub mod token;
mod user;

use crate::board::Board;
use crate::color::Color;
use crate::user::User;

lazy_static! {
    static ref CREATED_FILE_PATH: String = {
        let working_directory = env!("CARGO_MANIFEST_DIR");
        let file_path = "card.json";
        format!("{}/{}", working_directory, file_path)
    };
}

const GUIDE: &'static str = "
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
";

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn hoge_obtain(x: u8, y: u8) -> String {
    String::from("obtain")
}
fn hoge_pickup(x: u8, y: u8) -> String {
    String::from("pickup")
}

fn hoge_pickup_token(color: Color) -> String {
    String::from("pickup2")
}

fn hoge_pickup_token2(color1: Color, color2: Color, color3: Color) -> String {
    String::from("pickup3")
}
fn eval(s: &str) -> String {
    let output = match s {
        "1" => hoge_obtain(0, 0),
        "2" => hoge_obtain(0, 1),
        "3" => hoge_obtain(0, 2),
        "4" => hoge_obtain(0, 3),
        "5" => hoge_obtain(1, 0),
        "6" => hoge_obtain(1, 1),
        "7" => hoge_obtain(1, 2),
        "8" => hoge_obtain(1, 3),
        "9" => hoge_obtain(2, 0),
        "10" => hoge_obtain(2, 1),
        "11" => hoge_obtain(2, 2),
        "12" => hoge_obtain(2, 3),
        "13" => hoge_pickup(0, 0),
        "14" => hoge_pickup(0, 1),
        "15" => hoge_pickup(0, 2),
        "16" => hoge_pickup(0, 3),
        "17" => hoge_pickup(1, 0),
        "18" => hoge_pickup(1, 1),
        "19" => hoge_pickup(1, 2),
        "20" => hoge_pickup(1, 3),
        "21" => hoge_pickup(2, 0),
        "22" => hoge_pickup(2, 1),
        "23" => hoge_pickup(2, 2),
        "24" => hoge_pickup(2, 3),
        "25" => hoge_pickup_token(Color::White),
        "26" => hoge_pickup_token(Color::Black),
        "27" => hoge_pickup_token(Color::Red),
        "28" => hoge_pickup_token(Color::Blue),
        "29" => hoge_pickup_token(Color::Green),
        "30" => hoge_pickup_token2(Color::Black, Color::White, Color::Red),
        "31" => hoge_pickup_token2(Color::Black, Color::White, Color::Blue),
        "32" => hoge_pickup_token2(Color::Black, Color::White, Color::Green),
        "33" => hoge_pickup_token2(Color::Black, Color::Red, Color::Blue),
        "34" => hoge_pickup_token2(Color::Black, Color::Red, Color::Green),
        "35" => hoge_pickup_token2(Color::Black, Color::Blue, Color::Green),
        "36" => hoge_pickup_token2(Color::White, Color::Red, Color::Blue),
        "37" => hoge_pickup_token2(Color::White, Color::Red, Color::Green),
        "38" => hoge_pickup_token2(Color::White, Color::Blue, Color::Green),
        "39" => hoge_pickup_token2(Color::Red, Color::Blue, Color::Green),
        _ => String::from(""),
    };

    output
}

fn print() -> () {
    println!("{}", GUIDE.to_string());
}
fn main() {
    let mut board: Board = Default::default();
    board.create();

    // let mut black_token_stack = vec![];

    // black_token_stack.push(Token {
    //     color: Color::Black,
    // });
    // black_token_stack.push(Token {
    //     color: Color::Black,
    // });
    // black_token_stack.push(Token {
    //     color: Color::Black,
    // });
    // black_token_stack.push(Token {
    //     color: Color::Black,
    // });
    // black_token_stack.push(Token {
    //     color: Color::Black,
    // });

    board.drop_card(2, 0);
    board.drop_card(2, 1);
    board.drop_card(2, 2);

    // init user
    let mut user: User = Default::default();

    // if
    let tokens = user.get_tokens();
    let card = board.get_card(2, 0);
    card.is_available(tokens.0, tokens.1, tokens.2, tokens.3, tokens.4);

    user.obtain(card.clone());
    board.drop_card(2, 0);

    //println!("{}", GUIDE.to_string());

    // トークンの確保

    loop {
        let command: String = read();
        let result = eval(&command);
        print();
    }
}

// - カードの確保
//     - ３枚まで
//     - 黄金トークンを取得
//          - 5枚まで
//
// - カードの購入
//     - Cost < 手持ちのトークン+カードのColar
//
// - トークンの確保
//     - 違う色3枚
//     - 残りトークンが0枚のトークンからは取得出来ない
//     - 4枚以上のトークンから2枚取得
