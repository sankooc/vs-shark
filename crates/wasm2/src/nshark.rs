use js_sys::Uint8Array;
use pcap::common::Instance;
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
        // let mut slice = vec![0; s.length() as usize];
        // s.copy_to(&mut slice[..]);
        // let slice = s.to_vec();
        // let _start = instant::Instant::now();

        let ins = Instance::new();
        
        // // let mut ins = load_data(slice, conf.into()).unwrap();
        
        // ins.update(&slice).unwrap();
        // ins.ctx.cost = start.elapsed().as_millis() as usize;
        WContext {
            ctx: Box::new(ins),
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, s: &Uint8Array) -> String {
        let slice = s.to_vec();
        // let _start = instant::Instant::now();
        // let mut ins = Instance::new();
        // ins.update(&slice).unwrap();
        self.ctx.update(slice).unwrap()
        
    }
}

#[wasm_bindgen]
pub fn load(conf: Conf) -> WContext {
    WContext::new(conf)
}
