use crate::color::Color::*;
use crate::color::Color;

use std::collections::HashMap;
use std::fmt;

const MAX_NUMBER_OF_TOKEN: usize = 5;

#[derive(Debug)]
pub struct TokenStack(HashMap<Color, Vec<Token>>);

impl TokenStack {
    pub fn new() -> TokenStack {
        let colors = [Black, White, Red, Blue, Green, Gold];

        let mut new_stack = HashMap::new();
        for color in colors.into_iter() {
            new_stack.insert(color.clone(), vec![]);
        }

        TokenStack(new_stack)
    }

    pub fn len(&self, color: Color) -> u8 {
        self.0.get(&color).unwrap().len() as u8
    }
}

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
