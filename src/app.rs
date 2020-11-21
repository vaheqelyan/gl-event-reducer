use crate::dom::{Bound, Dom, Element};
use crate::gl_core::Gl;

use crate::resource::Resource;
use crate::utils::{find_bound_xy, generate};
use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use glfw::{Action, Context, Key};

#[derive(Debug)]
pub struct Cursor {
    pub x: f32,
    pub y: f32,
}

pub struct App {
    dom: Rc<RefCell<Dom>>,
    pub cursor: Cursor,
    pub focus: Option<usize>,
}

pub enum Event {
    Ready,
}

pub enum EventFlow {
    Click,
    Type(char),
    Backspace,
    Left,
    Right,
}

impl App {
    pub fn new() -> Self {
        App {
            dom: Rc::new(RefCell::new(Dom::new())),
            cursor: Cursor { x: 0.0, y: 0.0 },
            focus: None,
        }
    }

    pub fn reducer(&self, event: Event, gl: &mut Gl, resource: &mut Resource) {
        match event {
            Event::Ready => {
                let one = self.dom.borrow_mut().create(Bound {
                    x: 10.0,
                    y: 10.0,
                    width: 180.0,
                    height: 32.0,
                    element: Element::Input,
                });

                let one = self.dom.borrow_mut().create(Bound {
                    x: 10.0,
                    y: 43.0,
                    width: 180.0,
                    height: 32.0,
                    element: Element::Input,
                });

                gl.draw(generate(&self.dom, resource, self.focus));
            }
        }
    }

    pub fn dispatch(&self, event: Event, gl: &mut Gl, resource: &mut Resource) {
        self.reducer(event, gl, resource);
    }

    pub fn reflow(&self) {}

    pub fn dispatch_event(&mut self, event: EventFlow, gl: &mut Gl, resource: &mut Resource) {
        match event {
            EventFlow::Type(key) => {
                if self.focus != None {
                    let mut borrow_dom = self.dom.borrow_mut();
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.input(key, 0, &self.focus.unwrap(), bound);
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::Backspace => {
                if self.focus != None {
                    let mut borrow_dom = self.dom.borrow_mut();
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.backspace(&self.focus.unwrap(), bound);
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::Left => {
                if self.focus != None {
                    let mut borrow_dom = self.dom.borrow_mut();
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.cursor_left(&self.focus.unwrap(), bound);
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::Right => {
                if self.focus != None {
                    let mut borrow_dom = self.dom.borrow_mut();
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.cursor_right(&self.focus.unwrap(), bound);
                }
                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::Click => {
                let element_id = find_bound_xy(&self.cursor, &self.dom);
                if element_id != None {
                    let borrow_dom = self.dom.borrow();
                    let element = borrow_dom.get(element_id.unwrap());
                    if let Element::Input = element.element {
                        self.focus = element_id;
                    }
                }
            }

            _ => (),
        }
    }
}
