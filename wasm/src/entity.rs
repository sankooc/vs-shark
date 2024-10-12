use core::{
    common::{
        base::{Context, DomainService, Endpoint}, concept::{Case, HttpRequestBuilder, Statistic, TLSHS}, Ref2
    },
    specs::http::HTTP,
};
use std::cell::{Ref, RefCell};

use js_sys::Uint8Array;
// use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;



#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct Field {
    pub start: usize,
    pub size: usize,
    summary: String,
    children: RefCell<Vec<core::common::base::Field>>,
    data: Uint8Array,
}
impl Field {
    pub fn convert(embed: &core::common::base::Field) -> Self {
        let (start, size);
        core::common::base::Field { start, size, .. } = *embed;
        let summary = embed.summary.clone();
        let a: &[u8] = embed.data.as_ref();
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
pub struct WFileInfo {
    pub link_type: u32,
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
    // fn new(info: FileInfo) -> WFileInfo {
    //     WFileInfo {
    //         link_type: info.link_type,
    //         file_type: format!("{:?}", info.file_type),
    //         start_time: info.start_time,
    //         version: info.version.clone(),
    //     }
    // }
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

#[derive(Clone)]
#[wasm_bindgen]
pub struct WEndpoint {
    ip: String,
    pub port: u16,
    host: String,
    pub count: u16,
    pub throughput: u32,
    pub retransmission: u16,
    pub invalid: u16,
}

impl WEndpoint {
    fn new(ep: &Endpoint, ctx: &Context) -> Self {
        let (ip, port, host) = ctx._to_hostnames(ep);
        let info = &ep.info;
        Self{ ip, port, host, count: info.count, throughput: info.throughput, retransmission: info.retransmission, invalid: info.invalid }
    }
}

#[wasm_bindgen]
impl WEndpoint {
    #[wasm_bindgen(getter)]
    pub fn ip(&self) -> String {
        self.ip.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn host(&self) -> String {
        self.host.clone()
    }
}

#[wasm_bindgen]
pub struct TCPConversation{
    source: WEndpoint,
    target: WEndpoint,
}
impl TCPConversation {
    pub fn new(s: &Endpoint, t: &Endpoint, ctx: &Context) -> Self {
        let source = WEndpoint::new(s, ctx);
        let target = WEndpoint::new(t, ctx);
        Self{source, target}
    }
}
#[wasm_bindgen]
impl TCPConversation {
    #[wasm_bindgen(getter)]
    pub fn source(&self) -> WEndpoint {
        self.source.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn target(&self) -> WEndpoint {
        self.target.clone()
    }
}


#[wasm_bindgen]
pub struct HttpConversation {
    method: Option<String>,
    status: Option<String>,
    pub ttr: u64,
    req: HttpEntity,
    res: HttpEntity,
}
#[wasm_bindgen]
impl HttpConversation {
    #[wasm_bindgen(getter)]
    pub fn req(&self) -> HttpEntity {
        self.req.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn res(&self) -> HttpEntity {
        self.res.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.clone().unwrap_or(String::from(""))
    }
    #[wasm_bindgen(getter)]
    pub fn status(&self) -> String {
        self.status.clone().unwrap_or(String::from(""))
    }
}
#[derive(Clone)]
#[wasm_bindgen]
pub struct HttpEntity {
    host: String,
    pub port: u16,
    http: Ref2<HTTP>,
}
impl HttpEntity {
    pub fn new(host: String, port: u16, http: Ref2<HTTP>) -> Self {
        Self { host, port, http }
    }
}
impl HttpConversation {
    pub fn new(http: &HttpRequestBuilder) -> Self {
        let req = HttpEntity::new(http.source.clone(), http.srp, http.request.clone().unwrap());
        let res = HttpEntity::new(http.dest.clone(), http.dsp, http.response.clone().unwrap());
        let mut ttr = 0;
        if http.end > http.start {
            ttr = http.end - http.start; 
        }
        Self {
            req,
            res,
            ttr,
            method: http.method.clone(),
            status: http.status.clone(),
        }
    }
}
#[wasm_bindgen]
impl HttpEntity {
    #[wasm_bindgen(getter)]
    pub fn host(&self) -> String {
        self.host.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn head(&self) -> String {
        self.http.as_ref().borrow().head()
    }
    #[wasm_bindgen(getter)]
    pub fn header(&self) -> Vec<String> {
        self.http.as_ref().borrow().header()
    }
    // #[wasm_bindgen(getter)]
    // pub fn content_len(&self) -> usize {
    //     let _http = self.http.as_ref().borrow();
    //     _http.len
    // }
    // #[wasm_bindgen(getter)]
    // pub fn content(&self) -> Uint8Array {
    //     let _http = self.http.as_ref().borrow();
    //     // if _http.len > 0 {
            
    //     // }
    //     let data:&[u8] = &_http.content;
    //     return data.into()
    // }
}
#[wasm_bindgen]
#[derive(Clone)]
pub struct WCase {
    name: String,
    value: usize,
}

#[wasm_bindgen]
impl WCase {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> usize {
        self.value
    }
}
impl WCase {
    pub fn new(cs: &Case) -> Self {
        Self { name: cs.name.clone(), value: cs.value }
    }
}
#[wasm_bindgen]
pub struct WStatistic {
    http_method: Vec<WCase>,
    http_status: Vec<WCase>,
    http_type: Vec<WCase>,
}
#[wasm_bindgen]
impl WStatistic {
    #[wasm_bindgen(getter)]
    pub fn http_method(&self) -> Vec<WCase> {
        self.http_method.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn http_status(&self) -> Vec<WCase> {
        self.http_status.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn http_type(&self) -> Vec<WCase> {
        self.http_type.clone()
    }
}
impl WStatistic {
    pub fn new(stat: Ref<Statistic>) -> Self {
        let http_method = stat.http_method.to_list().iter().map(|f| WCase::new(f)).collect::<Vec<_>>();
        let http_status = stat.http_status.to_list().iter().map(|f| WCase::new(f)).collect::<Vec<_>>();
        let http_type = stat.http_type.to_list().iter().map(|f| WCase::new(f)).collect::<Vec<_>>();
        Self { http_method, http_status, http_type }
    }
}

#[wasm_bindgen]
pub struct  WTLSHS {
    _ins: TLSHS
}
#[wasm_bindgen]
impl WTLSHS {
    #[wasm_bindgen(getter)]
    pub fn source(&self) -> String {
        self._ins.source.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn target(&self) -> String {
        self._ins.target.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn server_name(&self) -> Vec<String> {
        self._ins.server_name.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn used_cipher(&self) -> String {
        self._ins.used_cipher.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn used_version(&self) -> String {
        self._ins.used_version.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn used_negotiation(&self) -> Vec<String> {
        self._ins.used_negotiation.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn support_cipher(&self) -> Vec<String> {
        self._ins.support_cipher.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn support_version(&self) -> Vec<String> {
        self._ins.support_version.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn support_negotiation(&self) -> Vec<String> {
        self._ins.support_negotiation.clone()
    }
}

impl WTLSHS {
    pub fn new(_ins: TLSHS) -> Self {
        Self{_ins: _ins.clone()}
    }
}