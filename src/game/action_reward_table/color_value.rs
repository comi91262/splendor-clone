use std::collections::HashMap;

use super::ColorValue;
use crate::game::color::Color::{self, *};
use crate::game::gem::GEMS;

impl ColorValue {
    pub fn new() -> ColorValue {
        let mut color_value = HashMap::new();
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.into_iter() {
            color_value.insert(color.clone(), 0.0);
        }
        ColorValue(color_value)
    }
    pub fn get(&self, color: Color) -> f32 {
        self._get(color)
    }
    pub fn set(&mut self, color: Color, value: f32) {
        self.0.insert(color, value);
    }

    pub fn set_gold_value(&mut self) {
        let mut max = 0.0;
        for color in GEMS.iter() {
            let color_value = self._get(*color);
            if max <= color_value {
                max = color_value;
            }
        }
        self.set(Gold, max);
    }

    fn _get(&self, color: Color) -> f32 {
        self.0.get(&color).unwrap().clone()
    }
}
