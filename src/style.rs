#[derive(Debug)]
pub enum Dimension {
    Px(f32),
    Perc(f32),
    Auto,
    None,
}

#[derive(Debug)]
pub enum Display {
    Block,
    InlineBlock,
}

#[derive(Debug)]
pub enum Overflow {
    Scroll,
    Hidden,
}

#[derive(Debug)]
pub struct Style {
    pub width: Dimension,
    pub height: Dimension,
    pub display: Display,
    pub bg_color: [f32; 3],
    pub overflow: Overflow,
    pub max_height: Dimension,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            width: Dimension::Px(0.0),
            height: Dimension::Px(0.0),
            display: Display::Block,
            bg_color: [0.0, 0.0, 0.0],
            overflow: Overflow::Hidden,
            max_height: Dimension::None,
        }
    }
}
