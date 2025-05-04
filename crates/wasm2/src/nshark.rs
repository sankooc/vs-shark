use js_sys::Uint8Array;
use pcap::common::{concept::Criteria, Instance};
use wasm_bindgen::prelude::*;

use crate::entity::Conf;

#[wasm_bindgen]
pub struct WContext {
    ctx: Box<Instance>,
}


#[wasm_bindgen]
impl WContext {
    #[wasm_bindgen(constructor)]
    pub fn new(_: Conf) -> WContext {
        let ins = Instance::new();
        WContext {
            ctx: Box::new(ins),
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, s: &Uint8Array) -> String {
        let slice = s.to_vec();
        self.ctx.update(slice).unwrap().to_json()
    }

    
    #[wasm_bindgen]
    pub fn count(&self, catelog: String) -> usize {
        self.ctx.get_count(&catelog)
    }
    
    #[wasm_bindgen]
    pub fn list(&self, catelog: String, start: usize, size: usize) -> String {
        let cri = Criteria{start, size};
        match catelog.as_str() {
            "frame" => self.ctx.frames_list_json(cri).unwrap(),
            _ => "{}".into()
        }
    }

    #[wasm_bindgen]
    pub fn select(&self, catelog: String, index: usize) -> String {
        match catelog.as_str() {
            "frame" => self.ctx.select_frame(index).unwrap(),
            _ => "{}".into()
        }
    }
}

#[wasm_bindgen]
pub fn load(conf: Conf) -> WContext {
    WContext::new(conf)
}
