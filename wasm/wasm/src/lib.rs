use wasm_bindgen::prelude::*;

use query::query_str;

#[wasm_bindgen]
pub fn query_wasm(full: &str, select: &str) -> String {
    match query_str(full, select) {
        Ok(res) => res,
        Err(_) => String::from("error"),
    }
}
