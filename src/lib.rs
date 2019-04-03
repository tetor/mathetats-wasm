mod utils;

use std::f64;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn min(x: f64, y: f64) -> f64 {
    x.min(y)
}

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, mathetats-wasm!");
// }
