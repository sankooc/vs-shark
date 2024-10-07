use core::{
    common::{
        concept::{Case, HttpRequestBuilder, Statistic, TLSHS},
        Ref2,
    },
    specs::http::HTTP,
};
use std::cell::Ref;

use js_sys::Uint8Array;
// use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen(getter)]
    pub fn content_len(&self) -> usize {
        let _http = self.http.as_ref().borrow();
        _http.len
    }
    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Uint8Array {
        let _http = self.http.as_ref().borrow();
        // if _http.len > 0 {
            
        // }
        let data:&[u8] = &_http.content;
        return data.into()
    }
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