use crate::dom::{Dom, Element, ElementMetaData};
use crate::gl_core::Gl;

use crate::render::Render;
use crate::resource::Resource;
//use crate::utils::find_bound_xy;
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
    render: Render,
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

    Init(f32, f32),

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
            render: Render::new(),
        }
    }

    pub fn reducer(&mut self, event: Event, gl: &mut Gl, resource: &mut Resource) {
        match event {
            Event::Ready => {
                use crate::style::{Dimension, Display, Overflow, Style};

                let container = self.dom.div(Style {
                    width: Dimension::Perc(100.0),
                    height: Dimension::Perc(100.0),
                    bg_color: [233.0, 233.0, 233.0],
                    ..Default::default()
                });

                let col1 = self.dom.div(Style {
                    width: Dimension::Perc(33.3),
                    height: Dimension::Perc(50.0),
                    bg_color: [165.0, 105.0, 80.0],
                    display: Display::InlineBlock,
                    ..Default::default()
                });

                let col2 = self.dom.div(Style {
                    width: Dimension::Perc(33.3),
                    height: Dimension::Perc(50.0),
                    bg_color: [249.0, 163.0, 91.0],
                    display: Display::InlineBlock,
                    ..Default::default()
                });

                let col3 = self.dom.div(Style {
                    width: Dimension::Perc(33.3),
                    height: Dimension::Px(100.0),
                    bg_color: [236.0, 115.0, 121.0],
                    overflow: Overflow::Scroll,
                    display: Display::InlineBlock,
                    ..Default::default()
                });

                let block1 = self.dom.div(Style {
                    width: Dimension::Perc(100.0),
                    height: Dimension::Px(200.0),
                    bg_color: [40.0, 26.0, 21.0],
                    ..Default::default()
                });

                let block2 = self.dom.div(Style {
                    width: Dimension::Perc(50.0),
                    height: Dimension::Px(100.0),
                    bg_color: [57.0, 64.0, 92.0],
                    top: Dimension::Px(500.0),
                    ..Default::default()
                });

                let block3 = self.dom.div(Style {
                    width: Dimension::Perc(50.0),
                    height: Dimension::Px(100.0),
                    bg_color: [0.0, 157.0, 255.0],
                    ..Default::default()
                });

                let block4 = self.dom.div(Style {
                    width: Dimension::Perc(50.0),
                    height: Dimension::Px(100.0),
                    bg_color: [255.0, 3.0, 255.0],
                    ..Default::default()
                });

                /*let block1_1 = self.dom.div(Style {
                    width: Dimension::Px(10.0),
                    height: Dimension::Px(100.0),
                    bg_color: [0.0, 0.0, 0.0],
                    right: Dimension::Perc(50.0),
                    ..Default::default()
                });*/

                let body = self.dom.get_top_screen();

                self.dom.append(col1, container);
                self.dom.append(col2, container);
                self.dom.append(col3, container);

                self.dom.append(block1, col3);

                //self.dom.append(block1_1, block2);
                self.dom.append(block2, col3);
                self.dom.append(block3, col3);
                self.dom.append(block4, col3);
                self.dom.append(container, body);

                self.dom.layout();

                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));

                //gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));
            }
        }
    }

    pub fn dispatch(&mut self, event: Event, gl: &mut Gl, resource: &mut Resource) {
        self.reducer(event, gl, resource);
    }

    pub fn reflow(&self) {}

    pub fn dispatch_event(&mut self, event: EventFlow, gl: &mut Gl, resource: &mut Resource) {
        match event {
            EventFlow::Init(width, height) => {
                self.dom.top_screen(width, height);
            }

            EventFlow::Type(key) => {
                /*if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.input(key, 0, &self.focus.unwrap(), bound);
                }

                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));*/
            }

            EventFlow::Backspace => {
                /*if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.backspace(&self.focus.unwrap(), bound);
                }

                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));*/
            }

            EventFlow::Left => {
                /*if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.cursor_left(&self.focus.unwrap(), bound);
                }

                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));*/
            }

            EventFlow::Right => {
                /*if self.focus != None {
                    let mut borrow_dom = &mut self.dom;
                    let bound = borrow_dom.get(self.focus.unwrap()).width;

                    borrow_dom.ddom.cursor_right(&self.focus.unwrap(), bound);
                }
                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));*/
            }

            EventFlow::PointerDown => {
                /*self.event_state.pointerdown = true;
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

                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));*/
            }

            EventFlow::PointerMove => {
                /*self.event_state.pointermove = true;
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

                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));*/
            }

            EventFlow::PointerUp => {
                /*self.event_state.pointerdown = false;
                self.event_state.pointermove = false;
                self.focus = find_bound_xy(&self.cursor, &self.dom);

                gl.draw(self.render.render_buffer(&self.dom, resource, self.focus));*/
            }

            _ => (),
        }
    }
}
