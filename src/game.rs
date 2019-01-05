use crate::board::Board;
use crate::color::Color;
use crate::level::Level;
use crate::user::User;

use rand::rngs::ThreadRng;
use rand::Rng;

pub fn read(rng: &mut ThreadRng) -> u8 {
    rng.gen::<u8>() % 42 + 1
}

pub fn eval(input: u8, user: &mut User, board: &mut Board, rng: &mut ThreadRng) -> String {
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
        _ => unreachable!(),
    };

    match output {
        Ok(result) => {
            return result.to_string();
        }
        Err(error_msg) => {
            println!("{}", error_msg);
            let input = read(rng);
            eval(input, user, board, rng)
        }
    }
}

pub fn print(result: &str, user: &User) -> () {
    println!("結果: {}", result);
    println!("ユーザーステータス: {}", user);
}

pub fn is_over(user: &User) -> bool {
    // TODO magic number
    user.get_vp() >= 15
}

fn reserve_development_card(
    x: u8,
    y: u8,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    println!("カードの確保");
    if user.is_over_capacity_of_hand() {
        Err("手札がいっぱいです")
    } else {
        match board.get_card(x, y) {
            Some(card) => {
                user.add_to_hands(card);
                // 金トークンの取得
                match board.get_token(Color::Gold) {
                    Some(token) => {
                        user.add_token(token);
                        Ok("カードを確保しました")
                    }
                    None => Ok("カードを確保しましたが、金トークンは取得できませんでした"),
                }
            }
            None => Err("その場所にはもうカードがありません"),
        }
    }
}

fn buy_development_card(
    x: u8,
    y: u8,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    println!("手札の購入");
    match board.peek_card(x, y) {
        Some(card) => {
            if card.is_available(&user) {
                user.pay(&card);
                let card = board.uget_card(x, y);
                user.obtain(card);
                Ok("手札を購入しました")
            } else {
                Err("必要な宝石数が足りません")
            }
        }
        None => Err("そこにはカードがありません"),
    }
}

fn select_two_same_tokens(
    color: Color,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    println!("トークンを取得");
    if board.can_get_token(color) {
        for _ in 0..2 {
            let token = board.uget_token(color);
            user.add_token(token);
        }
        Ok("トークンを取得しました")
    } else {
        Err("残りのトークン数が4より少ないです")
    }
}

fn select_three_tokens(
    color1: Color,
    color2: Color,
    color3: Color,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    println!("トークンを取得");
    match board.get_token(color1) {
        Some(token) => user.add_token(token),
        None => (),
    }
    match board.get_token(color2) {
        Some(token) => user.add_token(token),
        None => (),
    }
    match board.get_token(color3) {
        Some(token) => user.add_token(token),
        None => (),
    }
    Ok("トークンを取得しました")
}

fn reserve_stack_card(
    level: Level,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    println!("スタックされたカード取得");
    if user.is_over_capacity_of_hand() {
        Err("手札がいっぱいです")
    } else {
        match board.get_stack_card(level) {
            Some(card) => {
                user.add_to_hands(card);
                match board.get_token(Color::Gold) {
                    Some(token) => {
                        user.add_token(token);
                        Ok("カードを確保しました")
                    }
                    None => Ok("カードを確保しましたが、金トークンは取得できませんでした"),
                }
            }
            None => Err("指定のスタックにカードはありませんでした"),
        }
    }
}
