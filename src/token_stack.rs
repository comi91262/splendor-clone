use crate::color::Color;
use crate::color::Color::*;

use std::collections::HashMap;
use std::fmt;

mod token;

const MAX_NUMBER_OF_TOKEN: usize = 5;

#[derive(Clone)]
pub struct TokenStack(HashMap<Color, Vec<Token>>);

#[derive(Clone, Copy, PartialEq)]
pub struct Token {
    color: Color,
}

impl fmt::Display for TokenStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "取得したトークン:\n{:?}", self.0)
    }
}

impl TokenStack {
    pub fn new() -> TokenStack {
        let colors = [Black, White, Red, Blue, Green, Gold];

        let mut new_stack = HashMap::new();
        for color in colors.into_iter() {
            new_stack.insert(color.clone(), vec![]);
        }

        TokenStack(new_stack)
    }
    pub fn create_stack(color: Color) -> Vec<Token> {
        let mut stack = vec![];

        for _ in 0..MAX_NUMBER_OF_TOKEN {
            stack.push(Token::new(color));
        }

        stack
    }
    pub fn len(&self, color: Color) -> u8 {
        self.0.get(&color).unwrap().len() as u8
    }

    pub fn add(&mut self, token: Token) {
        self._get(token.get_color()).push(token);
    }

    pub fn remove(&mut self, color: Color) -> Option<Token> {
        self._get(color).pop()
    }

    pub fn addn(&mut self, tokens: Vec<Token>) {
        for token in tokens.into_iter() {
            self.add(token);
        }
    }

    pub fn removen(&mut self, color: Color, n: u8) -> Vec<Token> {
        let mut result = vec![];
        for _ in 0..n {
            if let Some(token) = self.remove(color) {
                result.push(token)
            }
        }

        if result.len() != n as usize {
            panic!("n mismatches in removen"); // TODO
        }
        result
    }

    fn _get(&mut self, color: Color) -> &mut Vec<Token> {
        self.0.get_mut(&color).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::TokenStack;
    use crate::color::Color::*;

    #[test]
    fn test_addn() {
        let mut stack = TokenStack::new();
        let tokens = vec![];
        stack.addn(tokens);
        assert_eq!(stack.remove(Black), None);

        let tokens = vec![Token::new(Black), Token::new(Black), Token::new(White)];
        stack.addn(tokens);
        assert_eq!(stack.remove(Black), Some(Token::new(Black)));
        assert_eq!(stack.remove(White), Some(Token::new(White)));
        assert_eq!(stack.remove(Black), Some(Token::new(Black)));
        assert_eq!(stack.remove(Black), None);
        assert_eq!(stack.remove(White), None);
    }

    #[test]
    fn test_removen() {
        let mut stack = TokenStack::new();
        let mut tokens = stack.removen(Black, 0);
        assert_eq!(tokens.pop(), None);

        stack.addn(vec![
            Token::new(Black),
            Token::new(Black),
            Token::new(White),
        ]);
        let mut tokens = stack.removen(White, 1);
        assert_eq!(tokens.pop(), Some(Token::new(White)));
        assert_eq!(tokens.pop(), None);
    }

}
