use once_cell::sync::Lazy;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add2(x: i32, y: i32) -> i32 {
    x + y
}

static BINARY: Lazy<Vec<u8>> = Lazy::new(|| vec![0, 1, 2, 3, 4]);

#[wasm_bindgen]
pub fn array() -> *const u8 {
    let mut str = String::new();
    for x in BINARY.iter() {
        str.push(to_hex(*x));
        str.push(to_hex((*x) >> 4));
    }
    log(&str);
    return BINARY.as_ptr();

    fn to_hex(num: u8) -> char {
        let b = (num & 0xF) as u32;
        unsafe {
            if b < 10 {
                char::from_u32_unchecked(('0' as u32) + b)
            } else {
                char::from_u32_unchecked(('A' as u32) + (b - 10))
            }
        }
    }
}
