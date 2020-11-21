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

use crate::app::Cursor;
use crate::dom::{Bound, Dom, Element};
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

pub(crate) fn c(u: f32, v: f32, w: f32, h: f32) -> (f32, f32) {
    //(u,v)
    //(u/5.0,v/5.0)
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
    matrix: &glm::TMat4<f32>,
    original_w: f32,
    original_h: f32,
) -> Vec<f32> {
    let r = rgb[0] * RGB_FACTOR;
    let g = rgb[1] * RGB_FACTOR;
    let b = rgb[2] * RGB_FACTOR;
    let layer = l;

    let x = pos[0];
    let y = pos[1];

    let w = size[0];
    let h = size[1];

    let t1 = matrix * glm::vec4(x, y, 0.0, 1.0);
    let t2 = matrix * glm::vec4(x + w, y, 0.0, 1.0);
    let t3 = matrix * glm::vec4(x + w, y + h, 0.0, 1.0);
    let t4 = matrix * glm::vec4(x, y + h, 0.0, 1.0);

    let uv1 = c(0.0, 0.0, original_w, original_h);
    let uv2 = c(1.0, 0.0, original_w, original_h);
    let uv3 = c(1.0, 1.0, original_w, original_h);
    let uv4 = c(0.0, 1.0, original_w, original_h);

    vec![
        t1.x, t1.y, uv1.0, uv1.0, layer, r, g, b, // ---
        t2.x, t2.y, uv2.0, uv2.1, layer, r, g, b, // ---
        t3.x, t3.y, uv3.0, uv3.1, layer, r, g, b, // ---
        t4.x, t4.y, uv4.0, uv4.1, layer, r, g, b,
    ]
}

pub fn count_chars(input: &String, cursor: usize, font: &Font, container: f32) -> f32 {
    let mut size: f32 = 0.0;
    for c in input.chars() {
        let measure = font.get(c.to_string());
        size += (measure.advance * 0.07);
    }

    clamp_min(size - container, 0.0)
}

pub fn generate(
    dom: &Rc<RefCell<Dom>>,
    resource: &mut Resource,
    focus_id: Option<usize>,
) -> Vec<f32> {
    let mut buffer = vec![];

    let matrix = glm::ortho(0.0, 300 as f32, 500 as f32, 0.0, -1.0, 1.0);

    for x in &dom.borrow().vec {
        let get_dom = dom.borrow();
        let bound = get_dom.get(*x);

        if let Element::Input = bound.element {
            let input = get_dom.get_input(*x);

            buffer.append(&mut div(
                xy(bound.x, bound.y),
                size(bound.width, bound.height),
                rgb(179.0, 179.0, 255.0),
                layer(0.1),
                &matrix,
                bound.width,
                bound.height,
            ));

            let mut tx = bound.x - input.push_left;
            let s: f32 = 0.07;
            let mut ty = bound.y + 192.0 * s;

            for l in input.value.chars() {
                let is_empty = l == ' ';
                let measure = get_dom.ddom.font.get(l.to_string());
                let get_layer = if is_empty {
                    0.1
                } else {
                    resource.get_layer_id(&measure.path)
                };

                let mut x2 = (tx - (measure.originX * s));
                let mut y2 = (ty - (measure.originY * s));

                buffer.append(&mut div(
                    xy(x2, y2),
                    size(measure.width * s, measure.height * s),
                    rgb(0.0, 0.0, 0.0),
                    layer(get_layer),
                    &matrix,
                    measure.width,
                    measure.height,
                ));

                tx = (tx + (measure.advance * s)).round();
            }

            if focus_id == Some(*x) {
                buffer.append(&mut div(
                    xy(bound.x + input.cursor_pos, bound.y),
                    size(1.0, 192.0 * s),
                    rgb(6.0, 95.0, 212.0),
                    layer(0.1),
                    &matrix,
                    bound.width,
                    bound.height,
                ));
            }
        }
    }

    buffer
}

pub fn find_bound_xy(cursor: &Cursor, dom: &Rc<RefCell<Dom>>) -> Option<usize> {
    let mut id: Option<usize> = None;

    for x in &dom.borrow().vec {
        let get_dom = dom.borrow();
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
}
