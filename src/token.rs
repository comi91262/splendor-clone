use crate::color::Color;

const MAX_NUMBER_OF_TOKEN: usize = 5;

#[derive(Clone, Debug, Copy)]
pub struct Token {
    color: Color,
}

impl Token {
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
