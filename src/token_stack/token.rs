use super::Token;
use crate::color::Color;

use std::fmt;

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
}
