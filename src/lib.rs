mod utils;

use base64::{engine::general_purpose, Engine as _};
use overlay::handlers::{put_text, BookCoverParams};
use wasm_bindgen::prelude::*;
pub mod overlay;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

#[wasm_bindgen]
pub fn create_cover(base64_img: String, base64_font: String, params: JsValue) -> String {
    utils::set_panic_hook();
    let img_bytes = base64_to_bytes(base64_img);
    let font_bytes = base64_to_bytes(base64_font);
    let book_cover_params: BookCoverParams =
        serde_wasm_bindgen::from_value(params).expect("Failed to parse params");
    let bytes = put_text(img_bytes, font_bytes, book_cover_params).expect("Failed to create cover");
    bytes_to_base64(bytes)
}

// converts base64 string to Vec<u8>
fn base64_to_bytes(base64: String) -> Vec<u8> {
    general_purpose::STANDARD
        .decode(base64)
        .expect("Failed to decode base64")
}

// converts bytes to base64
fn bytes_to_base64(bytes: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(bytes)
}
