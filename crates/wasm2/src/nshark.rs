use js_sys::Uint8Array;
use pcap::common::{concept::Criteria, Instance};
use wasm_bindgen::prelude::*;

use crate::entity::{Conf, FrameRange, FrameResult};

#[wasm_bindgen]
pub struct WContext {
    ctx: Box<Instance>,
}


#[wasm_bindgen]
impl WContext {
    #[wasm_bindgen(constructor)]
    pub fn new(conf: Conf) -> WContext {
        let ins = Instance::new(conf.batch_size());
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
    pub fn update_slice(&mut self, s: &[u8]) -> String {
        self.ctx.update_slice(s).unwrap().to_json()
    }

    
    #[wasm_bindgen]
    pub fn count(&self, catelog: String) -> usize {
        self.ctx.get_count(&catelog)
    }
    
    #[wasm_bindgen]
    pub fn list(&mut self, catelog: String, start: usize, size: usize) -> String {
        let cri = Criteria{start, size};
        match catelog.as_str() {
            "frame" => self.ctx.frames_list_json(cri).unwrap(),
            _ => "{}".into()
        }
    }

    #[wasm_bindgen]
    pub fn frame_range(&self, index: usize) -> FrameRange {
        let mut rs = FrameRange::new();
        if let Some(f) = self.ctx.frame(index) {
            if let Some(range) = f.frame_range() {
                rs.frame = range.into();
            }
            if let Some(range) = f.range() {
                rs.data = range.into();
            }
        }
        rs
    }

    #[wasm_bindgen]
    pub fn select(&self, catelog: String, index: usize, s: &Uint8Array) -> String {
        match catelog.as_str() {
            "frame" => {
                let slice = s.to_vec();
                self.ctx.select_frame_json(index, slice).unwrap()
            },
            _ => "{}".into()
        }
    }
    
    #[wasm_bindgen]
    pub fn select_frame(&self, index: usize, s: &Uint8Array) -> FrameResult {
        let slice = s.to_vec();
        if let Some((list, _, extra)) = self.ctx.select_frame(index, slice) {
            let data = serde_json::to_string(&list).unwrap();
            let rs = FrameResult::new(data, extra);
            return rs;
        }
        FrameResult::empty()
    }
}

#[wasm_bindgen]
pub fn load(conf: Conf) -> WContext {
    WContext::new(conf)
}
