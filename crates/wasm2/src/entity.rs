use wasm_bindgen::prelude::wasm_bindgen;

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


impl From<std::ops::Range<usize>> for Range {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self{start: value.start, end: value.end}
    }
}