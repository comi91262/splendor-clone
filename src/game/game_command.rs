use crate::game::board::Board;
use crate::game::color::Color;
use crate::game::color::Color::*;
use crate::game::jewelry_box::JEWELRIES;
use crate::game::level::Level;
use crate::game::level::Level::*;
use crate::game::user::User;

use std::fmt;

#[derive(Clone)]
pub enum GameCommand {
    ReserveDevelopmentCard { x: u8, y: u8 },
    BuyDevelopmentCard { x: u8, y: u8 },
    SelectTwoSameTokens(Color),
    SelectThreeTokens(Color, Color, Color),
    ReserveStackCard(Level),
    BuyReservedCard(u8),
}

impl fmt::Display for GameCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GameCommand::*;
        match self {
            ReserveDevelopmentCard { x, y } => write!(f, "カードを確保する({}, {})", x, y),
            BuyDevelopmentCard { x, y } => write!(f, "カードを購入する({}, {})", x, y),
            SelectTwoSameTokens(c) => write!(f, "同じ色のトークンを取得: {}", c),
            SelectThreeTokens(c1, c2, c3) => {
                write!(f, "違う色のトークンを取得: {} {} {}", c1, c2, c3)
            }
            ReserveStackCard(l) => write!(f, "スタックからカードを確保 {:?} ", l),
            BuyReservedCard(index) => {
                write!(f, "手札のカードを購入する: {}枚目", index)
            }
        }
    }
}

pub fn to_command(input: u8) -> GameCommand {
    use self::GameCommand::*;

    struct Point {
        x: u8,
        y: u8,
    };
    let coordinate: [Point; 12] = [
        Point { x: 0, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 0, y: 3 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: 2 },
        Point { x: 1, y: 3 },
        Point { x: 2, y: 0 },
        Point { x: 2, y: 1 },
        Point { x: 2, y: 2 },
        Point { x: 2, y: 3 },
    ];

    let color_set = [
        (Black, White, Red),
        (Black, White, Blue),
        (Black, White, Green),
        (Black, Red, Blue),
        (Black, Red, Green),
        (Black, Blue, Green),
        (White, Red, Blue),
        (White, Red, Green),
        (White, Blue, Green),
        (Red, Blue, Green),
    ];

    let level: [Level; 3] = [One, Two, Three];

    match input as usize {
        p @ 0...11 => ReserveDevelopmentCard {
            x: coordinate[p].x,
            y: coordinate[p].y,
        },
        p @ 12...23 => BuyDevelopmentCard {
            x: coordinate[p - 12].x,
            y: coordinate[p - 12].y,
        },
        c @ 24...28 => SelectTwoSameTokens(JEWELRIES[c - 24]),
        c @ 29...38 => SelectThreeTokens(
            color_set[c - 29].0,
            color_set[c - 29].1,
            color_set[c - 29].2,
        ),
        l @ 39...41 => ReserveStackCard(level[l - 39]),
        i @ 42...44 => BuyReservedCard((i - 42) as u8),
        _ => unreachable!(),
    }
}

pub fn reserve_development_card(
    x: u8,
    y: u8,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    if user.is_over_capacity_of_hand() {
        Err("試行: カードの確保, 結果: 手札がいっぱいです")
    } else {
        match board.get_card(x, y) {
                Some(card) => {
                    user.add_to_hands(card);
                    // 金トークンの取得
                    match board.get_token(Color::Gold) {
                    Some(token) => {
                        user.add_token(token);
                        Ok("試行: カードの確保, 結果: カードを確保しました")
                    }
                    None => Ok("試行: カードの確保, 結果: カードを確保しましたが、金トークンは取得できませんでした"),
                }
                }
                None => Err("試行: カードの確保, 結果: その場所にはもうカードがありません"),
            }
    }
}

pub fn buy_development_card(
    x: u8,
    y: u8,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    let is_available;
    match board.peek_card(x, y) {
        Some(card) => {
            is_available = card.is_available(&user);
        }
        None => {
            return Err(
                "試行: カードの購入, 結果: そこにはカードがありません",
            )
        }
    }

    if is_available {
        let card = board.uget_card(x, y);
        user.pay(&card, board.get_token_stack());
        user.obtain(card);
        let is_visited = visit(user, board);
        if is_visited {
            Ok("試行: 確保したカードの購入, 結果: カードを購入しました また、貴族の訪問がありました。")
        } else {
            Ok("試行: 確保したカードの購入, 結果: カードを購入しました")
        }
    } else {
        Err("試行: カードの購入, 結果: 必要な宝石数が足りません")
    }
}

