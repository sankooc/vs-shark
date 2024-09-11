use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::cmp;
use std::ops::Deref;

use core::common::FileInfo;
use core::files::{DomainService, Element, Frame, Instance};
use core::{entry::*, files};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WContext {
    ctx: Box<Instance>,
}

#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct Field {
    pub start: usize,
    pub size: usize,
    summary: String,
    children: RefCell<Vec<files::Field>>,
    data: Uint8Array,
    // children: Vec<Field>,
}
impl Field {
    pub fn convert(embed: &files::Field) -> Self {
        let (start, size);
        files::Field { start, size, .. } = *embed;
        let summary = embed.summary.clone();
        let a: &[u8] = embed.borrow().data.as_ref();
        let data: Uint8Array = a.into();
        let children = embed.children.clone();
        Field {
            start,
            size,
            summary,
            data,
            children,
        }
    }
}
#[wasm_bindgen]
impl Field {
    #[wasm_bindgen(getter)]
    pub fn summary(&self) -> String {
        self.summary.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn children(&self) -> Vec<Field> {
        let mut children = Vec::new();
        for c in self.children.borrow().iter() {
            children.push(Field::convert(c));
        }
        children
    }
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Uint8Array {
        self.data.clone()
    }
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
        DNSRecord {
            name: data.name(),
            _type: data._type(),
            proto: data.proto(),
            class: data.class(),
            content: data.content(),
            ttl: data.ttl(),
        }
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
pub struct TCPConversation{
    source: String,
    dest: String,
    count: u16,
    throughput: u32,
}
#[wasm_bindgen]
impl TCPConversation {
    #[wasm_bindgen(getter)]
    pub fn source(&self) -> String {
        self.source.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn dest(&self) -> String {
        self.dest.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> u16 {
        self.count
    }
    #[wasm_bindgen(getter)]
    pub fn throughput(&self) -> u32 {
        self.throughput
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
    pub fn get_frame_count(&self) -> usize {
        self.ctx.get_frames().len()
    }
    
    fn _frame(frame: &Frame, start_ts: u64) -> FrameInfo {
        let mut item = FrameInfo {
            ..Default::default()
        };
        let sum = frame.summary.borrow();
        item.index = sum.index;
        item.time = (frame.ts - start_ts) as u32;
        item.len = frame.capture_size;
        match &sum.ip {
            Some(ip) => {
                let _ip = ip.as_ref().borrow();
                item.source = _ip.source_ip_address();
                item.dest = _ip.target_ip_address();
            }
            None => {}
        }
        item.protocol = sum.protocol.clone();
        item.info = frame.info();
        item.status = "info".into();
        match frame.eles.borrow().last() {
            Some(ele) => {
                item.status = ele.status();
            },
            _ => {}
        }
        item.irtt = 1;
        item
    }

    
    #[wasm_bindgen]
    pub fn select_frames(&mut self, start: usize, size: usize) -> Vec<FrameInfo> {
        let start_ts = self.get_info().start_time;
        let _fs = self.ctx.get_frames();
        let total = _fs.len();
        if total <= start {
            return Vec::new()
        }
        let end = cmp::min(start + size, total);
        let _data: &[Frame] = &_fs.deref()[start..end];
        let mut rs = Vec::new();
        for frame in _data.iter() {
            let item = WContext::_frame(frame, start_ts);
            rs.push(item);
        }
        rs
    }

    #[wasm_bindgen]
    pub fn get_aval_protocals(&self) -> Vec<String>{
        Vec::new()
    }

    #[wasm_bindgen]
    pub fn get_frames(&mut self) -> Vec<FrameInfo> {
        let start_ts = self.get_info().start_time;
        let mut rs = Vec::new();
        for frame in self.ctx.get_frames().iter() {
            let item = WContext::_frame(frame, start_ts);
            rs.push(item);
        }
        rs
    }
    #[wasm_bindgen]
    pub fn get_fields(&self, index: u32) -> Vec<Field> {
        let binding = self.ctx.get_frames();
        let f = binding.get(index as usize).unwrap();
        f.get_fields().iter().map(|f| Field::convert(&f)).collect()
    }

    
    #[wasm_bindgen]
    pub fn get_dns_record(&self) -> Vec<DNSRecord> {
        let mut rs = Vec::new();
        for d in self.ctx.context().dns.borrow().iter() {
            let aa = d.as_ref().borrow();
            rs.push(DNSRecord::create(aa));
        }
        rs
    }
    #[wasm_bindgen]
    pub fn get_dns_count(&self) -> usize {
        self.ctx.context().get_dns_count()
    }
    #[wasm_bindgen]
    pub fn get_conversations_count(&self) -> usize{
        let ct = self.ctx.context();
        let cons = ct.conversations();
        cons.len()
    }
    #[wasm_bindgen]
    pub fn get_conversations(&self) -> Vec<TCPConversation>{
        let ct = self.ctx.context();
        let cons = ct.conversations();
        let mut rs = Vec::new();
        for con in cons.values().into_iter() {
            let source = con.ep1.as_ref().borrow().stringfy();
            let dest = con.ep2.as_ref().borrow().stringfy();
            let count:u16 = con.count.get();
            let throughput = con.throughput.get();
            rs.push(TCPConversation{source, dest, count, throughput})
        }
        rs
    }
}

#[wasm_bindgen]
pub fn load(s: &Uint8Array) -> WContext {
    WContext::new(s)
}
