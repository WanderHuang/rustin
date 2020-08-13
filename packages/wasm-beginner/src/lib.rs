mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 引入外部(js)方法
// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

mod bubble;
mod standard;

/// 算法集合发布
/// 冒泡排序
#[wasm_bindgen]
pub fn bubble(arr: &[f64]) -> Vec<f64> {
    bubble::run(arr)
}

/// 标准库排序
#[wasm_bindgen]
pub fn standard(arr: &[f64]) -> Vec<f64> {
    standard::run(arr)
}
