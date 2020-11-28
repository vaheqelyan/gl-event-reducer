use std::collections::HashMap;
use std::path::Path;

use image::GenericImageView;

#[derive(Debug, Clone)]
pub struct File {
    pub id: u32,
    pub layer: f32,
    pub src: String,
    pub bytes: Vec<u8>,

    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Resource {
    vec: Vec<File>,
    map: HashMap<String, File>,
    id: f32,
    loaded: bool,
}

fn init_resources() -> HashMap<String, File> {
    let mut resources = HashMap::new();
    resources.insert(
        "text/48.png".to_string(),
        File {
            id: 0,
            layer: 0.0,
            src: "text/48.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/49.png".to_string(),
        File {
            id: 0,
            layer: 1.0,
            src: "text/49.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/50.png".to_string(),
        File {
            id: 0,
            layer: 2.0,
            src: "text/50.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/51.png".to_string(),
        File {
            id: 0,
            layer: 3.0,
            src: "text/51.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/52.png".to_string(),
        File {
            id: 0,
            layer: 4.0,
            src: "text/52.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/53.png".to_string(),
        File {
            id: 0,
            layer: 5.0,
            src: "text/53.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/54.png".to_string(),
        File {
            id: 0,
            layer: 6.0,
            src: "text/54.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/55.png".to_string(),
        File {
            id: 0,
            layer: 7.0,
            src: "text/55.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/56.png".to_string(),
        File {
            id: 0,
            layer: 8.0,
            src: "text/56.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/57.png".to_string(),
        File {
            id: 0,
            layer: 9.0,
            src: "text/57.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "".to_string(),
        File {
            id: 0,
            layer: 10.0,
            src: "".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/33.png".to_string(),
        File {
            id: 0,
            layer: 11.0,
            src: "text/33.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/34.png".to_string(),
        File {
            id: 0,
            layer: 12.0,
            src: "text/34.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/35.png".to_string(),
        File {
            id: 0,
            layer: 13.0,
            src: "text/35.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/36.png".to_string(),
        File {
            id: 0,
            layer: 14.0,
            src: "text/36.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/37.png".to_string(),
        File {
            id: 0,
            layer: 15.0,
            src: "text/37.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/38.png".to_string(),
        File {
            id: 0,
            layer: 16.0,
            src: "text/38.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/39.png".to_string(),
        File {
            id: 0,
            layer: 17.0,
            src: "text/39.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/40.png".to_string(),
        File {
            id: 0,
            layer: 18.0,
            src: "text/40.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/41.png".to_string(),
        File {
            id: 0,
            layer: 19.0,
            src: "text/41.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/42.png".to_string(),
        File {
            id: 0,
            layer: 20.0,
            src: "text/42.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/43.png".to_string(),
        File {
            id: 0,
            layer: 21.0,
            src: "text/43.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/44.png".to_string(),
        File {
            id: 0,
            layer: 22.0,
            src: "text/44.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/45.png".to_string(),
        File {
            id: 0,
            layer: 23.0,
            src: "text/45.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/46.png".to_string(),
        File {
            id: 0,
            layer: 24.0,
            src: "text/46.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/47.png".to_string(),
        File {
            id: 0,
            layer: 25.0,
            src: "text/47.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/58.png".to_string(),
        File {
            id: 0,
            layer: 26.0,
            src: "text/58.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/59.png".to_string(),
        File {
            id: 0,
            layer: 27.0,
            src: "text/59.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/60.png".to_string(),
        File {
            id: 0,
            layer: 28.0,
            src: "text/60.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/61.png".to_string(),
        File {
            id: 0,
            layer: 29.0,
            src: "text/61.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/62.png".to_string(),
        File {
            id: 0,
            layer: 30.0,
            src: "text/62.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/63.png".to_string(),
        File {
            id: 0,
            layer: 31.0,
            src: "text/63.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/64.png".to_string(),
        File {
            id: 0,
            layer: 32.0,
            src: "text/64.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/65.png".to_string(),
        File {
            id: 0,
            layer: 33.0,
            src: "text/65.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/66.png".to_string(),
        File {
            id: 0,
            layer: 34.0,
            src: "text/66.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/67.png".to_string(),
        File {
            id: 0,
            layer: 35.0,
            src: "text/67.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/68.png".to_string(),
        File {
            id: 0,
            layer: 36.0,
            src: "text/68.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/69.png".to_string(),
        File {
            id: 0,
            layer: 37.0,
            src: "text/69.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/70.png".to_string(),
        File {
            id: 0,
            layer: 38.0,
            src: "text/70.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/71.png".to_string(),
        File {
            id: 0,
            layer: 39.0,
            src: "text/71.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/72.png".to_string(),
        File {
            id: 0,
            layer: 40.0,
            src: "text/72.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/73.png".to_string(),
        File {
            id: 0,
            layer: 41.0,
            src: "text/73.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/74.png".to_string(),
        File {
            id: 0,
            layer: 42.0,
            src: "text/74.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/75.png".to_string(),
        File {
            id: 0,
            layer: 43.0,
            src: "text/75.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/76.png".to_string(),
        File {
            id: 0,
            layer: 44.0,
            src: "text/76.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/77.png".to_string(),
        File {
            id: 0,
            layer: 45.0,
            src: "text/77.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/78.png".to_string(),
        File {
            id: 0,
            layer: 46.0,
            src: "text/78.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/79.png".to_string(),
        File {
            id: 0,
            layer: 47.0,
            src: "text/79.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/80.png".to_string(),
        File {
            id: 0,
            layer: 48.0,
            src: "text/80.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/81.png".to_string(),
        File {
            id: 0,
            layer: 49.0,
            src: "text/81.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/82.png".to_string(),
        File {
            id: 0,
            layer: 50.0,
            src: "text/82.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/83.png".to_string(),
        File {
            id: 0,
            layer: 51.0,
            src: "text/83.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/84.png".to_string(),
        File {
            id: 0,
            layer: 52.0,
            src: "text/84.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/85.png".to_string(),
        File {
            id: 0,
            layer: 53.0,
            src: "text/85.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/86.png".to_string(),
        File {
            id: 0,
            layer: 54.0,
            src: "text/86.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/87.png".to_string(),
        File {
            id: 0,
            layer: 55.0,
            src: "text/87.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/88.png".to_string(),
        File {
            id: 0,
            layer: 56.0,
            src: "text/88.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/89.png".to_string(),
        File {
            id: 0,
            layer: 57.0,
            src: "text/89.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/90.png".to_string(),
        File {
            id: 0,
            layer: 58.0,
            src: "text/90.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/91.png".to_string(),
        File {
            id: 0,
            layer: 59.0,
            src: "text/91.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/92.png".to_string(),
        File {
            id: 0,
            layer: 60.0,
            src: "text/92.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/93.png".to_string(),
        File {
            id: 0,
            layer: 61.0,
            src: "text/93.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/94.png".to_string(),
        File {
            id: 0,
            layer: 62.0,
            src: "text/94.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/95.png".to_string(),
        File {
            id: 0,
            layer: 63.0,
            src: "text/95.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/96.png".to_string(),
        File {
            id: 0,
            layer: 64.0,
            src: "text/96.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/97.png".to_string(),
        File {
            id: 0,
            layer: 65.0,
            src: "text/97.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/98.png".to_string(),
        File {
            id: 0,
            layer: 66.0,
            src: "text/98.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/99.png".to_string(),
        File {
            id: 0,
            layer: 67.0,
            src: "text/99.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/100.png".to_string(),
        File {
            id: 0,
            layer: 68.0,
            src: "text/100.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/101.png".to_string(),
        File {
            id: 0,
            layer: 69.0,
            src: "text/101.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/102.png".to_string(),
        File {
            id: 0,
            layer: 70.0,
            src: "text/102.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/103.png".to_string(),
        File {
            id: 0,
            layer: 71.0,
            src: "text/103.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/104.png".to_string(),
        File {
            id: 0,
            layer: 72.0,
            src: "text/104.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/105.png".to_string(),
        File {
            id: 0,
            layer: 73.0,
            src: "text/105.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/106.png".to_string(),
        File {
            id: 0,
            layer: 74.0,
            src: "text/106.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/107.png".to_string(),
        File {
            id: 0,
            layer: 75.0,
            src: "text/107.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/108.png".to_string(),
        File {
            id: 0,
            layer: 76.0,
            src: "text/108.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/109.png".to_string(),
        File {
            id: 0,
            layer: 77.0,
            src: "text/109.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/110.png".to_string(),
        File {
            id: 0,
            layer: 78.0,
            src: "text/110.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/111.png".to_string(),
        File {
            id: 0,
            layer: 79.0,
            src: "text/111.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/112.png".to_string(),
        File {
            id: 0,
            layer: 80.0,
            src: "text/112.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/113.png".to_string(),
        File {
            id: 0,
            layer: 81.0,
            src: "text/113.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/114.png".to_string(),
        File {
            id: 0,
            layer: 82.0,
            src: "text/114.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/115.png".to_string(),
        File {
            id: 0,
            layer: 83.0,
            src: "text/115.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/116.png".to_string(),
        File {
            id: 0,
            layer: 84.0,
            src: "text/116.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/117.png".to_string(),
        File {
            id: 0,
            layer: 85.0,
            src: "text/117.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/118.png".to_string(),
        File {
            id: 0,
            layer: 86.0,
            src: "text/118.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/119.png".to_string(),
        File {
            id: 0,
            layer: 87.0,
            src: "text/119.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/120.png".to_string(),
        File {
            id: 0,
            layer: 88.0,
            src: "text/120.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/121.png".to_string(),
        File {
            id: 0,
            layer: 89.0,
            src: "text/121.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/122.png".to_string(),
        File {
            id: 0,
            layer: 90.0,
            src: "text/122.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/123.png".to_string(),
        File {
            id: 0,
            layer: 91.0,
            src: "text/123.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/124.png".to_string(),
        File {
            id: 0,
            layer: 92.0,
            src: "text/124.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/125.png".to_string(),
        File {
            id: 0,
            layer: 93.0,
            src: "text/125.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );
    resources.insert(
        "text/126.png".to_string(),
        File {
            id: 0,
            layer: 94.0,
            src: "text/126.png".to_string(),
            bytes: Vec::new(),
            width: 0,
            height: 0,
        },
    );

    resources
}

impl Resource {
    pub fn new() -> Self {
        let x = init_resources();
        Resource {
            vec: Vec::new(),
            map: x,
            id: 98.0,
            loaded: false,
        }
    }

    pub fn get_layer_id(&self, key: &String) -> f32 {
        self.map.get(key).unwrap().layer
    }

    pub fn get(&self) -> &HashMap<String, File> {
        &self.map
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn load(&mut self) {
        if !self.loaded {
            for (key, file) in &mut self.map {

                //println!("{:?} {:?}", Path::new(&file.src).exists(), file.src);
                if &file.src.is_empty() != &true {
                    let img = image::open(&Path::new(&file.src)).unwrap();

                    let to_rgba = img.to_rgba();
                    let data = to_rgba.into_vec();
                    file.bytes = data;
                    file.width = img.width() as i32;
                    file.height = img.height() as i32;
                }
            }
            self.loaded = true;
        }
    }
}
