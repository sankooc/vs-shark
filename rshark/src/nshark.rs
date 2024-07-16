use crate::common::Context;
use crate::entry::*;
use crate::files::CContext;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WContext {
    ctx: Box<CContext>,
}
#[wasm_bindgen]
impl WContext {
    #[wasm_bindgen(constructor)]
    pub fn new (s: &Uint8Array) -> WContext {
        let mut slice = vec![0; s.length() as usize];
        s.copy_to(&mut slice[..]);
        WContext {
            ctx: Box::new(load_data(slice).unwrap()),
        }
    }

    // pub fn get_file_type(&self) -> String {
        // format!("{:?}",self.ctx.get_file_type().file_type)
    // }
}


#[wasm_bindgen]
pub fn load(s: &Uint8Array) -> WContext {
    WContext::new(s)
}
