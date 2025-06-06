use js_sys::Uint8Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct Conf {
    resolve_all: bool,
    batch_size: usize,
}
#[wasm_bindgen]
impl Conf {
    #[wasm_bindgen]
    pub fn new(resolve_all: bool, batch_size: usize) -> Self {
        Self { resolve_all, batch_size }
    }
    #[wasm_bindgen]
    pub fn resolve_all(&self) -> bool {
        self.resolve_all
    }
    #[wasm_bindgen]
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[wasm_bindgen]
impl Range {
    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }
    #[wasm_bindgen]
    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

#[wasm_bindgen]
pub struct FrameResult{
    list: String,
    extra: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl FrameResult {
    pub fn new(list: String, extra: Option<Vec<u8>>) -> Self {
        Self { list, extra }
    }

    pub fn empty() -> Self {
        Self { list: "{}".into(), extra: None }
    }
    #[wasm_bindgen]
    pub fn list(&self) -> String {
        self.list.clone()
    }
    #[wasm_bindgen]
    pub fn extra(&self) -> Uint8Array {
        if let Some(v) = &self.extra {
            Uint8Array::from(v.as_slice())
        } else {
            Uint8Array::from(JsValue::null())
        }
    }
}


#[wasm_bindgen]
pub struct FrameRange{
    pub frame: Range,
    pub data: Range,
}

#[wasm_bindgen]
impl FrameRange {
    pub fn new() -> Self{
        Self{frame: Range::empty(), data: Range::empty()}
    }
    #[wasm_bindgen]
    pub fn compact(&self) -> bool {
        self.frame.start == self.data.start && self.frame.end == self.data.end
    }
}



impl From<std::ops::Range<usize>> for Range {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self{start: value.start, end: value.end}
    }
}