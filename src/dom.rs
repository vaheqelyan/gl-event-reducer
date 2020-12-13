use crate::dom_db::DomDB;
use crate::low_dom::{Element, ElementMetaData, LowDom};
use crate::style::{Dimension, Display, Overflow, Style};
pub(crate) struct Dom {
    pub(crate) low_dom: LowDom,
    pub(crate) dom_db: DomDB,
}

impl Dom {
    pub(crate) fn new() -> Self {
        Dom {
            low_dom: LowDom::new(),
            dom_db: DomDB::new(),
        }
    }

    pub fn input(&mut self, style: Style) -> usize {
        let id = self.low_dom.create(ElementMetaData {
            element_type: Element::Input,
            belong_to_screen: 0,
            scroll: false,
        });
        self.dom_db.register_input(id, style);
        id
    }
    pub fn div(&mut self, style: Style) -> usize {
        if let Overflow::Scroll = style.overflow {
            let id = self.low_dom.create(ElementMetaData {
                element_type: Element::Div,
                belong_to_screen: 0,
                scroll: true,
            });

            let scroll_body = self.low_dom.create(ElementMetaData {
                element_type: Element::Div,
                belong_to_screen: 0,
                scroll: true,
            });

            let thumb_line = self.low_dom.create(ElementMetaData {
                element_type: Element::Div,
                belong_to_screen: 0,
                scroll: true,
            });

            let thumb = self.low_dom.create(ElementMetaData {
                element_type: Element::Div,
                belong_to_screen: 0,
                scroll: true,
            });

            self.dom_db.register_div(id, style, &mut self.low_dom);
            self.dom_db.register_div(
                scroll_body,
                Style {
                    width: Dimension::Grow(1.0),
                    height: Dimension::Perc(100.0),
                    bg_color: [255.0, 255.0, 255.0],
                    ..Default::default()
                },
                &mut self.low_dom,
            );
            self.dom_db.register_div(
                thumb_line,
                Style {
                    width: Dimension::Px(10.0),
                    height: Dimension::Perc(100.0),
                    bg_color: [218.0, 220.0, 224.0],
                    ..Default::default()
                },
                &mut self.low_dom,
            );
            self.dom_db.register_div(
                thumb,
                Style {
                    width: Dimension::Px(10.0),
                    height: Dimension::Px(10.0),
                    bg_color: [154.0, 160.0, 166.0],
                    ..Default::default()
                },
                &mut self.low_dom,
            );

            println!("{:?}", self.low_dom.vec);

            self.append(scroll_body, id);
            self.append(thumb_line, id);
            self.append(thumb, thumb_line);

            id
        } else {
            let id = self.low_dom.create(ElementMetaData {
                element_type: Element::Div,
                belong_to_screen: 0,
                scroll: false,
            });

            self.dom_db.register_div(id, style, &mut self.low_dom);
            id
        }
    }

    pub fn get_papa(&self) -> usize {
        0
    }

    pub fn make(&mut self, width: f32, height: f32) {
        let id = self.low_dom.create(ElementMetaData {
            element_type: Element::Div,
            belong_to_screen: 0,
            scroll: true,
        });
        self.dom_db.register_div(
            id,
            Style {
                width: Dimension::Px(width),
                height: Dimension::Px(height),
                ..Default::default()
            },
            &mut self.low_dom,
        );

        //self.layout.viewport(width, height, &mut self.ddom);
    }

    pub fn append(&mut self, element: usize, parent: usize) {
        self.low_dom.append(element, parent);
    }
}
