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

fn eval(s: &str) -> () {}

fn print() -> () {
    println!("{}", GUIDE.to_string());
}

fn main() {
    println!("{}", GUIDE.to_string());

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
