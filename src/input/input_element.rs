use crate::app::Cursor;
use crate::dom::Bound;
use crate::font::Font;
use num_traits::{clamp, clamp_max, clamp_min, sign};

#[derive(Debug)]
pub struct Input {
    pub value: String,
    pub cursor: usize,
    pub cursor_pos: f32,
    pub push_left: f32,
    pub stop_backspace: bool,
    pub cache_len: usize,
    pub focus_x: f32,
    pub focus_range: f32,
}

impl Input {
    pub fn input(&mut self, value: char, offset: usize, container: f32, font: &mut Font) {
        let mut size: f32 = 0.0;

        self.value.insert(self.cursor, value);

        self.cursor += 1;
        self.cache_len = 0;

        for (pos, c) in self.value.chars().enumerate() {
            if pos < self.cursor {
                let measure = font.get(c.to_string());
                size = (size + (measure.advance * 0.07)).round();
            }

            self.cache_len += 1;
        }

        let original = size;
        size -= self.push_left;

        self.cursor_pos = clamp(size, 0.0, container - 1.0);

        let is_out_of_range = !((original - container) - self.push_left).is_sign_negative();

        self.push_left = if is_out_of_range {
            original - container
        } else {
            self.push_left
        };
        self.stop_backspace = false;
    }

    pub fn cursor_right(&mut self, container: f32, font: &Font) {
        let mut size: f32 = 0.0;

        for (pos, c) in self.value.chars().enumerate() {
            if pos < self.cursor + 1 {
                let measure = font.get(c.to_string());

                size = (size + (measure.advance * 0.07)).round();
            }
        }

        let original = size;
        size -= self.push_left;

        self.cursor_pos = clamp(size, 0.0, container - 1.0);
        self.cursor = clamp_max(self.cursor + 1, self.cache_len);

        let is_out_of_range = !((original - container) - self.push_left).is_sign_negative();

        self.push_left = if is_out_of_range {
            original - container
        } else {
            self.push_left
        };

        self.stop_backspace = self.cursor == 0;
    }

    pub fn cursor_left(&mut self, container: f32, font: &Font) {
        let mut size: f32 = 0.0;

        if self.cursor > 0 {
            for (pos, c) in self.value.chars().enumerate() {
                if pos < self.cursor - 1 {
                    let measure = font.get(c.to_string());

                    size = (size + (measure.advance * 0.07)).round();
                }
            }

            let original = size;
            size -= self.push_left;

            self.cursor_pos = clamp(size, 0.0, container - 1.0);
            self.cursor -= 1;
            self.push_left = if size.is_sign_negative() {
                let d = (self.push_left - original).abs();
                self.push_left - d
            } else {
                self.push_left
            };

            self.stop_backspace = self.cursor == 0;
        }
    }

    pub fn backspace(&mut self, container: f32, font: &Font) {
        if !self.stop_backspace {
            let new_cursor = clamp_min(self.cursor - 1, 0);
            let c = self.value.remove(self.cursor - 1);

            self.cursor -= 1;

            let mut size: f32 = 0.0;
            for (pos, c) in self.value.chars().enumerate() {
                if pos < self.cursor {
                    let measure = font.get(c.to_string());
                    size = (size + (measure.advance * 0.07)).round();
                }
            }

            let original = size;

            let measure = font.get(c.to_string());
            self.push_left = clamp_min(self.push_left - (measure.advance * 0.07), 0.0).floor();

            size -= self.push_left;

            self.cache_len -= 1;

            self.cursor_pos = clamp(size, 0.0, container - 1.0).floor();
            self.stop_backspace = self.cursor == 0;
        }
    }

    pub fn focus(&mut self, container: f32, x: f32, y: f32, cursor: &Cursor, font: &Font) {
        let x_input = cursor.x - x;

        self.focus_x = x_input;

        let mut size: f32 = 0.0;
        let mut stop: bool = false;
        for (pos, c) in self.value.chars().enumerate() {
            let measure = font.get(c.to_string());
            let char_size = (size + (measure.advance * 0.07)).round();

            if char_size > self.push_left + x_input && !stop {
                self.focus_range = char_size;
                let foo = char_size - self.push_left;
                let bar = foo - x_input;
                let perc = bar / (measure.advance * 0.07);
                if perc > 0.5 {
                    self.cursor_pos = size - self.push_left;
                    self.cursor = pos;
                } else {
                    self.cursor_pos = char_size - self.push_left;
                    self.cursor = pos + 1;
                }
                stop = true;
            }

            size = char_size;
        }
        self.stop_backspace = self.cursor == 0;
    }

    pub fn select(&mut self, container: f32, x: f32, y: f32, cursor: &Cursor, font: &Font) {
        let x_input = cursor.x - x;
        let screen_width: f32 = 300.0;
        let dir = sign::signum(x_input - self.focus_x);

        let mut size: f32 = 0.0;
        let mut range_size: f32 = 0.0;
        println!("-----------");

        // Get width of range
        for c in self.value.chars() {
            let measure = font.get(c.to_string());
            let char_size = (size + (measure.advance * 0.07)).round();

            /*println!(
                "{:?} {:?} {:?} {:?}",
                c,
                char_size * dir,
                self.focus_range * dir,
                char_size * dir >= self.focus_range * dir
                    && char_size * dir
                        < (((self.focus_range + (x_input - self.focus_range)) + self.push_left)
                            * dir)
                            .round()
            );*/

            if char_size * dir >= self.focus_range * dir
                && char_size * dir
                    < ((self.focus_range + (x_input - self.focus_range)) + self.push_left) * dir
            {
                println!("{:?}", c);
                if char_size > self.focus_range {
                    range_size = (range_size + (measure.advance * 0.07)).round();
                }
            }
            size = char_size;
        }

        // Get width of text at cursor
        let original = self.focus_range + range_size;

        // Check if it is out of range
        let is_out_of_range = !((original - container) - self.push_left).is_sign_negative();

        // If true substract :else return sam value
        self.push_left = if is_out_of_range {
            (original - container)
        } else {
            self.push_left
        };
    }
}
