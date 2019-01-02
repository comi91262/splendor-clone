// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};
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
use crate::user::User;

lazy_static! {
    static ref CREATED_FILE_PATH: String = {
        let working_directory = env!("CARGO_MANIFEST_DIR");
        let file_path = "card.json";
        format!("{}/{}", working_directory, file_path)
    };
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
// 上述記事中のUserRepository相当
// pub trait UserDao {/* ... */}
// 上述記事中のUserRepositoryComponent相当
// pub trait HaveUserDao {
//   type UserDao: UserDao;
//   fn user_dao(&self) -> Self::UserDao;
// }
// 上述記事中のUserService相当
// trait UserService: HaveUserDao {
//   pub fn get_user_by_id(&self, id: i32) -> Result<Option<User>> {
//     self.user_service().find_user(id)
//   }
// }
// UserServiceはHaveUserDaoにのみ依存するのでそれさえ実装していれば自動で実装を与えられます。
// もちろんテストなどで挙動を上書きしたければ具体的な型での実装で上書きできます。
// impl<T:HaveUserDao> UserService for T {}

// 上述記事中のUserServiceComponent相当
// trait HaveUserService {
//   type UserService: UserService;
//   fn user_service(&self) -> Self::UserService;
// }
// struct Server {
//   user_dao: UserPgDao,
//   group_dao: GroupPgDao,
// }

// impl HaveUserDao for Server {
//   type UserDao = UserPgDao;
//   fn user_dao(&self) -> Self::UserDao {
//     &self.user_dao
//   }
// }
// impl HaveUserService for Server{
//   type UserService = Self;
//   fn user_service(&self) -> Self::UserService {
//     self
//   }
// }

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
