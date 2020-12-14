use crate::dom_db::DomDB;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Element {
    Input,
    Div,
    Scroll,
}

#[derive(Debug)]
pub struct ElementMetaData {
    pub(crate) element_type: Element,
    pub(crate) belong_to_screen: usize,
    pub(crate) scroll: bool,
}

#[derive(Debug)]
pub struct LowDom {
    pub vec: Vec<usize>,
    pub map: HashMap<usize, ElementMetaData>,
    pub child_parent: HashMap<usize, usize>,
    id: usize,
}

impl LowDom {
    pub fn new() -> Self {
        LowDom {
            vec: Vec::new(),
            map: HashMap::new(),
            child_parent: HashMap::new(),
            id: 0,
        }
    }
    pub fn get(&self, id: usize) -> &ElementMetaData {
        let val = self.map.get(&id).unwrap();
        val
    }

    pub fn create(&mut self, element_type: ElementMetaData) -> usize {
        self.vec.push(self.id);
        self.map.insert(self.id, element_type);
        self.id += 1;

        self.id - 1
    }

    pub(crate) fn insert(&mut self, element: usize, parent: usize) {
        let meta = self.get(parent);
        let screen_id = meta.belong_to_screen;
        if meta.scroll == true {
            let mut meta_child = self.map.get_mut(&element).unwrap();
            meta_child.belong_to_screen = parent;
        } else {
            let mut meta_child = self.map.get_mut(&element).unwrap();
            meta_child.belong_to_screen = screen_id;
        }

        self.child_parent.insert(element, parent);
    }

    pub(crate) fn append(&mut self, element: usize, parent: usize, dom_db: &mut DomDB) {
        let meta = self.get(parent);
        let screen_id = meta.belong_to_screen;
        if meta.scroll == true {
            let mut meta_child = self.map.get_mut(&element).unwrap();
            meta_child.belong_to_screen = parent;
        } else {
            let mut meta_child = self.map.get_mut(&element).unwrap();
            meta_child.belong_to_screen = screen_id;
        }

        let div = dom_db.div_data.get(&parent).unwrap();
        let point_id = div.append(parent);

        self.child_parent.insert(element, point_id);
    }

    pub fn debug(&self) {
        println!("{:?}", self.vec);
        println!("{:#?}", self.map);
        println!("{:?}", self.child_parent);
    }
}
