use js_sys::Uint8Array;
use pcap::common::{
    concept::{ConversationCriteria, Criteria, HttpCriteria},
    Instance, ResourceLoader,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::entity::{Conf, FrameResult, HttpDetail, Range};

pub struct WASMLoader {
    id: String,
}
impl WASMLoader {
    pub fn new(id: String) -> Self {
        Self { id }
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

impl ResourceLoader for WASMLoader {
    fn load(&self, r: &std::ops::Range<usize>) -> anyhow::Result<Vec<u8>> {
        let data = load_data(&self.id, r.into());
        Ok(data.to_vec())
    }
    fn loads(&self, ranges: &[std::ops::Range<usize>]) -> anyhow::Result<Vec<u8>> {
        let rs = ranges.iter().map(|f| f.into()).collect();
        Ok(loads_data(&self.id, rs).to_vec())
    }
}
#[wasm_bindgen]
pub struct WContext {
    ctx: Instance<WASMLoader>,
}

#[wasm_bindgen]
impl WContext {
    #[wasm_bindgen(constructor)]
    pub fn new(conf: Conf) -> WContext {
        let loader = WASMLoader::new(conf.id());
        let ins = Instance::new(conf.batch_size(), loader);
        WContext { ctx: ins }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, s: &Uint8Array) -> Option<String> {
        let slice = s.to_vec();
        self.ctx.update(slice).ok().as_ref().and_then(jsonlize)
    }
    #[wasm_bindgen]
    pub fn update_slice(&mut self, s: &[u8]) -> Option<String> {
        self.ctx.update_slice(s).ok().as_ref().and_then(jsonlize)
        // self.ctx.update_slice(s).unwrap().to_json()
    }

    #[wasm_bindgen]
    pub fn select(&self, catelog: String, index: usize) -> Option<String> {
        match catelog.as_str() {
            "frame" => {
                // self.ctx.select_frame(index).map(|(items, _)| serde_json::to_string(&items).ok()).flatten()
                self.ctx.select_frame(index).and_then(|(items, _)| serde_json::to_string(&items).ok())
            }
            _ => None,
        }
    }

    #[wasm_bindgen]
    pub fn select_frame(&self, index: usize) -> FrameResult {
        if let Some((list, datsources)) = self.ctx.select_frame(index) {
            let data = serde_json::to_string(&list).unwrap();
            let rs = FrameResult::new(data, datsources);
            return rs;
        }
        FrameResult::empty()
    }
    
    #[wasm_bindgen]
    pub fn list_frames(&mut self, start: usize, size: usize) -> Option<String> {
        let cri = Criteria { start, size };
        let rs = self.ctx.frames_by(cri);
        jsonlize(&rs)
    }
    #[wasm_bindgen]
    pub fn list_conversations(&self, start: usize, size: usize, ip: String) -> Option<String> {
        let filter = if ip.is_empty() { ConversationCriteria::default() } else { ConversationCriteria::ip(ip) };
        let rs = self.ctx.conversations(Criteria { start, size }, filter);
        jsonlize(&rs)
    }
    #[wasm_bindgen]
    pub fn list_connections(&self, index: usize, start: usize, size: usize) -> Option<String> {
        let rs = self.ctx.connections(index, Criteria { start, size });
        jsonlize(&rs)
    }
    #[wasm_bindgen]
    pub fn list_http(&self, start: usize, size: usize, hostname: String, _method: String) -> Option<String> {
        let filter = if hostname.is_empty() { None } else { Some(HttpCriteria::hostname(hostname)) };
        let rs = self.ctx.http_connections(Criteria { start, size }, filter);
        jsonlize(&rs)
    }
    #[wasm_bindgen]
    pub fn list_udp(&self, start: usize, size: usize, ip: String) -> Option<String> {
        let filter = if ip.is_empty() { None } else { Some(ip) };
        let rs = self.ctx.udp_conversations(Criteria { start, size }, filter);
        jsonlize(&rs)
    }
    #[wasm_bindgen]
    pub fn list_tls(&self, start: usize, size: usize) -> Option<String> {
        let list = self.ctx.tls_connections(Criteria { start, size });
        jsonlize(&list)
    }
    #[wasm_bindgen]
    pub fn list_tls_conv(&self, index: usize, start: usize, size: usize) -> Option<String> {
        let list = self.ctx.tls_conv_list(index, Criteria { start, size });
        jsonlize(&list)
    }
    #[wasm_bindgen]
    pub fn list_dns(&self, start: usize, size: usize) -> Option<String> {
        let list = self.ctx.dns_records(Criteria { start, size });
        jsonlize(&list)
    }
    #[wasm_bindgen]
    pub fn dns_records(&self, index: usize, start: usize, size: usize) -> Option<String> {
        let list = self.ctx.dns_record(index, Criteria { start, size });
        jsonlize(&list)
    }
    #[wasm_bindgen]
    pub fn http_detail(&self, index: usize) -> Option<Vec<HttpDetail>> {
        self.ctx.http_detail(index).map(|data| data.into_iter().map(HttpDetail::from).collect())
    }

    #[wasm_bindgen]
    pub fn stat(&self, field: String) -> Option<String> {
        // let ctx = self.ctx.context();
        let items = match field.as_str() {
            "http_host" => self.ctx.stat_http_host(),
            "ip4" => self.ctx.stat_ip4(),
            "ip6" => self.ctx.stat_ip6(),
            "http_data" => {
                let rs = self.ctx.stat_http();
                return jsonlize(&rs);
            }
            "frame" => {
                let rs = self.ctx.stat_frame();
                return jsonlize(&rs);
            }
            "ip_address" => self.ctx.stat_ipaddress_distribute(),
            _ => {
                return None;
            }
        };
        jsonlize(&items)
    }
}

fn jsonlize<T>(data: &T) -> Option<String>
where
    T: Serialize,
{
    serde_json::to_string(&data).ok()
}

#[wasm_bindgen]
pub fn load(conf: Conf) -> WContext {
    wasm_log("start load wasm");
    WContext::new(conf)
}

#[wasm_bindgen]
extern "C" {
    fn load_data(id: &str, r: Range) -> Uint8Array;
    fn loads_data(id: &str, r: Vec<Range>) -> Uint8Array;
    fn wasm_log(str: &str);
}
