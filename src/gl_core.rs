extern crate gl;

use crate::resource::Resource;
use crate::shader::{FRAGMENT_SOURCE, VERTEX_SOURCE};
use crate::utils::{create_shader, make_index};

use self::gl::types::*;
use cgmath::Matrix;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

const SCR_WIDTH: u32 = 300;
const SCR_HEIGHT: u32 = 616;

const X: f32 = 0.0;
const Y: f32 = 0.0;

const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 300.0;

pub struct Gl {
    pub(crate) vao: u32,
    pub(crate) index_buffer: Vec<u32>,
    pub(crate) vertex_buffer: Vec<f32>,
    pub(crate) pointer: Option<*mut std::ffi::c_void>,
}

impl Gl {
    pub fn new() -> Gl {
        let p: *const i32 = ptr::null();

        let mut vertex: Vec<f32> = Vec::with_capacity(32_000);

        for i in 0..32_000 / 32 {
            vertex.append(&mut vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // ---
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]);
        }

        Gl {
            vao: 0,
            vertex_buffer: vertex,
            index_buffer: Vec::new(),
            pointer: None,
        }
    }

    // TODO REWRITE MEMCPY
    pub fn draw(&mut self, mut buffer: Vec<f32>) {
        self.vertex_buffer.clear();

        let mut vertex: Vec<f32> = Vec::with_capacity(32_000);

        for i in 0..32_000 / 32 {
            vertex.append(&mut vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // ---
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]);
        }
        self.vertex_buffer.append(&mut vertex);

        unsafe {
            ptr::copy_nonoverlapping(
                self.vertex_buffer.as_ptr(),
                self.pointer.unwrap() as *mut f32,
                self.vertex_buffer.len(),
            );
        };

        self.vertex_buffer.clear();
        self.vertex_buffer.append(&mut buffer);

        unsafe {
            ptr::copy_nonoverlapping(
                self.vertex_buffer.as_ptr(),
                self.pointer.unwrap() as *mut f32,
                self.vertex_buffer.len(),
            );
        };
    }

    pub fn setup(&mut self, resource: &Resource, w: f32, h: f32) {
        unsafe {
            let shader_program = create_shader(VERTEX_SOURCE, FRAGMENT_SOURCE);

            let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(vao);

            self.vao = vao;

            let map_flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;
            let create_flags = map_flags | gl::DYNAMIC_STORAGE_BIT;

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferStorage(
                gl::ARRAY_BUFFER,
                (self.vertex_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                //&0f32 as *const f32 as *const c_void,
                &self.vertex_buffer[0] as *const f32 as *const c_void,
                create_flags,
            );

            self.index_buffer = make_index(1000);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferStorage(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.index_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                //&0u32 as *const u32 as *const c_void,
                &self.index_buffer[0] as *const u32 as *const c_void,
                create_flags,
            );

            let pointer = gl::MapBufferRange(gl::ARRAY_BUFFER, 0, 32_000, map_flags);
            self.pointer = Some(pointer);

            let stripe = 8 * mem::size_of::<GLfloat>() as GLsizei;
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stripe, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stripe,
                (2 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                stripe,
                (4 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                stripe,
                (5 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(3);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            self.texture(resource);

            gl::UseProgram(shader_program);

            gl::Enable(gl::PRIMITIVE_RESTART);
            gl::PrimitiveRestartIndex(0xFFFF);

            let c_str_vert = CString::new("MVP".as_bytes()).unwrap();

            let model_loc = gl::GetUniformLocation(shader_program, c_str_vert.as_ptr());

            let model = cgmath::ortho(0.0, w, h, 0.0, -1.0, 1.0);

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
        }
    }

    pub(crate) fn texture(&self, resource: &Resource) {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, texture);
            gl::ActiveTexture(gl::TEXTURE0);

            // Create storage for the texture
            gl::TexStorage3D(
                gl::TEXTURE_2D_ARRAY,
                1,         // no mipmaps
                gl::RGBA8, // internal format
                325,
                325, // WxH
                100, //vprops.resource.len() as i32, // Number of layers
            );
        }

        let map = resource.get();
        for (key, value) in resource.get().iter() {
            if value.src.is_empty() != true {
                unsafe {
                    gl::TexSubImage3D(
                        gl::TEXTURE_2D_ARRAY,
                        0, // mipmap number
                        0,
                        0,
                        value.layer as i32, //0 as i32,
                        value.width,
                        value.height,
                        1,
                        gl::RGBA,                                      // format
                        gl::UNSIGNED_BYTE,                             // type
                        &value.bytes[0] as *const u8 as *const c_void, // pointer to data
                    );
                }
            }
        }

        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
        };
    }
}
