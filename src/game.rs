use crate::board::Board;
use crate::color::Color;
use crate::jewelry_box::JewelryBox;
use crate::level::Level;
use crate::user::User;

use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;

pub struct Game {
    rng: ThreadRng,
    color_value: HashMap<Color, f32>,
}

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
pub struct ActionReward {
    pub action: GameCommand,
    pub reward: f32,
}

impl ActionReward {
    fn new(action: GameCommand, reward: f32) -> ActionReward {
        ActionReward {
            action: action,
            reward: reward,
        }
    }
}

impl fmt::Debug for ActionReward {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action: {} 報酬点: {}", self.action, self.reward)
    }
}

// trait Repl {
//     fn read(&mut self) -> u8;
//     fn eval(&mut self, input: u8, user: &mut User, board: &mut Board) -> String;
//     fn print(output: &str, result: &user);
// }

impl Game {
    pub fn new() -> Game {
        let mut color_value = HashMap::new();
        color_value.insert(Color::Black, 0.0);
        color_value.insert(Color::White, 0.0);
        color_value.insert(Color::Red, 0.0);
        color_value.insert(Color::Blue, 0.0);
        color_value.insert(Color::Green, 0.0);
        color_value.insert(Color::Gold, 0.0);

        Game {
            rng: rand::thread_rng(),
            color_value: color_value,
        }
    }
    pub fn read(&mut self) -> GameCommand {
        let random_value = self.rng.gen::<u8>() % 45;
        self.to_command(random_value)
    }

    pub fn eval(&mut self, input: GameCommand, user: &mut User, board: &mut Board) -> String {
        let output = self.eval_by_selection(input, user, board);

        match output {
            Ok(result) => {
                return result.to_string();
            }
            Err(error_msg) => {
                println!("{}", error_msg);
                let input = self.read();
                self.eval(input, user, board)
            }
        }
    }

