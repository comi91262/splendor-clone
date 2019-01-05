use std::fmt;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Color {
    Black,
    White,
    Red,
    Blue,
    Green,
    Gold,
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "黒"),
            Color::White => write!(f, "白"),
            Color::Red => write!(f, "赤"),
            Color::Blue => write!(f, "青"),
            Color::Green => write!(f, "緑"),
            Color::Gold => write!(f, "金"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "黒"),
            Color::White => write!(f, "白"),
            Color::Red => write!(f, "赤"),
            Color::Blue => write!(f, "青"),
            Color::Green => write!(f, "緑"),
            Color::Gold => write!(f, "金"),
        }
    }
}
