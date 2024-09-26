use core::specs::http::HTTP;
use std::cell::Ref;

// use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HttpEntity {
    head: String,
    header: Vec<String>,
}
impl HttpEntity {
    pub fn new(http: Ref<HTTP>) -> Self{
      HttpEntity{head: http.head(), header: http.header()}
    }
}
#[wasm_bindgen]
impl HttpEntity {
    #[wasm_bindgen(getter)]
    pub fn head(&self) -> String {
        self.head.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn header(&self) -> Vec<String> {
        self.header.clone()
    }
}
