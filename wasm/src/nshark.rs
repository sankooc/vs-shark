use core::common::concept::{Criteria, Field};
use std::collections::HashSet;

use core::common::base::Instance;
use core::entry::*;
use std::ops::Deref;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;




#[wasm_bindgen]
pub struct FrameResult {
    pub start: usize,
    pub total: usize,
    items: Vec<FrameInfo>,
}
#[wasm_bindgen]
impl FrameResult {
    #[wasm_bindgen]
    pub fn items(&self) -> Vec<FrameInfo> {
        self.items.clone()
    }
}

impl FrameResult {
    pub fn new( start:usize, total: usize,items: Vec<FrameInfo>) -> Self {
        Self{start, total, items}
    }
}

#[wasm_bindgen]
#[derive(Default,Clone)]
pub struct FrameInfo {
    // frame: &'static Frame,
    pub index: u32,
    pub time: u32,
    source: String,
    dest: String,
    protocol: String,
    pub len: u32,
    pub irtt: u16,
    info: String,
    status: String,
}

#[wasm_bindgen]
impl FrameInfo {
    #[wasm_bindgen(getter)]
    pub fn source(&self) -> String {
        self.source.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn dest(&self) -> String {
        self.dest.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn protocol(&self) -> String {
        self.protocol.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn info(&self) -> String {
        self.info.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn status(&self) -> String {
        self.status.clone()
    }
}



#[wasm_bindgen]
pub struct WContext {
    ctx: Box<Instance>,
}


#[wasm_bindgen]
impl WContext {
    #[wasm_bindgen(constructor)]
    pub fn new(s: &Uint8Array) -> WContext {
        let mut slice = vec![0; s.length() as usize];
        s.copy_to(&mut slice[..]);
        WContext {
            ctx: Box::new(load_data(&slice).unwrap()),
        }
    }
    #[wasm_bindgen]
    pub fn info(&self) -> String {
        self.ctx.pcap_info().to_json()
    }
    
    #[wasm_bindgen]
    pub fn get_frame_count(&self) -> usize {
        self.ctx.get_frames().len()
    }
    
    #[wasm_bindgen]
    pub fn select_frame_items(&mut self, start: usize, size: usize, criteria: String) -> String {
        let cri = Criteria{start, size, criteria};
        if let Ok(str) = self.ctx.get_frames_json(cri) {
            return str;
        }
        return "{}".into()
    }
    #[wasm_bindgen]
    pub fn select_http_count(&self, _: Vec<String>) -> usize {
        let ctx = self.ctx.context();
        let list = ctx.get_http();
        list.len()
    }
    #[wasm_bindgen]
    pub fn select_http_items(&self, _start: usize, _size: usize,_criteria: Vec<String>) -> String {
        let ctx = self.ctx.context();
        ctx.http_list_json()
    }
    pub fn select_http_content(&self, index: usize, ts: u64) -> Uint8Array {
        let ctx = self.ctx.context();
        if let Some(rc) = ctx.http_content(index, ts) {
            let ra: &[u8] =  rc.deref();
            return ra.into();
        }
        Uint8Array::new(&JsValue::undefined())
    }
    #[wasm_bindgen]
    pub fn get_aval_protocals(&self) -> Vec<String> {
        let mut set = HashSet::new();
        for f in self.ctx.get_frames().iter() {
            set.insert(f.get_protocol());
        }
        set.into_iter().collect()
    }
    #[wasm_bindgen]
    pub fn get_fields(&self, index: u32) -> String {
        let binding = self.ctx.get_frames();
        let f = binding.get(index as usize).unwrap();
        if let Ok(str) = f.get_fields_json() {
            return str;
        }
        return "[]".into()
    }
    #[wasm_bindgen]
    pub fn pick_field(&self, index: u32, stack: Vec<u16>) -> super::entity::Field {
        let binding = self.ctx.get_frames();
        let f = binding.get(index as usize).unwrap();
        let list = f.get_fields();

        let mut _list:&[Field] = &list;
        for index in 0..stack.len() {
            let sel = *stack.get(index).unwrap();
            if index >= stack.len() - 1 {
                //
                if let Some(_field) = _list.get(sel as usize) {
                    return super::entity::Field::convert(_field);
                }
                break;
            }
            if let Some(_field) = _list.get(sel as usize) {
                _list = _field.children();
            } else {
                break;
            }
        }
        super::entity::Field::empty()
    }
    #[wasm_bindgen]
    pub fn select_dns_items(&self) -> String {
        if let Ok(str) = self.ctx.context().get_dns_record_json() {
            return str;
        }
        return "[]".into()
    }
    #[wasm_bindgen]
    pub fn select_dns_count(&self) -> usize {
        self.ctx.context().get_dns_count()
    }
    #[wasm_bindgen]
    pub fn select_conversation_count(&self) -> usize{
        let ct = self.ctx.context();
        let cons = ct.conversations();
        cons.len()
    }
    #[wasm_bindgen]
    pub fn select_conversation_items(&self) -> String {
        if let Ok(str) = self.ctx.context().get_conversation_json() {
            return str;
        }
        return "[]".into()
    }
    #[wasm_bindgen]
    pub fn statistic(&self) -> String {
        let ctx = self.ctx.context();
        let stat = ctx.get_statistc();
        stat.to_json()
    }
    #[wasm_bindgen]
    pub fn statistic_frames(&self) -> String {
        match self.ctx.statistic_frames() {
            Ok(_data) => {
                _data.to_json()
            }
            _ => String::from("{\"x\":[], \"y\": [], \"data\": []}"),
        }
    }
    #[wasm_bindgen]
    pub fn select_tls_items(&self) -> String {
        if let Ok(str) = self.ctx.context().get_tls_connection_json() {
            return str;
        }
        return "[]".into()
    }
    #[wasm_bindgen]
    pub fn select_tls_count(&self) -> usize {
        self.ctx.context().tls_connection_infos().len()
    }
}

#[wasm_bindgen]
pub fn load(s: &Uint8Array) -> WContext {
    WContext::new(s)
}
