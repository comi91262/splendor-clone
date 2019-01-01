// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};

use ndarray::Array2;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use std::fs::File;
// use std::io::Write;

use std::io::{BufRead, BufReader};

lazy_static! {
    static ref CREATED_FILE_PATH: String = {
        let working_directory = env!("CARGO_MANIFEST_DIR");
        let file_path = "card.json";
        format!("{}/{}", working_directory, file_path)
    };
}

#[derive(Debug)]
struct User {
    id: u8,
    hand: Vec<Card>,
    aquired_card: Vec<Card>
    vp: u8,
    black_token: u8,
    white_token: u8,
    red_token: u8,
    blue_token: u8,
    green_token: u8,
    gold_token: u8,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            vp: 0,
            hand: vec![],
            black_token: 0,
            white_token: 0,
            red_token: 0,
            blue_token: 0,
            green_token: 0,
            gold_token: 0,
        }
    }
}

impl User {
    fn _get_vp(&self) -> u8 {
        self.vp
    }
    fn _get_id(&self) -> u8 {
        self.id
    }
    fn _set_vp(&mut self, vp: u8) {
        self.vp = vp
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Color {
    Black,
    White,
    Red,
    Blue,
    Green,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
    level: u8,
    color: Color,
    point: u8,
    cost_black: u8,
    cost_white: u8,
    cost_red: u8,
    cost_blue: u8,
    cost_green: u8,
}


impl Default for Card {
    fn default() -> Self {
        Card {
            level: 0,
            color: Color::Black,
            point: 0,
            cost_black: 0,
            cost_white: 0,
            cost_red: 0,
            cost_blue: 0,
            cost_green: 0,
        }
    }
}

fn check(card: &Card, user: &User) -> bool {
    if card.cost_black != user.black_token {
        return false;
    }

    if card.cost_white != user.white_token {
        return false;
    }

    if card.cost_red != user.red_token {
        return false;
    }

    if card.cost_blue != user.blue_token {
        return false;
    }

    if card.cost_green != user.green_token {
        return false;
    }

    return true;
}


struct Token {
    color: Color,
}

const GUIDE: &'static str = "
1). カードの確保
2). カードの購入
3). トークンの確保
";

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn eval(s: &str) -> () {
    match s {
        "1" => println!("{}", "aaaa"),
        "3" => println!("{}", "aaaa"),
        _ => println!("{}", "bbb"),
    }
}

fn print() -> () {
    println!("{}", GUIDE.to_string());
}

fn main() {
    let mut board = Array2::<Card>::default((3, 4));
    let mut level1_stack = vec![];
    let mut black_token_stack =  vec![];

    black_token_stack.push(Token{color: Color::Black});
    black_token_stack.push(Token{color: Color::Black});
    black_token_stack.push(Token{color: Color::Black});
    black_token_stack.push(Token{color: Color::Black});
    black_token_stack.push(Token{color: Color::Black});

    for result in BufReader::new(File::open("card.json").unwrap()).lines() {
        let l = result.unwrap();
        let card: Card = serde_json::from_str(&l).unwrap();

        match card {
            Card { level: 1, .. } => level1_stack.push(card),
            Card { level: _, .. } => (),
        }
    }

    board[[2, 0]] = level1_stack.pop().unwrap();
    board[[2, 1]] = level1_stack.pop().unwrap();
    board[[2, 2]] = level1_stack.pop().unwrap();

    // init user
    let mut u = User {
        id: 1,
        ..Default::default()
    };

    // if
    let _ = check(&board[[2, 0]], &u);
    let card = board[[2, 0]].clone();

    u.hand.push(card);


    board[[2, 0]] = level1_stack.pop().unwrap();

    //println!("{}", GUIDE.to_string());

    // トークンの確保
    

    //loop {
    //    let command: String = read();
    //    let result = eval(&command);
    //    print();
    //}

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
