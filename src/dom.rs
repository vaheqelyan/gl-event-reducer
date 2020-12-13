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
        let id = self.low_dom.create(ElementMetaData {
            element_type: Element::Div,
            belong_to_screen: 0,
            scroll: match style.overflow {
                Overflow::Scroll => true,
                _ => false,
            },
        });

        self.dom_db.register_div(id, style);

        id
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
        );

        //self.layout.viewport(width, height, &mut self.ddom);
    }

    pub fn append(&mut self, element: usize, parent: usize) {
        self.low_dom.append(element, parent);
    }
}
