use crate::color::Color;
use crate::color::Color::*;

use std::collections::HashMap;
use std::fmt;

mod token;

const MAX_NUMBER_OF_TOKEN: usize = 5;

#[derive(Clone)]
pub struct TokenStack(HashMap<Color, Vec<Token>>);

#[derive(Clone, PartialEq)]
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
        let mut stack = HashMap::new();
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.iter() {
            stack.insert(*color, vec![]);
        }
        TokenStack(stack)
    }

    pub fn fill(mut self) -> TokenStack {
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.iter() {
            self.0.insert(*color, TokenStack::create_stack(*color));
        }
        TokenStack(self.0)
        //self.clone()
    }

    fn create_stack(color: Color) -> Vec<Token> {
        let mut stack = vec![];

        for _ in 0..MAX_NUMBER_OF_TOKEN {
            stack.push(Token::new(color));
        }

        stack
    }
    pub fn len(&self, color: Color) -> u8 {
        self.0.get(&color).unwrap().len() as u8
    }
    pub fn len_all(&self) -> u8 {
        let mut sum = 0;
        for (_, each_stack) in self.0.iter() {
            sum += each_stack.len();
        }
        sum as u8
    }
    pub fn add(&mut self, token: Token) {
        let color = token.get_color();
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
    fn test_new() {
        let mut stack = TokenStack::new();
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.iter() {
            assert_eq!(stack.len(*color), 0);
        }

        stack.fill();
        for color in colors.iter() {
            assert_eq!(stack.len(*color), 5);
        }
    }

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
