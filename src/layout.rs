use crate::div::div_element::Div;
use crate::dom::Dom;
use crate::dom_db::DomDB;
use crate::low_dom::{Element, ElementMetaData};
use crate::style::{Dimension, Direction, Display};
use itertools::Itertools;
use num_traits::clamp_min;
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
    pub(crate) fn viewport(&mut self, width: f32, height: f32, dom_db: &mut DomDB) {
        self.width = width;
        self.height = height;
        let mut result = &mut dom_db.div_data.get_mut(&0).unwrap().result;
        result.width = width;
        result.height = height;
    }
    pub(crate) fn get_children(
        &self,
        element: &usize,
        child_parent: &HashMap<usize, usize>,
    ) -> Vec<usize> {
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
        dom_db: &DomDB,
    ) -> (f32, f32, f32, f32) {
        if child == element {
            (self.width, self.height, 0.0, 0.0)
        } else {
            let result = &dom_db.div_data.get(&element).unwrap().result;
            (result.width, result.height, result.x, result.y)
        }
    }

    pub(crate) fn calc_dimension(
        dimension: &Dimension,
        bound: f32,
        grow_factor: f32,
        c_size: f32,
    ) -> f32 {
        match dimension {
            Dimension::Perc(width) => {
                let w = width / 100.0 * c_size;
                w
            }
            Dimension::Px(width) => *width,
            Dimension::Grow(grow) => grow * grow_factor,
            _ => 0.0,
        }
    }

    pub(crate) fn set_box_size(
        div: &mut Div,
        container: f32,
        grow_factor: f32,
        c_width: f32,
        c_height: f32,
    ) {
        div.result.width =
            Layout::calc_dimension(&div.style.width, container, grow_factor, c_width);

        div.result.height =
            Layout::calc_dimension(&div.style.height, container, grow_factor, c_height);
    }

    pub(crate) fn get_dimension(div: &Dimension) -> f32 {
        match div {
            Dimension::Px(width) => *width,
            _ => 0.0,
        }
    }

    pub(crate) fn get_flex_container(
        children: &Vec<usize>,
        dir: &Direction,
        container: f32,
        dom_db: &DomDB,
    ) -> (f32, f32, f32) {
        let remain_space: Option<(f32, f32, f32)> =
            children.iter().fold(Some((0.0, 0.0, 0.0)), |acc, element| {
                let style = &dom_db.div_data.get(&element).unwrap().style;

                let basis = match dir {
                    Direction::Row => {
                        Layout::calc_dimension(&style.width, container, 0.0, container)
                    }
                    Direction::Column => {
                        Layout::calc_dimension(&style.height, container, 0.0, container)
                    }
                };

                let grow = match dir {
                    Direction::Row => match style.width {
                        Dimension::Grow(grow) => grow,
                        _ => 0.0,
                    },
                    Direction::Column => match style.height {
                        Dimension::Grow(grow) => grow,
                        _ => 0.0,
                    },
                };

                let margin = match dir {
                    Direction::Row => Layout::get_dimension(&style.margin_left),
                    Direction::Column => Layout::get_dimension(&style.margin_top),
                };

                //let margin = Layout::get_dimension(&style.margin_left);

                let (basis_result, grow_result, margin_result) = acc.unwrap();

                Some((
                    basis + basis_result,
                    grow + grow_result,
                    margin + margin_result,
                ))
            });

        remain_space.unwrap()
    }

    pub(crate) fn traverse(
        &mut self,
        element: usize,
        children: Vec<usize>,
        dom_db: &mut DomDB,
        metadata: &HashMap<usize, ElementMetaData>,
        child_parent: &HashMap<usize, usize>,
        parent_id: usize,
        mut far_y: f32,
    ) {
        let desc = dom_db.div_data.get(&element).unwrap();
        let mut x = desc.result.x;
        let mut y = desc.result.y;

        let mut container = match desc.style.direction {
            Direction::Column => desc.result.height,
            Direction::Row => desc.result.width,
        };

        let bound_width = desc.result.width;
        let bound_height = desc.result.height;
        let bound_x = desc.result.x;
        let bound_y = desc.result.y;

        let dir = &desc.style.direction.clone();

        let (remain, total_grow, margin) =
            Layout::get_flex_container(&children, &dir, container, &dom_db);

        container = clamp_min((container - remain) - margin, 0.0);
        let grow_factor = container / total_grow;

        for child in children {
            let children = self.get_children(&child, child_parent);

            {
                let mut desc = dom_db.div_data.get_mut(&child).unwrap();

                Layout::set_box_size(&mut desc, container, grow_factor, bound_width, bound_height);

                match dir {
                    Direction::Column => {
                        let margin = Layout::calc_dimension(
                            &desc.style.margin_top,
                            container,
                            0.0,
                            bound_height,
                        );

                        desc.result.y = y + margin;
                        desc.result.x = x;
                        y += (desc.result.height) + margin;
                    }
                    Direction::Row => {
                        let margin = Layout::calc_dimension(
                            &desc.style.margin_left,
                            container,
                            0.0,
                            bound_width,
                        );

                        desc.result.x = x + margin;
                        desc.result.y = y;
                        x += (desc.result.width) + margin;
                    }
                };

                far_y = y;
            };

            self.traverse(
                child,
                children,
                dom_db,
                metadata,
                child_parent,
                element,
                far_y,
            );
        }
    }

    pub(crate) fn calculate_bound(
        &mut self,
        el_ids: &Vec<usize>,
        el_meta: &HashMap<usize, ElementMetaData>,
        dom_db: &mut DomDB,
        child_parent: &HashMap<usize, usize>,
    ) {
        self.traverse(el_ids[0], vec![0], dom_db, el_meta, child_parent, 0, 0.0);
    }

    pub(crate) fn layout(&mut self, dom: &mut Dom) {
        self.calculate_bound(
            &dom.low_dom.vec,
            &dom.low_dom.map,
            &mut dom.dom_db,
            &dom.low_dom.child_parent,
        );
    }
}
