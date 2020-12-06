use crate::ddom::Ddom;
use crate::div::div_element::Div;
use crate::dom::{Element, ElementMetaData};
use crate::style::{Dimension, Display};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Layout {
    width: f32,
    height: f32,
}

impl Layout {
    pub(crate) fn new() -> Self {
        Layout {
            width: 0.0,
            height: 0.0,
        }
    }
    pub(crate) fn viewport(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
    pub(crate) fn get_children(
        &self,
        element: &usize,
        child_parent: &HashMap<usize, usize>,
    ) -> Vec<usize> {
        //println!("{:?} {:?}", element, child_parent);
        let mut els = vec![];

        for (key, value) in child_parent.iter().sorted() {
            if element == value {
                els.push(*key);
            }
        }
        els
    }
    pub(crate) fn parent_bound(
        &self,
        child: usize,
        element: usize,
        ddom: &Ddom,
    ) -> (f32, f32, f32, f32) {
        if child == element {
            (self.width, self.height, 0.0, 0.0)
        } else {
            let result = &ddom.div_data.get(&element).unwrap().result;
            (result.width, result.height, result.x, result.y)
        }
    }

    pub(crate) fn set_size(div: &mut Div, parent_width: f32, parent_height: f32) {
        match div.style.width {
            Dimension::Perc(width) => {
                let w = width / 100.0 * parent_width;
                div.result.width = w;
            }
            Dimension::Px(width) => {
                div.result.width = width;
            }
            Dimension::Auto => (),
            Dimension::None => (),
        }

        match div.style.height {
            Dimension::Perc(height) => {
                let h = height / 100.0 * parent_height;
                div.result.height = h;
            }
            Dimension::Px(height) => {
                div.result.height = height;
            }

            Dimension::Auto => (),
            Dimension::None => (),
        }
    }

    pub(crate) fn traverse(
        &mut self,
        element: usize,
        children: Vec<usize>,
        ddom: &mut Ddom,
        metadata: &HashMap<usize, ElementMetaData>,
        child_parent: &HashMap<usize, usize>,
        parent_id: usize,
    ) {
        let desc = ddom.div_data.get(&element).unwrap();
        let mut x = desc.result.x;
        let mut y = desc.result.y;
        for child in children {
            let children = self.get_children(&child, child_parent);
            let (parent_width, parent_height, parent_x, parent_y) =
                self.parent_bound(child, element, ddom);

            let mut desc = ddom.div_data.get_mut(&child).unwrap();
            println!("{:?} {:?}", child, metadata.get(&child).unwrap());

            Layout::set_size(&mut desc, parent_width, parent_height);

            match desc.style.display {
                Display::Block => {
                    desc.result.y = y;
                    desc.result.x = x;
                    y += parent_y + desc.result.height;
                }
                Display::InlineBlock => {
                    desc.result.x = x;
                    desc.result.y = y;
                    x += parent_x + desc.result.width;
                }
            };

            self.traverse(child, children, ddom, metadata, child_parent, element);
        }
    }

    pub(crate) fn calculate_bound(
        &mut self,
        el_ids: &Vec<usize>,
        el_meta: &HashMap<usize, ElementMetaData>,
        ddom: &mut Ddom,
        child_parent: &HashMap<usize, usize>,
    ) {
        self.traverse(el_ids[0], vec![0], ddom, el_meta, child_parent, 0);
    }
}
