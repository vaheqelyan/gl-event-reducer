use crate::ddom::Ddom;
use crate::layout::Layout;
use crate::style::{Dimension, Display, Overflow, Style};
use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::div::div_element::Div;
use crate::input::input_element::Input;

#[derive(Debug)]
pub enum Element {
    Input,
    Div,
    Scroll,
}

#[derive(Debug)]
pub struct ElementMetaData {
    pub(crate) element_type: Element,
    pub(crate) belong_to_screen: usize,
    pub(crate) scroll: bool,
}

#[derive(Debug)]
pub struct Dom {
    pub vec: Vec<usize>,
    map: HashMap<usize, ElementMetaData>,
    child_parent: HashMap<usize, usize>,
    id: usize,
    pub ddom: Ddom,
    layout: Layout,
}

impl Dom {
    pub fn new() -> Self {
        Dom {
            vec: Vec::new(),
            map: HashMap::new(),
            child_parent: HashMap::new(),
            id: 0,
            ddom: Ddom::new(),
            layout: Layout::new(),
        }
    }
    pub fn get(&self, id: usize) -> &ElementMetaData {
        let val = self.map.get(&id).unwrap();
        val
    }

    pub fn scroll(&mut self, style: Style) -> usize {
        let id = self.create(ElementMetaData {
            element_type: Element::Scroll,
            belong_to_screen: 0,
            scroll: false,
        });
        self.ddom.register_scroll(id, style);
        id
    }
    pub fn input(&mut self, style: Style) -> usize {
        let id = self.create(ElementMetaData {
            element_type: Element::Input,
            belong_to_screen: 0,
            scroll: false,
        });
        self.ddom.register_input(id, style);
        id
    }
    pub fn div(&mut self, style: Style) -> usize {
        let id = self.create(ElementMetaData {
            element_type: Element::Div,
            belong_to_screen: 0,
            scroll: match style.overflow {
                Overflow::Scroll => true,
                _ => false,
            },
        });

        /*if let Overflow::Scroll = style.overflow {
            let scroll_skillet = self.create(ElementMetaData {
                element_type: Element::Div,
                belong_to_screen: 0,
                scroll: false,
            });

            self.ddom.register_div(
                scroll_skillet,
                Style {
                    width: Dimension::Px(10.0),
                    height: Dimension::Px(100.0),
                    bg_color: [255.0, 220.0, 97.0],
                    right: Dimension::Px(0.1),
                    ..Default::default()
                },
            );

            self.append(scroll_skillet, id);
        }*/

        self.ddom.register_div(id, style);

        id
    }

    pub fn create(&mut self, element_type: ElementMetaData) -> usize {
        self.vec.push(self.id);
        self.map.insert(self.id, element_type);
        self.id += 1;

        self.id - 1
    }

    pub fn top_screen(&mut self, width: f32, height: f32) {
        let id = self.create(ElementMetaData {
            element_type: Element::Div,
            belong_to_screen: 0,
            scroll: true,
        });
        self.ddom.register_div(
            id,
            Style {
                width: Dimension::Px(width),
                height: Dimension::Px(height),
                ..Default::default()
            },
        );

        self.layout.viewport(width, height, &mut self.ddom);
    }

    pub fn get_top_screen(&self) -> usize {
        0
    }

    pub fn get_input(&self, id: usize) -> &Input {
        self.ddom.input_data.get(&(&id + 1)).unwrap()
    }

    pub fn get_div(&self, id: usize) -> &Div {
        self.ddom.div_data.get(&(&id)).unwrap()
    }

    pub fn append(&mut self, element: usize, parent: usize) {
        let meta = self.get(parent);
        let screen_id = meta.belong_to_screen;
        if meta.scroll == true {
            let mut meta_child = self.map.get_mut(&element).unwrap();
            meta_child.belong_to_screen = parent;
        } else {
            let mut meta_child = self.map.get_mut(&element).unwrap();
            meta_child.belong_to_screen = screen_id;
        }

        self.child_parent.insert(element, parent);
    }

    pub fn debug(&self) {
        println!("{:?}", self.vec);
        println!("{:#?}", self.map);
        println!("{:?}", self.child_parent);
    }

    pub fn layout(&mut self) {
        self.layout
            .calculate_bound(&self.vec, &self.map, &mut self.ddom, &self.child_parent);
    }
}
