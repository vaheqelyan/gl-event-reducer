use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ddom::Ddom;

use crate::input::input_element::Input;

#[derive(Debug)]
pub enum Element {
    Input,
    Box,
}

#[derive(Debug)]
pub struct Dom {
    pub vec: Vec<usize>,
    map: HashMap<usize, Bound>,
    id: usize,
    pub ddom: Ddom,
}

#[derive(Debug)]
pub struct Bound {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,

    pub element: Element,
}

impl Dom {
    pub fn new() -> Self {
        Dom {
            vec: Vec::new(),
            map: HashMap::new(),
            id: 0,
            ddom: Ddom::new(),
        }
    }
    pub fn get(&self, id: usize) -> &Bound {
        let val = self.map.get(&id).unwrap();
        val
    }
    pub fn create(&mut self, value: Bound) -> usize {
        if let Element::Input = value.element {
            self.ddom.create_input(self.id + 1);
        }

        self.vec.push(self.id);
        self.map.insert(self.id, value);
        self.id += 1;

        self.id
    }

    pub fn get_input(&self, id: usize) -> &Input {
        self.ddom.input_data.get(&(&id + 1)).unwrap()
    }
}
