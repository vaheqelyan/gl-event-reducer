use crate::boot::Cursor;
use crate::font::Font;
use crate::style::Style;
use num_traits::{clamp, clamp_max, clamp_min, sign};

#[derive(Debug)]
pub struct Scroll {
    style: Style,
}

impl Scroll {
    pub(crate) fn new(style: Style) -> Self {
        Scroll { style }
    }
}
