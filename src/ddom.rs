/*
 * 170.27 9.7300005
 * 160.40001 9.87
 * 154.45001 5.95
 * 148.50002 5.95
 * 142.55002 5.95
 * 136.60002 5.95
 * 130.65002 5.95
 * 124.70003 5.95
 * 118.75003 5.95
 * 112.80003 5.95
 *
 *
 * */
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
                size = (size + (measure.advance * 0.07)).round();
            }
            println!("{:?}", size);

            let measure = self.font.get(value.to_string());

            //println!("{:?} {:?} {:?}", size,measure.advance * 0.07,measure.width * 0.07);
            let r = ((measure.advance - measure.width) * 0.07).abs().round();
            //println!("{:?}",r);
            data_element.cursor_pos = clamp(size - r, 0.0, container - 1.0);
            data_element.push_left = clamp_min(size - container, 0.0);
        }
    }

    pub fn cursor_left(&mut self, id: &usize, container: f32) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            if data_element.cursor > 0 {
                let mut size: f32 = 0.0;

                for (pos, c) in data_element.value.chars().enumerate() {
                    if pos < data_element.cursor - 1 {
                        let measure = self.font.get(c.to_string());
                        size = (size + (measure.advance * 0.07)).round();
                    }
                }

                let original = size;
                size -= data_element.push_left;

                let cursor_at = data_element.cursor as usize;
                let cur_char = data_element.value.chars().nth(cursor_at - 1).unwrap();

                let measure = self.font.get(cur_char.to_string());

                let r = ((measure.advance - measure.width) * 0.07).abs().round();

                data_element.cursor_pos = clamp((size - r), 0.0, container - 1.0);
                data_element.cursor -= 1;
                data_element.push_left = if size.is_sign_negative() {
                    let d = (data_element.push_left - original).abs();
                    data_element.push_left - d
                //data_element.push_left - ((measure.width * 0.07).round() + size)
                } else {
                    data_element.push_left
                };
            }
        }
    }

    pub fn backspace(&mut self, id: &usize, container: f32) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            if !data_element.value.is_empty() {
                let c = data_element.value.remove(data_element.cursor - 1);

                let mut size: f32 = 0.0;
                for (pos, c) in data_element.value.chars().enumerate() {
                    if pos < data_element.cursor {
                        let measure = self.font.get(c.to_string());
                        size = (size + (measure.advance * 0.07)).round();
                    }
                }

                let cursor_at = data_element.cursor as usize;
                let cur_char = data_element.value.chars().nth(cursor_at - 2).unwrap();
                let measure = self.font.get(cur_char.to_string());
                let r = ((measure.advance - measure.width) * 0.07).abs().round();

                data_element.cursor -= 1;
                data_element.cursor_pos = clamp(size - r, 0.0, container);
                data_element.push_left = clamp_min(size - container, 0.0);
            }
        }
    }
}
