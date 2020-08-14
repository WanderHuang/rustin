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
mod quick_sort;

/// 算法集合发布
/// 冒泡排序
#[wasm_bindgen]
pub fn bubble(arr: &[i32]) -> Vec<i32> {
    bubble::run(arr)
}

/// 标准库排序
#[wasm_bindgen]
pub fn standard(arr: &[i32]) -> Vec<i32> {
    standard::run(arr)
}

#[wasm_bindgen(js_name = "quickSort")]
pub fn quick_sort(arr: &[i32]) -> Vec<i32> {
    quick_sort::run(&mut arr.to_vec(), 0, arr.len() - 1).to_vec()
}
