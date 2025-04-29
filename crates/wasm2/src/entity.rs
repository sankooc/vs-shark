use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Conf {
    resolve_all: bool,
}
#[wasm_bindgen]
impl Conf {
    #[wasm_bindgen]
    pub fn new(resolve_all: bool) -> Self {
        Self{resolve_all}
    }
    #[wasm_bindgen]
    pub fn resolve_all(&self) -> bool {
        self.resolve_all
    }
}