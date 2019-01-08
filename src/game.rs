use crate::board::Board;
use crate::color::Color;
use crate::level::Level;
use crate::user::User;

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Game {
    rng: ThreadRng,
}

// trait Repl {
//     fn read(&mut self) -> u8;
//     fn eval(&mut self, input: u8, user: &mut User, board: &mut Board) -> String;
//     fn print(output: &str, result: &user);
// }

impl Game {
    pub fn create() -> Game {
        Game {
            rng: rand::thread_rng(),
        }
    }
    pub fn read(&mut self) -> u8 {
        self.rng.gen::<u8>() % 45 + 1
    }

    pub fn eval(&mut self, input: u8, user: &mut User, board: &mut Board) -> String {
        let output = self.eval_by_selection(input, user, board);

        match output {
            Ok(result) => {
                return result.to_string();
            }
            Err(error_msg) => {
                println!("結果: {}", error_msg);
                let input = self.read();
                self.eval(input, user, board)
            }
        }
    }

    fn eval_by_selection(
        &self,
        input: u8,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        match input {
            1 => self.reserve_development_card(0, 0, user, board),
            2 => self.reserve_development_card(0, 1, user, board),
            3 => self.reserve_development_card(0, 2, user, board),
            4 => self.reserve_development_card(0, 3, user, board),
            5 => self.reserve_development_card(1, 0, user, board),
            6 => self.reserve_development_card(1, 1, user, board),
            7 => self.reserve_development_card(1, 2, user, board),
            8 => self.reserve_development_card(1, 3, user, board),
            9 => self.reserve_development_card(2, 0, user, board),
            10 => self.reserve_development_card(2, 1, user, board),
            11 => self.reserve_development_card(2, 2, user, board),
            12 => self.reserve_development_card(2, 3, user, board),
            13 => self.buy_development_card(0, 0, user, board),
            14 => self.buy_development_card(0, 1, user, board),
            15 => self.buy_development_card(0, 2, user, board),
            16 => self.buy_development_card(0, 3, user, board),
            17 => self.buy_development_card(1, 0, user, board),
            18 => self.buy_development_card(1, 1, user, board),
            19 => self.buy_development_card(1, 2, user, board),
            20 => self.buy_development_card(1, 3, user, board),
            21 => self.buy_development_card(2, 0, user, board),
            22 => self.buy_development_card(2, 1, user, board),
            23 => self.buy_development_card(2, 2, user, board),
            24 => self.buy_development_card(2, 3, user, board),
            25 => self.select_two_same_tokens(Color::White, user, board),
            26 => self.select_two_same_tokens(Color::Black, user, board),
            27 => self.select_two_same_tokens(Color::Red, user, board),
            28 => self.select_two_same_tokens(Color::Blue, user, board),
            29 => self.select_two_same_tokens(Color::Green, user, board),
            30 => self.select_three_tokens(Color::Black, Color::White, Color::Red, user, board),
            31 => self.select_three_tokens(Color::Black, Color::White, Color::Blue, user, board),
            32 => self.select_three_tokens(Color::Black, Color::White, Color::Green, user, board),
            33 => self.select_three_tokens(Color::Black, Color::Red, Color::Blue, user, board),
            34 => self.select_three_tokens(Color::Black, Color::Red, Color::Green, user, board),
            35 => self.select_three_tokens(Color::Black, Color::Blue, Color::Green, user, board),
            36 => self.select_three_tokens(Color::White, Color::Red, Color::Blue, user, board),
            37 => self.select_three_tokens(Color::White, Color::Red, Color::Green, user, board),
            38 => self.select_three_tokens(Color::White, Color::Blue, Color::Green, user, board),
            39 => self.select_three_tokens(Color::Red, Color::Blue, Color::Green, user, board),
            40 => self.reserve_stack_card(Level::One, user, board),
            41 => self.reserve_stack_card(Level::Two, user, board),
            42 => self.reserve_stack_card(Level::Three, user, board),
            43 => self.buy_reserved_card(0, user, board),
            44 => self.buy_reserved_card(1, user, board),
            45 => self.buy_reserved_card(2, user, board),
            _ => unreachable!(),
        }
    }

    pub fn print(&self, result: &str, user: &User) -> () {
        println!("結果: {}", result);
        println!("ユーザーステータス: {}", user);
    }

    pub fn is_over(&self, user: &User) -> bool {
        // TODO magic number
        user.get_vp() >= 15
    }

    fn reserve_development_card(
        &self,
        x: u8,
        y: u8,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        println!("試行: カードの確保");
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
        &self,
        x: u8,
        y: u8,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        println!("試行: カードの購入");
        let is_available;
        match board.peek_card(x, y) {
            Some(card) => {
                is_available = card.is_available(&user);
            }
            None => return Err("そこにはカードがありません"),
        }

        if is_available {
            let card = board.uget_card(x, y);
            user.pay(&card, board.get_token_stack());
            user.obtain(card);
            Ok("カードを購入しました")
        } else {
            Err("必要な宝石数が足りません")
        }
    }

    fn select_two_same_tokens(
        &self,
        color: Color,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        println!("試行: トークンを取得");
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
        &self,
        color1: Color,
        color2: Color,
        color3: Color,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        println!("試行: トークンを取得");
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
        &self,
        level: Level,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        println!("試行: スタックされたカード取得");
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

    pub fn buy_reserved_card(
        &self,
        order: u8,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        println!("試行: 確保したカードの購入");

        let is_available;
        match user.peek_card_in_hands(order) {
            Some(card) => is_available = card.is_available(&user),
            None => return Err("そこにはカードがありません"),
        }
        if is_available {
            let card = user.uget_card_in_hands(order);
            user.pay(&card, board.get_token_stack());
            user.obtain(card);
            user.remove_card_in_hands(order);
            Ok("カードを購入しました")
        } else {
            Err("必要な宝石数が足りません")
        }
    }

    pub fn visit(&self, user: &mut User, board: &mut Board) {
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

        for order in remove_tile_order.into_iter().rev() {
            println!("貴族の訪問がありました。");
            board.get_noble_tile().remove(order as usize);
        }
    }

    pub fn look(&self, step: u8, user: &User, board: &Board) -> u8 {
        for input in 1..13 {
            let mut user = user.clone();
            let mut board = board.clone();
            let result = self.eval_by_selection(input, &mut user, &mut board);
            println!("{:?}", result);
        }

        1
    }
}
