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

fn hoge_obtain(x: u8, y: u8, user: &mut User, board: &mut Board) -> String {
    if user.is_over_capacity_of_hand() {
        String::from("手札がいっぱいです")
    } else {
        match board.get_card(x, y) {
            Some(card) => {
                user.add_to_hands(card);
                match board.get_token(Color::Gold) {
                    Some(token) => user.add_token(token),
                    None => (),
                }
            }
            None => (),
        }
        String::from("OK")
    }
}
fn hoge_pickup(x: u8, y: u8, user: &mut User, board: &mut Board) -> String {
    String::from("pickup")
}

fn hoge_pickup_token(color: Color, user: &mut User, board: &mut Board) -> String {
    String::from("pickup2")
}

fn hoge_pickup_token2(
    color1: Color,
    color2: Color,
    color3: Color,
    user: &mut User,
    board: &mut Board,
) -> String {
    String::from("pickup3")
}
fn eval(s: &str, user: &mut User, board: &mut Board) -> String {
    let output = match s {
        "1" => hoge_obtain(0, 0, user, board),
        "2" => hoge_obtain(0, 1, user, board),
        "3" => hoge_obtain(0, 2, user, board),
        "4" => hoge_obtain(0, 3, user, board),
        "5" => hoge_obtain(1, 0, user, board),
        "6" => hoge_obtain(1, 1, user, board),
        "7" => hoge_obtain(1, 2, user, board),
        "8" => hoge_obtain(1, 3, user, board),
        "9" => hoge_obtain(2, 0, user, board),
        "10" => hoge_obtain(2, 1, user, board),
        "11" => hoge_obtain(2, 2, user, board),
        "12" => hoge_obtain(2, 3, user, board),
        "13" => hoge_pickup(0, 0, user, board),
        "14" => hoge_pickup(0, 1, user, board),
        "15" => hoge_pickup(0, 2, user, board),
        "16" => hoge_pickup(0, 3, user, board),
        "17" => hoge_pickup(1, 0, user, board),
        "18" => hoge_pickup(1, 1, user, board),
        "19" => hoge_pickup(1, 2, user, board),
        "20" => hoge_pickup(1, 3, user, board),
        "21" => hoge_pickup(2, 0, user, board),
        "22" => hoge_pickup(2, 1, user, board),
        "23" => hoge_pickup(2, 2, user, board),
        "24" => hoge_pickup(2, 3, user, board),
        "25" => hoge_pickup_token(Color::White, user, board),
        "26" => hoge_pickup_token(Color::Black, user, board),
        "27" => hoge_pickup_token(Color::Red, user, board),
        "28" => hoge_pickup_token(Color::Blue, user, board),
        "29" => hoge_pickup_token(Color::Green, user, board),
        "30" => hoge_pickup_token2(Color::Black, Color::White, Color::Red, user, board),
        "31" => hoge_pickup_token2(Color::Black, Color::White, Color::Blue, user, board),
        "32" => hoge_pickup_token2(Color::Black, Color::White, Color::Green, user, board),
        "33" => hoge_pickup_token2(Color::Black, Color::Red, Color::Blue, user, board),
        "34" => hoge_pickup_token2(Color::Black, Color::Red, Color::Green, user, board),
        "35" => hoge_pickup_token2(Color::Black, Color::Blue, Color::Green, user, board),
        "36" => hoge_pickup_token2(Color::White, Color::Red, Color::Blue, user, board),
        "37" => hoge_pickup_token2(Color::White, Color::Red, Color::Green, user, board),
        "38" => hoge_pickup_token2(Color::White, Color::Blue, Color::Green, user, board),
        "39" => hoge_pickup_token2(Color::Red, Color::Blue, Color::Green, user, board),
        _ => String::from(""),
    };

    output
}

fn print(result: &str, user: &User) -> () {
    println!("結果: {}", result);
    println!("ユーザーステータス: {:?}", user);
}
fn main() {
    let mut board: Board = Default::default();
    board.create();

    board.drop_card(2, 0);
    board.drop_card(2, 1);
    board.drop_card(2, 2);
    board.drop_card(2, 3);

    // init user
    let mut user: User = Default::default();
    user.create();

    // if
    // let tokens = user.get_tokens();
    // let card = board.get_card(2, 0);
    // card.is_available(tokens.0, tokens.1, tokens.2, tokens.3, tokens.4);

    // user.obtain(card.clone());
    // board.drop_card(2, 0);

    loop {
        println!("{}", GUIDE.to_string());
        let command: String = read();
        let result = eval(&command, &mut user, &mut board);
        print(&result, &user);
    }
}

// - カードの購入
//     - Cost < 手持ちのトークン+カードのColar
//
// - トークンの確保
//     - 違う色3枚
//     - 残りトークンが0枚のトークンからは取得出来ない
//     - 4枚以上のトークンから2枚取得
