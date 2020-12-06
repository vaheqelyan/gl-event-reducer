extern crate nalgebra_glm as glm;
const RGB_FACTOR: f32 = 1.0 / 255.0;

use num_traits::{clamp_max, clamp_min};

extern crate gl;
use self::gl::types::*;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

use crate::boot::Cursor;
use crate::dom::{Dom, Element, ElementMetaData};
use crate::font::Font;

use crate::resource::Resource;
use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn create_shader(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
    // build and compile our shader program
    // ------------------------------------
    // vertex shader
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }

        // fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        // check for shader compile errors
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }

        // link shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        // check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}

pub fn make_index(elements_count: u32) -> Vec<u32> {
    let mut n: u32 = 0;

    let mut x: Vec<u32> = Vec::with_capacity((elements_count as usize) * 7);

    for _ in 0..elements_count {
        x.append(&mut vec![n, n + 1, n + 2, n + 2, n + 3, n, 0xFFFF]);

        n = n + 4;
    }

    x
}

pub(crate) fn c(u: f32, v: f32, w: f32, h: f32, x: bool) -> (f32, f32) {
    (u / (325.0 / w), v / (325.0 / h))
}

pub(crate) fn rgb(r: f32, g: f32, b: f32) -> [f32; 3] {
    [r, g, b]
}

pub(crate) fn xy(x: f32, y: f32) -> [f32; 2] {
    [x, y]
}

pub(crate) fn size(w: f32, h: f32) -> [f32; 2] {
    [w, h]
}

pub(crate) fn layer(l: f32) -> f32 {
    l
}

pub(crate) fn div(
    pos: [f32; 2],
    size: [f32; 2],
    rgb: [f32; 3],
    l: f32,
    original_w: f32,
    original_h: f32,
    xx: bool,
) -> Vec<f32> {
    let r = rgb[0] * RGB_FACTOR;
    let g = rgb[1] * RGB_FACTOR;
    let b = rgb[2] * RGB_FACTOR;
    let layer = l;

    let x = pos[0];
    let y = pos[1];

    let w = size[0];
    let h = size[1];

    let uv1 = c(0.0, 0.0, original_w, original_h, xx);
    let uv2 = c(1.0, 0.0, original_w, original_h, xx);
    let uv3 = c(1.0, 1.0, original_w, original_h, xx);
    let uv4 = c(0.0, 1.0, original_w, original_h, xx);

    /*vec![
        t1.x, t1.y, uv1.0, uv1.0, layer, r, g, b, // ---
        t2.x, t2.y, uv2.0, uv2.1, layer, r, g, b, // ---
        t3.x, t3.y, uv3.0, uv3.1, layer, r, g, b, // ---
        t4.x, t4.y, uv4.0, uv4.1, layer, r, g, b,
    ]*/

    vec![
        x,
        y,
        uv1.0,
        uv1.1,
        layer,
        r,
        g,
        b,
        x + w,
        y,
        uv2.0,
        uv2.1,
        layer,
        r,
        g,
        b,
        x + w,
        y + h,
        uv3.0,
        uv3.1,
        layer,
        r,
        g,
        b,
        x,
        y + h,
        uv4.0,
        uv4.1,
        layer,
        r,
        g,
        b,
    ]
}

/*pub fn find_bound_xy(cursor: &Cursor, dom: &Dom) -> Option<usize> {
    let mut id: Option<usize> = None;

    for x in &dom.vec {
        let get_dom = &dom;
        let bound = get_dom.get(*x);

        if cursor.x >= bound.x
            && cursor.x <= bound.x + bound.width
            && cursor.y >= bound.y
            && cursor.y <= bound.y + bound.height
        {
            id = Some(*x);
        }
    }

    id
}*/
