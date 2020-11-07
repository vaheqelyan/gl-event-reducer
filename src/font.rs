use std::collections::HashMap;

#[derive(Debug)]
pub struct Char {
    pub width: f32,
    pub height: f32,
    pub originX: f32,
    pub originY: f32,
    pub advance: f32,
    pub path: String,
    pub layer: usize,
}

pub fn get_font() -> HashMap<String, Char> {
    let mut char_list = HashMap::new();
    char_list.insert(
        "0".to_string(),
        Char {
            width: 134.0,
            height: 191.0,
            originX: -3.0,
            originY: 181.0,
            advance: 139.0,
            path: "text/48.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "1".to_string(),
        Char {
            width: 122.0,
            height: 186.0,
            originX: -12.0,
            originY: 179.0,
            advance: 139.0,
            path: "text/49.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "2".to_string(),
        Char {
            width: 128.0,
            height: 188.0,
            originX: -6.0,
            originY: 181.0,
            advance: 139.0,
            path: "text/50.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "3".to_string(),
        Char {
            width: 133.0,
            height: 191.0,
            originX: -3.0,
            originY: 181.0,
            advance: 139.0,
            path: "text/51.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "4".to_string(),
        Char {
            width: 140.0,
            height: 186.0,
            originX: 1.0,
            originY: 179.0,
            advance: 139.0,
            path: "text/52.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "5".to_string(),
        Char {
            width: 133.0,
            height: 188.0,
            originX: -3.0,
            originY: 179.0,
            advance: 139.0,
            path: "text/53.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "6".to_string(),
        Char {
            width: 130.0,
            height: 191.0,
            originX: -6.0,
            originY: 181.0,
            advance: 139.0,
            path: "text/54.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "7".to_string(),
        Char {
            width: 128.0,
            height: 186.0,
            originX: -6.0,
            originY: 179.0,
            advance: 139.0,
            path: "text/55.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "8".to_string(),
        Char {
            width: 132.0,
            height: 191.0,
            originX: -4.0,
            originY: 181.0,
            advance: 139.0,
            path: "text/56.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "9".to_string(),
        Char {
            width: 130.0,
            height: 191.0,
            originX: -5.0,
            originY: 181.0,
            advance: 139.0,
            path: "text/57.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        " ".to_string(),
        Char {
            width: 14.0,
            height: 14.0,
            originX: 7.0,
            originY: 7.0,
            advance: 69.0,
            path: "text/32.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "!".to_string(),
        Char {
            width: 38.0,
            height: 186.0,
            originX: -16.0,
            originY: 179.0,
            advance: 69.0,
            path: "text/33.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        '"'.to_string(),
        Char {
            width: 82.0,
            height: 68.0,
            originX: -4.0,
            originY: 179.0,
            advance: 88.0,
            path: "text/34.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "#".to_string(),
        Char {
            width: 151.0,
            height: 185.0,
            originX: 6.0,
            originY: 178.0,
            advance: 139.0,
            path: "text/35.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "$".to_string(),
        Char {
            width: 146.0,
            height: 216.0,
            originX: 4.0,
            originY: 192.0,
            advance: 139.0,
            path: "text/36.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "%".to_string(),
        Char {
            width: 219.0,
            height: 189.0,
            originX: -2.0,
            originY: 180.0,
            advance: 222.0,
            path: "text/37.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "&".to_string(),
        Char {
            width: 168.0,
            height: 189.0,
            originX: -2.0,
            originY: 180.0,
            advance: 166.0,
            path: "text/38.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "'".to_string(),
        Char {
            width: 37.0,
            height: 68.0,
            originX: -6.0,
            originY: 179.0,
            advance: 47.0,
            path: "text/39.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "(".to_string(),
        Char {
            width: 80.0,
            height: 247.0,
            originX: -9.0,
            originY: 188.0,
            advance: 83.0,
            path: "text/40.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        ")".to_string(),
        Char {
            width: 80.0,
            height: 247.0,
            originX: 5.0,
            originY: 188.0,
            advance: 83.0,
            path: "text/41.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "*".to_string(),
        Char {
            width: 103.0,
            height: 102.0,
            originX: 3.0,
            originY: 179.0,
            advance: 97.0,
            path: "text/42.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "+".to_string(),
        Char {
            width: 136.0,
            height: 136.0,
            originX: -5.0,
            originY: 151.0,
            advance: 146.0,
            path: "text/43.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        ",".to_string(),
        Char {
            width: 38.0,
            height: 73.0,
            originX: -16.0,
            originY: 34.0,
            advance: 69.0,
            path: "text/44.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "-".to_string(),
        Char {
            width: 75.0,
            height: 34.0,
            originX: -4.0,
            originY: 83.0,
            advance: 83.0,
            path: "text/45.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        ".".to_string(),
        Char {
            width: 38.0,
            height: 41.0,
            originX: -16.0,
            originY: 34.0,
            advance: 69.0,
            path: "text/46.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "/".to_string(),
        Char {
            width: 83.0,
            height: 198.0,
            originX: 7.0,
            originY: 188.0,
            advance: 69.0,
            path: "text/47.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        ":".to_string(),
        Char {
            width: 38.0,
            height: 146.0,
            originX: -16.0,
            originY: 139.0,
            advance: 69.0,
            path: "text/58.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        ";".to_string(),
        Char {
            width: 38.0,
            height: 178.0,
            originX: -16.0,
            originY: 139.0,
            advance: 69.0,
            path: "text/59.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "<".to_string(),
        Char {
            width: 136.0,
            height: 141.0,
            originX: -5.0,
            originY: 153.0,
            advance: 146.0,
            path: "text/60.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "=".to_string(),
        Char {
            width: 136.0,
            height: 94.0,
            originX: -5.0,
            originY: 129.0,
            advance: 146.0,
            path: "text/61.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        ">".to_string(),
        Char {
            width: 136.0,
            height: 141.0,
            originX: -5.0,
            originY: 153.0,
            advance: 146.0,
            path: "text/62.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "?".to_string(),
        Char {
            width: 134.0,
            height: 188.0,
            originX: -3.0,
            originY: 181.0,
            advance: 139.0,
            path: "text/63.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "@".to_string(),
        Char {
            width: 227.0,
            height: 230.0,
            originX: -13.0,
            originY: 188.0,
            advance: 254.0,
            path: "text/64.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "A".to_string(),
        Char {
            width: 180.0,
            height: 186.0,
            originX: 6.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/65.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "B".to_string(),
        Char {
            width: 147.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/66.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "C".to_string(),
        Char {
            width: 172.0,
            height: 191.0,
            originX: -6.0,
            originY: 181.0,
            advance: 180.0,
            path: "text/67.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "D".to_string(),
        Char {
            width: 162.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 180.0,
            path: "text/68.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "E".to_string(),
        Char {
            width: 150.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/69.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "F".to_string(),
        Char {
            width: 136.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 152.0,
            path: "text/70.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "G".to_string(),
        Char {
            width: 177.0,
            height: 191.0,
            originX: -6.0,
            originY: 181.0,
            advance: 194.0,
            path: "text/71.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "H".to_string(),
        Char {
            width: 154.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 180.0,
            path: "text/72.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "I".to_string(),
        Char {
            width: 37.0,
            height: 186.0,
            originX: -16.0,
            originY: 179.0,
            advance: 69.0,
            path: "text/73.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "J".to_string(),
        Char {
            width: 117.0,
            height: 188.0,
            originX: 3.0,
            originY: 179.0,
            advance: 125.0,
            path: "text/74.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "K".to_string(),
        Char {
            width: 157.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/75.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "L".to_string(),
        Char {
            width: 124.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 139.0,
            path: "text/76.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "M".to_string(),
        Char {
            width: 181.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 208.0,
            path: "text/77.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "N".to_string(),
        Char {
            width: 154.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 180.0,
            path: "text/78.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "O".to_string(),
        Char {
            width: 185.0,
            height: 191.0,
            originX: -5.0,
            originY: 181.0,
            advance: 194.0,
            path: "text/79.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "P".to_string(),
        Char {
            width: 147.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/80.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "Q".to_string(),
        Char {
            width: 185.0,
            height: 236.0,
            originX: -5.0,
            originY: 181.0,
            advance: 194.0,
            path: "text/81.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "R".to_string(),
        Char {
            width: 162.0,
            height: 186.0,
            originX: -14.0,
            originY: 179.0,
            advance: 180.0,
            path: "text/82.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "S".to_string(),
        Char {
            width: 158.0,
            height: 191.0,
            originX: -4.0,
            originY: 181.0,
            advance: 166.0,
            path: "text/83.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "T".to_string(),
        Char {
            width: 155.0,
            height: 186.0,
            originX: 1.0,
            originY: 179.0,
            advance: 152.0,
            path: "text/84.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "U".to_string(),
        Char {
            width: 156.0,
            height: 188.0,
            originX: -12.0,
            originY: 179.0,
            advance: 180.0,
            path: "text/85.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "V".to_string(),
        Char {
            width: 179.0,
            height: 186.0,
            originX: 6.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/86.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "W".to_string(),
        Char {
            width: 248.0,
            height: 186.0,
            originX: 6.0,
            originY: 179.0,
            advance: 236.0,
            path: "text/87.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "X".to_string(),
        Char {
            width: 170.0,
            height: 186.0,
            originX: 1.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/88.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "Y".to_string(),
        Char {
            width: 170.0,
            height: 186.0,
            originX: 1.0,
            originY: 179.0,
            advance: 166.0,
            path: "text/89.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "Z".to_string(),
        Char {
            width: 151.0,
            height: 186.0,
            originX: -1.0,
            originY: 179.0,
            advance: 152.0,
            path: "text/90.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "[".to_string(),
        Char {
            width: 64.0,
            height: 247.0,
            originX: -11.0,
            originY: 188.0,
            advance: 69.0,
            path: "text/91.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "\\".to_string(),
        Char {
            width: 83.0,
            height: 198.0,
            originX: 7.0,
            originY: 188.0,
            advance: 69.0,
            path: "text/92.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "]".to_string(),
        Char {
            width: 64.0,
            height: 247.0,
            originX: 5.0,
            originY: 188.0,
            advance: 69.0,
            path: "text/93.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "^".to_string(),
        Char {
            width: 129.0,
            height: 104.0,
            originX: 6.0,
            originY: 179.0,
            advance: 117.0,
            path: "text/94.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "_".to_string(),
        Char {
            width: 160.0,
            height: 30.0,
            originX: 11.0,
            originY: -27.0,
            advance: 139.0,
            path: "text/95.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "`".to_string(),
        Char {
            width: 66.0,
            height: 52.0,
            originX: -6.0,
            originY: 191.0,
            advance: 83.0,
            path: "text/96.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "a".to_string(),
        Char {
            width: 143.0,
            height: 151.0,
            originX: -4.0,
            originY: 141.0,
            advance: 139.0,
            path: "text/97.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "b".to_string(),
        Char {
            width: 127.0,
            height: 198.0,
            originX: -9.0,
            originY: 188.0,
            advance: 139.0,
            path: "text/98.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "c".to_string(),
        Char {
            width: 122.0,
            height: 151.0,
            originX: -4.0,
            originY: 141.0,
            advance: 125.0,
            path: "text/99.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "d".to_string(),
        Char {
            width: 126.0,
            height: 198.0,
            originX: -4.0,
            originY: 188.0,
            advance: 139.0,
            path: "text/100.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "e".to_string(),
        Char {
            width: 131.0,
            height: 151.0,
            originX: -4.0,
            originY: 141.0,
            advance: 139.0,
            path: "text/101.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "f".to_string(),
        Char {
            width: 80.0,
            height: 195.0,
            originX: 3.0,
            originY: 188.0,
            advance: 69.0,
            path: "text/102.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "g".to_string(),
        Char {
            width: 126.0,
            height: 200.0,
            originX: -4.0,
            originY: 141.0,
            advance: 139.0,
            path: "text/103.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "h".to_string(),
        Char {
            width: 120.0,
            height: 195.0,
            originX: -10.0,
            originY: 188.0,
            advance: 139.0,
            path: "text/104.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "i".to_string(),
        Char {
            width: 36.0,
            height: 195.0,
            originX: -10.0,
            originY: 188.0,
            advance: 55.0,
            path: "text/105.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "j".to_string(),
        Char {
            width: 59.0,
            height: 247.0,
            originX: 13.0,
            originY: 188.0,
            advance: 55.0,
            path: "text/106.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "k".to_string(),
        Char {
            width: 123.0,
            height: 195.0,
            originX: -10.0,
            originY: 188.0,
            advance: 125.0,
            path: "text/107.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "l".to_string(),
        Char {
            width: 36.0,
            height: 195.0,
            originX: -10.0,
            originY: 188.0,
            advance: 55.0,
            path: "text/108.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "m".to_string(),
        Char {
            width: 189.0,
            height: 148.0,
            originX: -10.0,
            originY: 141.0,
            advance: 208.0,
            path: "text/109.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "n".to_string(),
        Char {
            width: 120.0,
            height: 148.0,
            originX: -10.0,
            originY: 141.0,
            advance: 139.0,
            path: "text/110.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "o".to_string(),
        Char {
            width: 132.0,
            height: 151.0,
            originX: -4.0,
            originY: 141.0,
            advance: 139.0,
            path: "text/111.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "p".to_string(),
        Char {
            width: 127.0,
            height: 200.0,
            originX: -9.0,
            originY: 141.0,
            advance: 139.0,
            path: "text/112.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "q".to_string(),
        Char {
            width: 127.0,
            height: 200.0,
            originX: -4.0,
            originY: 141.0,
            advance: 139.0,
            path: "text/113.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "r".to_string(),
        Char {
            width: 77.0,
            height: 148.0,
            originX: -10.0,
            originY: 141.0,
            advance: 83.0,
            path: "text/114.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "s".to_string(),
        Char {
            width: 123.0,
            height: 151.0,
            originX: 0.0,
            originY: 141.0,
            advance: 125.0,
            path: "text/115.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "t".to_string(),
        Char {
            width: 78.0,
            height: 178.0,
            originX: 3.0,
            originY: 169.0,
            advance: 69.0,
            path: "text/116.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "u".to_string(),
        Char {
            width: 120.0,
            height: 149.0,
            originX: -9.0,
            originY: 139.0,
            advance: 139.0,
            path: "text/117.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "v".to_string(),
        Char {
            width: 138.0,
            height: 146.0,
            originX: 6.0,
            originY: 139.0,
            advance: 125.0,
            path: "text/118.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "w".to_string(),
        Char {
            width: 196.0,
            height: 146.0,
            originX: 7.0,
            originY: 139.0,
            advance: 180.0,
            path: "text/119.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "x".to_string(),
        Char {
            width: 134.0,
            height: 146.0,
            originX: 4.0,
            originY: 139.0,
            advance: 125.0,
            path: "text/120.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "y".to_string(),
        Char {
            width: 138.0,
            height: 198.0,
            originX: 6.0,
            originY: 139.0,
            advance: 125.0,
            path: "text/121.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "z".to_string(),
        Char {
            width: 117.0,
            height: 146.0,
            originX: -3.0,
            originY: 139.0,
            advance: 125.0,
            path: "text/122.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "{".to_string(),
        Char {
            width: 89.0,
            height: 247.0,
            originX: 3.0,
            originY: 188.0,
            advance: 83.0,
            path: "text/123.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "|".to_string(),
        Char {
            width: 35.0,
            height: 248.0,
            originX: -15.0,
            originY: 188.0,
            advance: 65.0,
            path: "text/124.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "}".to_string(),
        Char {
            width: 89.0,
            height: 247.0,
            originX: 3.0,
            originY: 188.0,
            advance: 83.0,
            path: "text/125.png".to_string(),
            layer: 0,
        },
    );
    char_list.insert(
        "~".to_string(),
        Char {
            width: 138.0,
            height: 45.0,
            originX: -4.0,
            originY: 105.0,
            advance: 146.0,
            path: "text/126.png".to_string(),
            layer: 0,
        },
    );

    char_list
}

#[derive(Debug)]
pub struct Font {
    chars: HashMap<String, Char>,
}

impl Font {
    pub fn new() -> Self {
        Font { chars: get_font() }
    }

    pub fn get(&self, c: String) -> &Char {
        &self.chars.get(&c).unwrap()
    }
}
