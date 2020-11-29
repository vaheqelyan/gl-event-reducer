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

pub struct Boot {
    dom: Dom,
    pub cursor: Cursor,
    pub focus: Option<usize>,
    pub event_state: EventState,
}

pub enum Event {
    Ready,
}

pub struct EventState {
    pointerdown: bool,
    pointermove: bool,
    pointerup: bool,
}

pub enum EventFlow {
    Type(char),
    Backspace,
    Left,
    Right,

    PointerDown,
    PointerMove,
    PointerUp,
}

impl Boot {
    pub fn new() -> Self {
        Boot {
            dom: Dom::new(),
            cursor: Cursor { x: 0.0, y: 0.0 },
            focus: None,
            event_state: EventState {
                pointerdown: false,
                pointermove: false,
                pointerup: false,
            },
        }
    }

    pub fn reducer(&mut self, event: Event, gl: &mut Gl, resource: &mut Resource) {
        match event {
            Event::Ready => {
                let one = self.dom.create(Bound {
                    x: 10.0,
                    y: 10.0,
                    width: 180.0,
                    height: 32.0,
                    element: Element::Input,
                });

                let one = self.dom.create(Bound {
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

    pub fn dispatch(&mut self, event: Event, gl: &mut Gl, resource: &mut Resource) {
        self.reducer(event, gl, resource);
    }

    pub fn reflow(&self) {}

    pub fn dispatch_event(&mut self, event: EventFlow, gl: &mut Gl, resource: &mut Resource) {
        match event {
            EventFlow::Type(key) => {
                if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.input(key, 0, &self.focus.unwrap(), bound);
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::Backspace => {
                if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.backspace(&self.focus.unwrap(), bound);
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::Left => {
                if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.cursor_left(&self.focus.unwrap(), bound);
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::Right => {
                if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.cursor_right(&self.focus.unwrap(), bound);
                }
                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::PointerDown => {
                self.event_state.pointerdown = true;
                let element_id = find_bound_xy(&self.cursor, &self.dom);
                if element_id != None {
                    let mut borrow_dom = &mut self.dom;
                    let element = borrow_dom.get(element_id.unwrap());
                    let bound = borrow_dom.get(element_id.unwrap());
                    let width = bound.width;
                    let x = bound.x;
                    let y = bound.y;
                    if let Element::Input = element.element {
                        self.focus = element_id;
                        borrow_dom
                            .ddom
                            .focus(&self.focus.unwrap(), width, x, y, &self.cursor);
                    }
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::PointerMove => {
                self.event_state.pointermove = true;
                if self.focus != None && self.event_state.pointerdown == true {
                    let mut borrow_dom = &mut self.dom;

                    let element = borrow_dom.get(self.focus.unwrap());
                    let bound = borrow_dom.get(self.focus.unwrap());
                    let width = bound.width;
                    let x = bound.x;
                    let y = bound.y;

                    borrow_dom
                        .ddom
                        .select(&self.focus.unwrap(), width, x, y, &self.cursor);
                }

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            EventFlow::PointerUp => {
                self.event_state.pointerdown = false;
                self.event_state.pointermove = false;
                self.focus = find_bound_xy(&self.cursor, &self.dom);

                gl.draw(generate(&self.dom, resource, self.focus));
            }

            _ => (),
        }
    }
}
