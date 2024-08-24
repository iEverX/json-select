use wasm_bindgen::prelude::*;

use query::query_str;

#[wasm_bindgen]
pub fn query_wasm(full: &str, select: &str) -> String {
    query_str(full, select).unwrap_or_else(|_| String::from("error"))
}
