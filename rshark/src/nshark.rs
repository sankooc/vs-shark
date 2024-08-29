use std::cell::Ref;

use crate::common::FileInfo;
use crate::entry::*;
use crate::files::{DomainService, Field, Instance};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WContext {
    ctx: Box<Instance>,
}


#[wasm_bindgen]
pub struct DNSRecord {
    name: String,
    _type: String,
    proto: String,
    class: String,
    content: String,
    pub ttl: u32,
}

impl DNSRecord {
    pub fn create(data: Ref<impl DomainService>) -> DNSRecord {
        DNSRecord{ name: data.name(), _type:data._type(), proto: data.proto(), class: data.class(), content: data.content(), ttl: data.ttl()}
    }
}

#[wasm_bindgen]
impl DNSRecord {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn _type(&self) -> String {
        self._type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn proto(&self) -> String {
        self.proto.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> String {
        self.content.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn class(&self) -> String {
        self.class.clone()
    }
}

#[wasm_bindgen]
pub struct WFileInfo {
    pub link_type: u16,
    file_type: String,
    pub start_time: u64,
    version: String,
}
#[wasm_bindgen]
impl WFileInfo {
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        self.version.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn file_type(&self) -> String {
        self.file_type.clone()
    }
}
impl WFileInfo {
    fn new(info: FileInfo) -> WFileInfo {
        WFileInfo {
            link_type: info.link_type,
            file_type: format!("{:?}", info.file_type),
            start_time: info.start_time,
            version: info.version.clone(),
        }
    }
}
#[wasm_bindgen]
#[derive(Default)]
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
    pub fn get_info(&mut self) -> WFileInfo {
        WFileInfo::new(self.ctx.get_info())
    }
    #[wasm_bindgen]
    pub fn get_frames(&mut self) -> Vec<FrameInfo> {
        let start_ts = self.get_info().start_time;
        let mut rs = Vec::new();
        for frame in self.ctx.get_frames().iter() {
            let mut item = FrameInfo {
                ..Default::default()
            };
            let sum = frame.summary.borrow();
            item.index = sum.index;
            item.time = (frame.ts - start_ts) as u32;
            item.len = frame.capture_size;
            item.source = sum.source.clone();
            item.dest = sum.target.clone();
            item.protocol = sum.protocol.clone();
            item.info = frame.info();
            item.irtt = 1;
            rs.push(item);
        }
        rs
    }
    #[wasm_bindgen]
    pub fn get_fields(&self, index: u32) -> Vec<Field> {
        let binding = self.ctx.get_frames();
        let f = binding.get(index as usize).unwrap();
        f.get_fields()
    }
    
    #[wasm_bindgen]
    pub fn get_dns_record(&self)-> Vec<DNSRecord>{
        self.ctx.context().get_dns()
    }
    #[wasm_bindgen]
    pub fn get_dns_count(&self) -> usize {
        self.ctx.context().get_dns_count()
    }
}

#[wasm_bindgen]
pub fn load(s: &Uint8Array) -> WContext {
    WContext::new(s)
}
