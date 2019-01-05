use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize, Deserialize)]
pub struct NobleTile {
    point: u8,
    black_bonus: u8,
    white_bonus: u8,
    red_bonus: u8,
    blue_bonus: u8,
    green_bonus: u8,
}

impl fmt::Debug for NobleTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}点 {} {} {} {} {}",
            self.point,
            self.black_bonus,
            self.white_bonus,
            self.red_bonus,
            self.blue_bonus,
            self.green_bonus,
        )
    }
}

impl fmt::Display for NobleTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
{}点 {} {} {} {} {}
        ",
            self.point,
            self.black_bonus,
            self.white_bonus,
            self.red_bonus,
            self.blue_bonus,
            self.green_bonus,
        )
    }
}

impl NobleTile {
    pub fn create_stack() -> Vec<NobleTile> {
        let mut stack = vec![];

        for result in BufReader::new(File::open("noble_tile.json").unwrap()).lines() {
            let l = result.unwrap();
            let tile: NobleTile = serde_json::from_str(&l).unwrap();
            stack.push(tile);
        }

        stack
    }
}
