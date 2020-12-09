#[derive(Debug)]
pub enum Dimension {
    Px(f32),
    Perc(f32),
    Auto,
    None,
    Grow(f32),
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

#[derive(Debug, Clone)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug)]
pub struct Style {
    pub direction: Direction,
    pub width: Dimension,
    pub height: Dimension,
    pub display: Display,
    pub bg_color: [f32; 3],
    pub overflow: Overflow,
    pub max_height: Dimension,
    pub left: Dimension,
    pub top: Dimension,
    pub right: Dimension,
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
            left: Dimension::Px(0.0),
            top: Dimension::Px(0.0),
            right: Dimension::Px(0.0),
            direction: Direction::Row,
        }
    }
}
