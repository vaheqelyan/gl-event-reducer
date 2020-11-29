extern crate glfw;

use cgmath::Matrix;

use glfw::{Action, Context, Key};
use std::sync::mpsc::{channel, Receiver, Sender};

use gl::types::*;

use std::cell::Cell;
use std::cell::RefCell;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::rc::Rc;
use std::str;

use crate::resource::Resource;

use crate::boot::{App, Event, EventFlow};
use crate::gl_core::Gl;

const RGB_FACTOR: f32 = 1.0 / 255.0;

const X: f32 = 0.0;
const Y: f32 = 0.0;

const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 300.0;

pub struct Window {
    glfw: glfw::Glfw,
    window_w: u32,
    window_h: u32,
    gl: Gl,
    pub app: App,
    pub resource: Resource,
}

impl Window {
    pub fn new(window_w: u32, window_h: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        Window {
            glfw,
            window_w,
            window_h,
            gl: Gl::new(),
            app: App::new(),
            resource: Resource::new(),
        }
    }
    pub fn run(&mut self) {
        self.glfw
            .window_hint(glfw::WindowHint::ContextVersion(3, 3));
        self.glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        self.glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        #[cfg(target_os = "macos")]
        self.glfw
            .window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // let doc = self.document;
        // glfw window creation
        // --------------------
        let (mut window, events) = self
            .glfw
            .create_window(
                self.window_w,
                self.window_h,
                "LearnOpenGL",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_all_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

        self.resource.load();

        self.gl
            .setup(&self.resource, self.window_w as f32, self.window_h as f32);

        self.app
            .dispatch(Event::Ready, &mut self.gl, &mut self.resource);

        let bah = &self.app;
        while !window.should_close() {
            //self.glfw.wait_events();
            self.process_events(&mut window, &events);

            unsafe {
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::BindVertexArray(self.gl.vao);
                gl::DrawElements(gl::TRIANGLES, 100_000, gl::UNSIGNED_INT, ptr::null());
            }

            window.swap_buffers();
            self.glfw.poll_events();
        }
    }
    fn process_events(
        &mut self,
        window: &mut glfw::Window,
        events: &Receiver<(f64, glfw::WindowEvent)>,
    ) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    self.app.cursor.x = x as f32;
                    self.app.cursor.y = y as f32;
                    self.app.dispatch_event(
                        EventFlow::PointerMove,
                        &mut self.gl,
                        &mut self.resource,
                    )
                }
                glfw::WindowEvent::Char(character) => {
                    self.app.dispatch_event(
                        EventFlow::Type(character),
                        &mut self.gl,
                        &mut self.resource,
                    );
                }

                glfw::WindowEvent::Key(key, scancode, keymod, mods) => match key {
                    glfw::Key::Backspace => match keymod {
                        Action::Repeat | Action::Press => self.app.dispatch_event(
                            EventFlow::Backspace,
                            &mut self.gl,
                            &mut self.resource,
                        ),
                        _ => (),
                    },
                    glfw::Key::Left => match keymod {
                        Action::Repeat | Action::Press => self.app.dispatch_event(
                            EventFlow::Left,
                            &mut self.gl,
                            &mut self.resource,
                        ),
                        _ => (),
                    },
                    glfw::Key::Right => match keymod {
                        Action::Repeat | Action::Press => self.app.dispatch_event(
                            EventFlow::Right,
                            &mut self.gl,
                            &mut self.resource,
                        ),
                        _ => (),
                    },
                    _ => (),
                },
                glfw::WindowEvent::MouseButton(btn, action, ..) => match action {
                    Action::Press => {
                        self.app.dispatch_event(
                            EventFlow::PointerDown,
                            &mut self.gl,
                            &mut self.resource,
                        );
                    }
                    Action::Release => {
                        self.app.dispatch_event(
                            EventFlow::PointerUp,
                            &mut self.gl,
                            &mut self.resource,
                        );
                    }
                    Action::Repeat => {}
                },
                glfw::WindowEvent::Scroll(x, y) => {}
                _ => {}
            }
        }
    }
}
