use crate::board::Board;
use crate::color::Color;
use crate::level::Level;
use crate::user::User;

pub fn reserve_development_card(x: u8, y: u8, user: &mut User, board: &mut Board) -> String {
    println!("手札の確保");
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
pub fn buy_development_card(x: u8, y: u8, user: &mut User, board: &mut Board) -> String {
    println!("手札の購入");
    match board.peek_card(x, y) {
        Some(card) => {
            if card.is_available(&user) {
                user.pay(&card);
                let card = board.uget_card(x, y);
                user.obtain(card);
                String::from("OK")
            } else {
                String::from("トークンが足りません")
            }
        }
        None => String::from("そこにはカードがありません"),
    }
}

pub fn select_two_same_tokens(color: Color, user: &mut User, board: &mut Board) -> String {
    println!("トークンを取得");
    if board.can_get_token(color) {
        for _ in 0..2 {
            let token = board.uget_token(color);
            user.add_token(token);
        }
    }
    String::from("OK")
}

pub fn select_three_tokens(
    color1: Color,
    color2: Color,
    color3: Color,
    user: &mut User,
    board: &mut Board,
) -> String {
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
    String::from("OK")
}

pub fn reserve_stack_card(level: Level, user: &mut User, board: &mut Board) -> String {
    println!("スタックされたカード取得");
    if user.is_over_capacity_of_hand() {
        String::from("手札がいっぱいです")
    } else {
        match board.get_stack_card(level) {
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
