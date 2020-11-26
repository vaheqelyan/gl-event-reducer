extern crate mylib;
use mylib::window;

fn main() {
    let mut window = window::Window::new(300, 616);
    window.run();
}
