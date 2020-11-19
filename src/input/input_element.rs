#[derive(Debug)]
pub struct Input {
    pub value: String,
    pub cursor: usize,
    pub cursor_pos: f32,
    pub push_left: f32,
    pub stop_backspace: bool,
}
