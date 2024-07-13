use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use crate::entry::*;

struct ImportantExcerpt<'a> {
  pub part:  u8,
  pub content: &'a str
}

#[wasm_bindgen]
pub struct Context {
  
}
#[wasm_bindgen]
pub struct Foo {
  part: Box<ImportantExcerpt<'static>>,
  // im: ImportantExcerpt,
}
#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(_: &str) -> Foo {
        Foo { part: Box::new(ImportantExcerpt{part: 1,content: "ct"}) }
    }

    pub fn get(&self) -> String {
      self.part.content.into()
    }
}

#[wasm_bindgen]
pub fn greet(s: &Uint8Array) -> Foo {
    let str = format!("rust in, {}!", s.length());
    // s.copy_to(dst);
    Foo::new(&str)
    // loadData(8);
}