    fn to_command(&self, input: u8) -> GameCommand {
        use self::GameCommand::*;
        use crate::color::Color::*;
        use crate::level::Level::*;

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
        let color: [Color; 5] = [Black, White, Red, Blue, Green];

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
            c @ 24...28 => SelectTwoSameTokens(color[c - 24]),
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

    fn eval_by_selection(
        &self,
        input: GameCommand,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        use self::GameCommand::*;

        match input {
            ReserveDevelopmentCard { x, y } => self.reserve_development_card(x, y, user, board),
            BuyDevelopmentCard { x, y } => self.buy_development_card(x, y, user, board),
            SelectTwoSameTokens(color) => self.select_two_same_tokens(color, user, board),
            SelectThreeTokens(color1, color2, color3) => {
                self.select_three_tokens(color1, color2, color3, user, board)
            }
            ReserveStackCard(level) => self.reserve_stack_card(level, user, board),
            BuyReservedCard(index) => self.buy_reserved_card(index, user, board),
        }
    }

    pub fn print(&self, result: &str, user: &User) -> () {
        println!("{}", result);
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

    fn buy_development_card(
        &self,
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
            Ok("試行: カードの購入, 結果: カードを購入しました")
        } else {
            Err("試行: カードの購入, 結果: 必要な宝石数が足りません")
        }
    }

    fn select_two_same_tokens(
        &self,
        color: Color,
        user: &mut User,
        board: &mut Board,
    ) -> Result<&'static str, &'static str> {
        if board.can_get_token(color) {
            for _ in 0..2 {
                let token = board.uget_token(color);
                user.add_token(token);
            }
            Ok("試行: トークンを取得, 結果: トークンを取得しました")
        } else {
            Err("試行: トークンを取得, 結果: 残りのトークン数が4より少ないです")
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
        let mut count = 0;
        if let Some(token) = board.get_token(color1) {
            user.add_token(token);
            count = count + 1;
        }
        if let Some(token) = board.get_token(color2) {
            user.add_token(token);
            count = count + 1;
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

    fn reserve_stack_card(
        &self,
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
        &self,
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
            Ok("試行: 確保したカードの購入, 結果: カードを購入しました")
        } else {
            Err("試行: 確保したカードの購入, 結果: 必要な宝石数が足りません")
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

    pub fn look(&mut self, step: u8, user: &User, board: &Board) -> GameCommand {
        use self::GameCommand::*;

        let mut action_rewards: Vec<ActionReward> = vec![];
        self.calc_color_value(user, board);

        for input in 0..45 {
            let command = self.to_command(input);
            let mut user = user.clone();
            let mut board = board.clone();
            match command {
                ReserveDevelopmentCard { x, y } => {
                    let output = self.reserve_development_card(x, y, &mut user, &mut board);
                    match output {
                        Ok(_) => action_rewards.push(ActionReward::new(
                            command,
                            *self.color_value.get(&Color::Gold).unwrap(),
                        )),
                        Err(_) => (),
                    };
                }
                BuyDevelopmentCard { x, y } => {
                    let output = self.buy_development_card(x, y, &mut user, &mut board);
                    match output {
                        Ok(_) => match user.get_acquired_cards().as_slice().last() {
                            Some(card) => action_rewards.push(ActionReward::new(
                                command,
                                card.get_point() as f32
                                    + self.color_value.get(&card.get_color()).unwrap(),
                            )),
                            None => (),
                        },
                        Err(_) => (),
                    };
                }
                SelectTwoSameTokens(c) => {
                    let result = self.select_two_same_tokens(c, &mut user, &mut board);
                    match result {
                        Ok(_) => action_rewards.push(ActionReward::new(
                            command,
                            2.0 * *self.color_value.get(&c).unwrap(),
                        )),
                        Err(_) => (),
                    };
                }
                SelectThreeTokens(c1, c2, c3) => {
                    let t1 = user.get_number_of_tokens(c1);
                    let t2 = user.get_number_of_tokens(c2);
                    let t3 = user.get_number_of_tokens(c3);

                    let result = self.select_three_tokens(c1, c2, c3, &mut user, &mut board);

                    let t1 = user.get_number_of_tokens(c1) - t1;
                    let t2 = user.get_number_of_tokens(c2) - t2;
                    let t3 = user.get_number_of_tokens(c3) - t3;

                    let mut total = 0.0;

                    if t1 > 0 {
                        total += self.color_value.get(&c1).unwrap();
                    }
                    if t2 > 0 {
                        total += self.color_value.get(&c2).unwrap();
                    }
                    if t3 > 0 {
                        total += self.color_value.get(&c3).unwrap();
                    }

                    match result {
                        Ok(_) => action_rewards.push(ActionReward::new(command, total)),
                        Err(_) => (),
                    };
                }
                ReserveStackCard(l) => {
                    let result = self.reserve_stack_card(l, &mut user, &mut board);
                    match result {
                        Ok(_) => action_rewards.push(ActionReward::new(command, 0.0)),
                        Err(_) => (),
                    };
                }
                BuyReservedCard(index) => {
                    let result = self.buy_reserved_card(index, &mut user, &mut board);
                    match result {
                        Ok(_) => match user.get_acquired_cards().as_slice().last() {
                            Some(card) => action_rewards.push(ActionReward::new(
                                command,
                                card.get_point() as f32
                                    + self.color_value.get(&card.get_color()).unwrap(),
                            )),
                            None => (),
                        },
                        Err(_) => (),
                    };
                }
            }
        }

        let mut max_value = 0.0;
        let mut command = GameCommand::ReserveDevelopmentCard { x: 0, y: 0 };

        for e in action_rewards.iter() {
            println!("{:?}", e);
            match e {
                ActionReward { action, reward } => {
                    if *reward > max_value {
                        command = action.clone();
                        max_value = *reward;
                    }
                }
            }
        }

        command
    }

    fn calc_color_value(&mut self, user: &User, board: &Board) {
        let mut required_cost = JewelryBox::new();
        let mut owned = JewelryBox::new();
        let colors = [
            Color::Black,
            Color::White,
            Color::Red,
            Color::Blue,
            Color::Green,
        ];

        // 基礎点 = 0.3
        // α = 1 - 所持宝石数 / 盤面の必要な宝石数
        for row in 0..3 {
            for col in 0..4 {
                if let Some(card) = board.peek_card(row, col) {
                    for color in colors.iter() {
                        required_cost.add_jewelry(*color, card.get_cost(*color));
                    }
                }
            }
        }

        for card in user.get_acquired_cards().iter() {
            for color in colors.iter() {
                owned.add_jewelry(*color, card.get_cost(*color));
            }
        }

        let mut max_color_value = 0.0;
        for color in colors.iter() {
            let color_value = self.color_value.get_mut(color).unwrap();
            *color_value = 0.3
                * (1.0
                    - owned.get_jewelry(*color) as f32 / required_cost.get_jewelry(*color) as f32);

            if max_color_value <= *color_value {
                max_color_value = *color_value;
            }
        }

        let gold_color_value = self.color_value.get_mut(&Color::Gold).unwrap();
        *gold_color_value = max_color_value;
    }
}
