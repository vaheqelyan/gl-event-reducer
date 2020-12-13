use crate::boot::Cursor;
use crate::font::Font;
use crate::low_dom::LowDom;
use std::collections::HashMap;

use crate::dom_db::DomDB as SelfDomDB;

use crate::style::Style;
use num_traits::{clamp, clamp_min};

use crate::div::div_element::Div;
use crate::input::input_element::Input;

pub(crate) struct DomDB {
    pub input_data: HashMap<usize, Input>,
    pub div_data: HashMap<usize, Div>,
    pub font: Font,
}

impl DomDB {
    pub(crate) fn new() -> Self {
        DomDB {
            input_data: HashMap::new(),
            div_data: HashMap::new(),
            font: Font::new(),
        }
    }

    pub fn register_div(&mut self, id: usize, style: Style, low_dom: &mut LowDom) {
        self.div_data.insert(id, Div::new(style, low_dom));
    }

    pub fn register_input(&mut self, id: usize, style: Style) {
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

    pub fn get_input(&self, id: usize) -> &Input {
        self.input_data.get(&(&id + 1)).unwrap()
    }

    pub fn get_div(&self, id: usize) -> &Div {
        self.div_data.get(&(&id)).unwrap()
    }
}