pub fn select_two_same_tokens(
    color: Color,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    if !user.can_get_token() {
        return Err("試行: トークンを取得, 結果: トークンの所持数が10を超えるため取得できませんでした");
    }
    if board.can_get_token(color) {
        let token = board.uget_token(color);
        user.add_token(token);
        if !user.can_get_token() {
            return Ok("試行: トークンを取得, 結果: トークンを取得しました");
        }
        let token = board.uget_token(color);
        user.add_token(token);
        Ok("試行: トークンを取得, 結果: トークンを取得しました")
    } else {
        Err("試行: トークンを取得, 結果: 残りのトークン数が4より少ないです")
    }
}

pub fn select_three_tokens(
    color1: Color,
    color2: Color,
    color3: Color,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    // TODO !!
    if !user.can_get_token() {
        return Err("試行: トークンを取得, 結果: トークンの所持数が10を超えるため取得できませんでした");
    }

    let mut count = 0;
    if let Some(token) = board.get_token(color1) {
        user.add_token(token);
        count = count + 1;
    }

    if !user.can_get_token() {
        return Ok("試行: トークンを取得, 結果: トークンを取得しました");
    }
    if let Some(token) = board.get_token(color2) {
        user.add_token(token);
        count = count + 1;
    }

    if !user.can_get_token() {
        return Ok("試行: トークンを取得, 結果: トークンを取得しました");
    }
    if let Some(token) = board.get_token(color3) {
        user.add_token(token);
        count = count + 1;
    }

    if count == 0 {
        Err("試行: トークンを取得, 結果: 取得できるトークンがありません")
    } else {
        Ok("試行: トークンを取得, 結果: トークンを取得しました")
    }
}

pub fn reserve_stack_card(
    level: Level,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    if user.is_over_capacity_of_hand() {
        Err("試行: スタックされたカード取得, 結果: 手札がいっぱいです")
    } else {
        match board.get_stack_card(level) {
                Some(card) => {
                    user.add_to_hands(card);
                    match board.get_token(Color::Gold) {
                    Some(token) => {
                        user.add_token(token);
                        Ok("試行: スタックされたカード取得, 結果: カードを確保しました")
                    }
                    None => Ok("試行: スタックされたカード取得, 結果: カードを確保しましたが、金トークンは取得できませんでした"),
                }
                }
                None => Err("試行: スタックされたカード取得, 結果: 指定のスタックにカードはありませんでした"),
            }
    }
}

pub fn buy_reserved_card(
    order: u8,
    user: &mut User,
    board: &mut Board,
) -> Result<&'static str, &'static str> {
    let is_available;
    match user.peek_card_in_hands(order) {
            Some(card) => is_available = card.is_available(&user),
            None => return Err("試行: 確保したカードの購入, 結果: そこにはカードがありません"),
        }
    if is_available {
        let card = user.uget_card_in_hands(order);
        user.pay(&card, board.get_token_stack());
        user.obtain(card);
        user.remove_card_in_hands(order);
        let is_visited = visit(user, board);
        if is_visited {
            Ok("試行: 確保したカードの購入, 結果: カードを購入しました また、貴族の訪問がありました。")
        } else {
            Ok("試行: 確保したカードの購入, 結果: カードを購入しました")
        }
    } else {
        Err("試行: 確保したカードの購入, 結果: 必要な宝石数が足りません")
    }
}

fn visit(user: &mut User, board: &mut Board) -> bool {
    let mut remove_tile_order = vec![];
    let mut order = 0;
    let jewelies = user.get_jewelries();

    for tile in board.get_noble_tile().iter_mut() {
        if tile.can_visit(&jewelies) {
            user.add_vp(tile.get_point());
            remove_tile_order.push(order);
        }
        order += 1;
    }

    let mut result = false;
    for order in remove_tile_order.into_iter().rev() {
        board.get_noble_tile().remove(order as usize);
        result = true;
    }
    result
}
