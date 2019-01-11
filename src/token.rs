use crate::color::Color;

use std::fmt;

const MAX_NUMBER_OF_TOKEN: usize = 5;

#[derive(Clone, Copy)]
pub struct Token {
    color: Color,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.color)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.color)
    }
}

impl Token {
    pub fn new(color: Color) -> Token {
        Token { color: color }
    }
    pub fn get_color(self) -> Color {
        self.color
    }

    pub fn create_stack(color: Color) -> Vec<Token> {
        let mut stack = vec![];

        for _ in 0..MAX_NUMBER_OF_TOKEN {
            stack.push(Token { color: color });
        }

        stack
    }
}
