use crate::dom::Bound;
use crate::font::Font;
use std::collections::HashMap;

use num_traits::{clamp, clamp_min};

#[derive(Debug)]
struct ElementData {
    value: String,
}

#[derive(Debug)]
pub struct Input {
    pub value: String,
    pub cursor: usize,
    pub cursor_pos: f32,
    pub push_left: f32,
}

#[derive(Debug)]
pub struct Ddom {
    data: HashMap<usize, ElementData>,
    pub input_data: HashMap<usize, Input>,
    pub font: Font,
}

impl Ddom {
    pub fn new() -> Ddom {
        Ddom {
            data: HashMap::new(),
            input_data: HashMap::new(),
            font: Font::new(),
        }
    }

    pub fn create_input(&mut self, id: usize) {
        self.input_data.insert(
            id,
            Input {
                value: "".to_string(),
                cursor: 0,
                cursor_pos: 0.0,
                push_left: 0.0,
            },
        );
    }

    pub fn input(&mut self, value: char, offset: usize, id: &usize, container: f32) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            let mut size: f32 = 0.0;

            data_element.value.push(value);
            data_element.cursor += 1;

            for c in data_element.value.chars() {
                let measure = self.font.get(c.to_string());
                size += (measure.advance * 0.07);
            }

            //println!("{:?}", clamp(size, 0.0, container) );
            data_element.cursor_pos = clamp(size, 0.0, container);
            data_element.push_left = clamp_min(size - container, 0.0);
        }
    }

    pub fn backspace(&mut self, id: &usize) {}
}
