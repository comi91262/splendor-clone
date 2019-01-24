use crate::color::Color;
use crate::level::Level;

use std::collections::HashMap;
use std::fmt;

mod card;

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub level: u8,
    pub color: Color,
    pub point: u8,
    pub cost_black: u8,
    pub cost_white: u8,
    pub cost_red: u8,
    pub cost_blue: u8,
    pub cost_green: u8,
}

#[derive(Clone)]
pub struct CardStack(HashMap<Level, Vec<Card>>);

impl CardStack {
    pub fn new() -> CardStack {
        let cards = Card::load("json/card.json");

        let mut level1_stack = vec![];
        let mut level2_stack = vec![];
        let mut level3_stack = vec![];
        for card in cards.into_iter() {
            match card {
                Card { level: 1, .. } => level1_stack.push(card),
                Card { level: 2, .. } => level2_stack.push(card),
                Card { level: 3, .. } => level3_stack.push(card),
                Card { level: _, .. } => unreachable!(),
            }
        }

        // シャッフルする
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        level1_stack.shuffle(&mut rng);
        level2_stack.shuffle(&mut rng);
        level3_stack.shuffle(&mut rng);

        let mut stack = HashMap::new();
        stack.insert(Level::One, level1_stack);
        stack.insert(Level::Two, level2_stack);
        stack.insert(Level::Three, level3_stack);

        CardStack(stack)
    }

    pub fn len(&self, level: Level) -> u8 {
        self.0.get(&level).unwrap().len() as u8
    }

    pub fn get(&mut self, level: Level) -> Option<Card> {
        self.0.get_mut(&level).unwrap().pop()
    }
}
