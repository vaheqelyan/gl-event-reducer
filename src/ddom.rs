use crate::boot::Cursor;
use crate::dom::Bound;
use crate::font::Font;
use std::collections::HashMap;

use num_traits::{clamp, clamp_min};

use crate::input::input_element::Input;

#[derive(Debug)]
struct ElementData {
    value: String,
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
                focus_range: 0.0,
                focus_x: 0.0,
                value: "".to_string(),
                cursor: 0,
                cursor_pos: 0.0,
                push_left: 0.0,
                stop_backspace: true,
                cache_len: 0,
            },
        );
    }

    pub fn input(&mut self, value: char, offset: usize, id: &usize, container: f32) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            data_element.input(value, offset, container, &mut self.font);
        }
    }

    pub fn cursor_right(&mut self, id: &usize, container: f32) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            data_element.cursor_right(container, &mut self.font);
        }
    }

    pub fn cursor_left(&mut self, id: &usize, container: f32) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            data_element.cursor_left(container, &mut self.font);
        }
    }

    pub fn backspace(&mut self, id: &usize, container: f32) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            data_element.backspace(container, &mut self.font);
        }
    }

    pub fn focus(&mut self, id: &usize, container: f32, x: f32, y: f32, cursor: &Cursor) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            data_element.focus(container, x, y, cursor, &mut self.font);
        }
    }

    pub fn select(&mut self, id: &usize, container: f32, x: f32, y: f32, cursor: &Cursor) {
        if let Some(data_element) = self.input_data.get_mut(&(id + 1)) {
            data_element.select(container, x, y, cursor, &mut self.font);
        }
    }
}
