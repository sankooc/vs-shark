use std::borrow::Borrow;
use std::cmp;
use std::collections::HashSet;

use core::common::FIELDSTATUS;
use core::common::base::{ Element, Frame, Instance};
use core::entry::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::entity::{DNSRecord, Field, HttpConversation, TCPConversation, WTLSHS};



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


fn _convert(f_status: FIELDSTATUS) -> &'static str {
    match f_status {
        FIELDSTATUS::WARN => "deactive",
        FIELDSTATUS::ERROR => "errordata",
        _ => "info"
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
    pub fn info(&self) -> String {
        self.ctx.pcap_info().to_json()
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
        match frame.eles.last() {
            Some(ele) => {
                item.status = _convert(ele.status()).into();
            },
            _ => {}
        }
        item.irtt = 1;
        item
    }

    
    #[wasm_bindgen]
    pub fn select_frame_items(&mut self, start: usize, size: usize, criteria: Vec<String>) -> FrameResult {
        let info = self.ctx.context().get_info();
        let start_ts = info.start_time;
        let _fs = self.ctx.get_frames();
        let mut total = 0;
        let mut items = Vec::new();
        if criteria.len() > 0 {
            let mut left = size;
            let _filters = HashSet::from_iter(criteria.iter().cloned());
            for frame in _fs.iter() {
                if frame.do_match(&_filters) {
                    total += 1;
                    if total > start && left > 0 {
                        left -= 1;
                        let item = WContext::_frame(frame, start_ts);
                        items.push(item);
                    }
                }
            }
            return FrameResult::new(start, total, items);
        }
        total = _fs.len();
        if total <= start {
            return FrameResult::new(start, 0, Vec::new());
        }
        let end = cmp::min(start + size, total);
        let _data: &[Frame] = &_fs[start..end];
        for frame in _data.iter() {
            let item = WContext::_frame(frame, start_ts);
            items.push(item);
        }
        FrameResult::new(start, total, items)
    }

    #[wasm_bindgen]
    pub fn select_http_count(&self, _: Vec<String>) -> usize {
        let ctx = self.ctx.context();
        let list = ctx.get_http();
        list.len()
    }
    #[wasm_bindgen]
    pub fn select_http_items(&self, start: usize, size: usize,_criteria: Vec<String>) -> Vec<HttpConversation>{
        let ctx = self.ctx.context();
        let len =  ctx.get_http().len();
        if start >= len {

        }
        let f = cmp::min(len, start + size);
        let mut list = Vec::new();
        let mut index: usize = start;
        let _list = ctx.get_http();
        loop {
            if index >= f {
                break;
            }
            let _http = _list.get(index).unwrap();
            list.push(HttpConversation::new(_http));
            index += 1;
        }
        list
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
    pub fn get_frames(&mut self) -> Vec<FrameInfo> {
        let info = self.ctx.context().get_info();
        let start_ts = info.start_time;
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
    pub fn select_dns_items(&self) -> Vec<DNSRecord> {
        let mut rs = Vec::new();
        for d in self.ctx.context().dns.iter() {
            let aa = d.as_ref().borrow();
            rs.push(DNSRecord::create(aa));
        }
        rs
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
    pub fn select_conversation_items(&self) -> Vec<TCPConversation>{
        let ct = self.ctx.context();
        let cons = ct.conversations();
        let mut rs = Vec::new();
        for con in cons.values().into_iter() {
            let reff = con.borrow();
            let (source, target) = reff.sort(ct.statistic.ip.get_map());
            let tcp = TCPConversation::new(source, target, ct);
            rs.push(tcp);
            drop(reff);
        }
        rs
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
    pub fn select_tls_items(&self) -> Vec<WTLSHS> {
        self.ctx.context().tls_connection_info().iter().map(|f| WTLSHS::new(f.to_owned())).collect::<_>()
    }
    #[wasm_bindgen]
    pub fn select_tls_count(&self) -> usize {
        self.ctx.context().tls_connection_info().len()
    }
}

#[wasm_bindgen]
pub fn load(s: &Uint8Array) -> WContext {
    WContext::new(s)
}
