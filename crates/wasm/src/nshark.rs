use js_sys::Uint8Array;
use pcap::common::{
    concept::{ConversationCriteria, Criteria, HttpCriteria}, Instance, ResourceLoader
};
use wasm_bindgen::prelude::*;

use crate::entity::{Conf, FrameResult, HttpDetail, Range};


pub struct WASMLoader {
    id: String
}
impl WASMLoader {
    pub fn new(id: String) -> Self {
        Self{id}
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
        let loader= WASMLoader::new(conf.id());
        let ins = Instance::new(conf.batch_size(), loader);
        WContext { ctx: ins }
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
        let cri = Criteria { start, size };
        match catelog.as_str() {
            "frame" => self.ctx.frames_list_json(cri).unwrap(),
            _ => "{}".into(),
        }
    }

    // #[wasm_bindgen]
    // pub fn frame_range(&self, index: usize) -> FrameRange {
    //     let mut rs = FrameRange::new();
    //     if let Some(f) = self.ctx.frame(index) {
    //         if let Some(range) = f.frame_range() {
    //             rs.frame = range.into();
    //         }
    //         if let Some(range) = f.range() {
    //             rs.data = range.into();
    //         }
    //     }
    //     rs
    // }

    #[wasm_bindgen]
    pub fn select(&self, catelog: String, index: usize) -> String {
        match catelog.as_str() {
            "frame" => {
                self.ctx.select_frame_json(index).unwrap()
            }
            _ => "{}".into(),
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
    pub fn list_conversations(&self, start: usize, size: usize, ip: String) -> String {
        let filter = if ip.is_empty() { ConversationCriteria::default() } else { ConversationCriteria::ip(ip) };
        let rs = self.ctx.conversations(Criteria { start, size }, filter);
        serde_json::to_string(&rs).unwrap_or("{}".into())
    }
    #[wasm_bindgen]
    pub fn list_connections(&self, index: usize, start: usize, size: usize) -> String {
        let rs = self.ctx.connections(index, Criteria { start, size });
        serde_json::to_string(&rs).unwrap_or("{}".into())
    }
    #[wasm_bindgen]
    pub fn list_http(&self, start: usize, size: usize, hostname: String, _method: String) -> String {
        let filter = if hostname.is_empty() { None } else { Some(HttpCriteria::hostname(hostname)) };
        let rs = self.ctx.http_connections(Criteria { start, size }, filter);
        serde_json::to_string(&rs).unwrap_or("{}".into())
    }
    #[wasm_bindgen]
    pub fn list_udp(&self, start: usize, size: usize, ip: String) -> String {
        let filter = if ip.is_empty() { None } else { Some(ip) };
        let rs = self.ctx.udp_conversations(Criteria { start, size }, filter);
        serde_json::to_string(&rs).unwrap_or("{}".into())
    }
    #[wasm_bindgen]
    pub fn list_tls(&self) -> Option<String> {
        let list =self.ctx.tls_infos();
        serde_json::to_string(&list).ok()
    }
    #[wasm_bindgen]
    pub fn http_detail(&self, index: usize) -> Option<Vec<HttpDetail>>{
        if let Some(data) = self.ctx.http_detail(index) {
            Some(data.into_iter().map(HttpDetail::from).collect())
        } else {
            None
        }
    }
    #[wasm_bindgen]
    pub fn stat(&self, field: String) -> String {
        let ctx = self.ctx.context();
        match field.as_str() {
            "http_host" => ctx.stat_http_host(),
            // "tls_sni" => ctx.stat_tls_sni(),
            "ip4" => ctx.stat_ip4(),
            "ip6" => ctx.stat_ip6(),
            "http_data" => ctx.stat_http_data(),
            "frame" => ctx.stat_frame(),
            _ => "[]".to_string(),
        }
    }
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