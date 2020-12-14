use crate::boot::Cursor;
use crate::font::Font;
use crate::low_dom::LowDom;
use crate::style::Overflow;
use crate::style::Style;
use num_traits::{clamp, clamp_max, clamp_min, sign};

#[derive(Debug)]
pub struct DivResult {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub far_y: f32,
}

#[derive(Debug)]
pub struct Div {
    pub result: DivResult,
    pub style: Style,
}

impl Div {
    pub fn new(style: Style, low_dom: &mut LowDom) -> Self {
        Div {
            result: DivResult {
                width: 0.0,
                height: 0.0,
                x: 0.0,
                y: 0.0,
                far_y: 0.0,
            },
            style,
        }
    }

    pub(crate) fn append(&self, id: usize) -> usize {
        if let Overflow::Scroll = self.style.overflow {
            id + 1
        } else {
            id
        }
    }
}
