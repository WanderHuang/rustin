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
pub fn bubble(arr: &[f64]) -> Vec<f64> {
    // crate::utils::set_panic_hook();
    // console_log::init_with_level(Level::Info).expect("error initializing log");

    let mut copy = arr.to_vec();
    // let len = arr.len();

    // for _i in 0..len {
    //     for j in 0..len - 1 {
    //         if copy[j] > copy[j + 1] {
    //             copy.swap(j, j + 1);
    //         }
    //     }
    // }

    copy.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    copy
}

