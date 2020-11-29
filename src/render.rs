use crate::dom::{Dom, Element};
use crate::resource::Resource;
use crate::utils::{div, layer, rgb, size, xy};

pub(crate) struct Render {}

impl Render {
    pub(crate) fn new() -> Self {
        Render {}
    }

    pub(crate) fn render_buffer(
        &mut self,
        dom: &Dom,
        resource: &mut Resource,
        focus_id: Option<usize>,
    ) -> Vec<f32> {
        let mut buffer = vec![];

        for x in &dom.vec {
            let get_dom = &dom;
            let bound = get_dom.get(*x);

            if let Element::Input = bound.element {
                let input = get_dom.get_input(*x);

                buffer.append(&mut div(
                    xy(bound.x, bound.y),
                    size(bound.width, bound.height),
                    rgb(241.0, 243.0, 244.0),
                    layer(0.1),
                    bound.width,
                    bound.height,
                    false,
                ));

                let mut tx = bound.x - input.push_left;
                let s: f32 = 0.5;
                let mut ty = bound.y + 30.0 * s;

                for l in input.value.chars() {
                    let is_empty = l == ' ';

                    let measure = get_dom.ddom.font.get(l.to_string());
                    if !is_empty {
                        let get_layer = resource.get_layer_id(&measure.path);

                        let mut x2 = (tx - (measure.originX * s));
                        let mut y2 = (ty - (measure.originY * s));

                        buffer.append(&mut div(
                            xy(x2, y2),
                            size(measure.width * s, measure.height * s),
                            rgb(0.0, 0.0, 0.0),
                            layer(get_layer),
                            measure.width,
                            measure.height,
                            true,
                        ));
                    }
                    tx = (tx + (measure.advance * s));
                }
                //println!("utils {:?}", (tx - 10.0) + input.push_left);

                if focus_id == Some(*x) {
                    buffer.append(&mut div(
                        xy(bound.x + input.cursor_pos, bound.y),
                        size(1.0, 30.0 * s),
                        rgb(6.0, 95.0, 212.0),
                        layer(0.1),
                        bound.width,
                        bound.height,
                        false,
                    ));
                }
            }
        }

        buffer
    }
}
