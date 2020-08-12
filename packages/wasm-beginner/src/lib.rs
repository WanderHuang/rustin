mod utils;

// use log::Level;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// 引入外部(js)方法
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

/// 发布方法(给js)
#[wasm_bindgen]
pub fn bubble(arr: Box<[JsValue]>) -> Box<[JsValue]> {
    // crate::utils::set_panic_hook();
    // console_log::init_with_level(Level::Info).expect("error initializing log");

    let mut copy = arr.clone();
    let len = arr.len();

    for _i in 0..len {
        for j in 0..len - 1 {
            let a = copy[j].as_f64();
            let b = copy[j + 1].as_f64();
            if a > b {
                exchange(&mut copy, j, j + 1);
            }
        }
    }

    arr
}

fn exchange(arr: &mut Box<[JsValue]>, i: usize, j: usize) {
    let temp = arr[i].clone();
    arr[i] = arr[j].clone();
    arr[j] = temp;
}
