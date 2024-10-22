use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use serde::Serialize;

use crate::specs::http::{HttpType, Request, Response, HTTP};

use super::{
    base::{Context, DomainService, Element, Endpoint, Frame},
    Ref2, FIELDSTATUS,
};

#[derive(Default)]
pub struct TCPConnectInfo {
    pub count: u16,
    pub throughput: u32,
    pub retransmission: u16,
    pub invalid: u16,
}

#[derive(Default)]
pub struct HttpRequestBuilder {
    pub source: String,
    pub dest: String,
    pub srp: u16,
    pub dsp: u16,
    pub start: u64,
    pub end: u64,
    pub method: Option<String>,
    pub status: Option<String>,
    pub request: Option<Ref2<HTTP>>,
    pub response: Option<Ref2<HTTP>>,
}

impl HttpRequestBuilder {
    pub fn new(source: String, dest: String, srp: u16, dsp: u16) -> Self {
        Self { source, dest, srp, dsp, ..Default::default() }
    }
    pub fn set_request(&mut self, http: Ref2<HTTP>, req: &Request, ts: u64) {
        self.request = Some(http);
        self.method = Some(req.method.clone());
        self.start = ts;
    }
    pub fn set_response(&mut self, http: Ref2<HTTP>, res: &Response, ts: u64) {
        self.response = Some(http);
        self.status = Some(res.code.clone());
        self.end = ts;
        // let request = self.request.take();
        // let req = request?;

        // Some(())
    }
}

#[derive(Serialize)]
pub struct Connect<T> {
    pub index: usize,
    pub source: String,
    pub target: String,
    pub list: Vec<T>,
}
#[derive(Default, Serialize)]
pub struct HttpMessage {
    pub ts: u64,
    head: String,
    headers: Vec<String>,
    method: String,
    _type: Option<String>,
    path: String,
    len: usize,
    #[serde(skip_serializing)]
    pub body: Option<Rc<Vec<u8>>>,
}

impl HttpMessage {
    pub fn new(ts: u64, _msg: &HTTP) -> Self {
        let head = _msg.head();
        let headers = _msg.header();
        let body = _msg.content.clone();
        let _type = _msg.content_type.clone();
        let len = _msg.len;
        let (method, path) = match _msg._type() {
            HttpType::REQUEST(req) => (req.method.clone(), req.path.clone()),
            HttpType::RESPONSE(res) => (res.code.clone(), res.status.clone()),
            HttpType::NONE => ("".into(), "".into()),
        };

        Self { ts, head, headers, body, _type, method, path, len }
    }
}

#[derive(Serialize)]
pub struct StatisticV {
    http_method: Vec<Case>,
    http_status: Vec<Case>,
    http_type: Vec<Case>,
    ip: Vec<Case>,
    ip_type: Vec<Case>,
}

#[derive(Default)]
pub struct Statistic {
    pub http_method: CaseGroup,
    pub http_status: CaseGroup,
    pub http_type: CaseGroup,
    pub ip: CaseGroup,
    pub ip_type: CaseGroup,
}

impl Statistic {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn to_json(&self) -> String {
        let enti = StatisticV {
            http_method: self.http_method.to_list(),
            http_status: self.http_status.to_list(),
            http_type: self.http_type.to_list(),
            ip: self.ip.to_list(),
            ip_type: self.ip_type.to_list(),
        };
        serde_json::to_string(&enti).unwrap()
    }
}

#[derive(Serialize)]
pub struct Case {
    pub name: String,
    pub value: usize,
}

#[derive(Default)]
pub struct CaseGroup {
    // _map: RefCell<HashMap<String, usize>>
    _map: HashMap<String, usize>,
}
impl CaseGroup {
    pub fn new() -> Self {
        Self { _map: HashMap::new() }
    }
    pub fn get_map(&self) -> &HashMap<String, usize> {
        &self._map
    }
    pub fn inc(&mut self, name: &str) {
        let val = self.get(name);
        self._map.insert(name.into(), val + 1);
    }
    pub fn get(&self, name: &str) -> usize {
        *(self._map.get(name.into()).unwrap_or(&0))
    }
    pub fn to_list(&self) -> Vec<Case> {
        let mut list = Vec::new();
        for (k, v) in self._map.iter() {
            list.push(Case { name: k.into(), value: *v })
        }
        list
    }
}

#[derive(Serialize)]
pub struct LineData {
    name: String,
    data: Vec<u32>,
}
impl LineData {
    pub fn new(name: String, data: Vec<u32>) -> Self {
        Self { name, data }
    }
}

#[derive(Default, Serialize)]
pub struct Lines {
    x: Vec<String>,
    y: HashSet<String>,
    data: Vec<LineData>,
}

impl Lines {
    pub fn new(x: Vec<String>, y: HashSet<String>, data: Vec<LineData>) -> Self {
        Self { x, y, data }
    }
    pub fn empty() -> Self {
        Lines{..Default::default()}
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Default)]
pub struct PCAPInfo {
    pub file_type: String,
    pub start_time: u64,
    pub end_time: u64,
    pub frame_count: usize,
    pub http_count: usize,
    pub dns_count: usize,
    pub tcp_count: usize,
    pub tls_count: usize,
    pub cost: usize,
}

impl PCAPInfo {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Default)]
pub struct TCPWrap {
    pub source_ip: String,
    pub source_port: u16,
    pub source_host: String,
    pub target_ip: String,
    pub target_port: u16,
    pub target_host: String,
    pub count: u16,
    pub throughput: u32,
}

#[derive(Default)]
pub struct EndpointWrap {
    pub ip: String,
    pub port: u16,
    pub host: String,
    pub count: u16,
    pub throughput: u32,
}

#[allow(dead_code)]
pub struct IPINFO {
    private: usize,
    loopback: usize,
    broadcast: usize,
    multicast: usize,
    unicast: usize,
}

#[derive(Default, Clone, Serialize)]
pub struct TLSHS {
    pub source: String,
    pub target: String,
    pub server_name: Vec<String>,
    pub support_version: Vec<String>,
    pub support_cipher: Vec<String>,
    pub support_negotiation: Vec<String>,
    pub used_version: String,
    pub used_cipher: String,
    pub used_negotiation: Vec<String>,
}

#[derive(Serialize)]
pub struct DNSRecord {
    name: String,
    _type: String,
    proto: String,
    class: String,
    content: String,
    pub ttl: u32,
}

impl DNSRecord {
    pub fn create(data: &dyn DomainService) -> DNSRecord {
        DNSRecord {
            // from: tcp.
            name: data.name(),
            _type: data._type(),
            proto: data.proto(),
            class: data.class(),
            content: data.content(),
            ttl: data.ttl(),
        }
    }
}

#[derive(Serialize)]
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
        Self {
            ip,
            port,
            host,
            count: info.count,
            throughput: info.throughput,
            retransmission: info.retransmission,
            invalid: info.invalid,
        }
    }
}

#[derive(Serialize)]
pub struct TCPConversation {
    source: WEndpoint,
    target: WEndpoint,
}
impl TCPConversation {
    pub fn new(s: &Endpoint, t: &Endpoint, ctx: &Context) -> Self {
        let source = WEndpoint::new(s, ctx);
        let target = WEndpoint::new(t, ctx);
        Self { source, target }
    }
}

#[derive(Serialize,Default)]
pub struct FrameInfo {
    index: u32,
    time: u32,
    source: String,
    dest: String,
    protocol: String,
    len: u32,
    irtt: u16,
    info: String,
    status: String,
}

impl FrameInfo {
    pub fn new(frame: &Frame, start: u64) -> Self {
        let mut item = FrameInfo { ..Default::default() };
        let sum = &frame.summary;
        item.index = sum.index;
        if frame.ts > start {
          item.time = (frame.ts - start) as u32;
        }
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
                // ele.status();
                item.status = _convert(ele.status()).into();
            }
            _ => {}
        }
        item.irtt = 1;
        item
    }
}

pub struct Criteria{
  pub criteria: String,
  pub size: usize,
  pub start: usize,
}

#[derive(Serialize)]
pub struct ListResult<T> {
  items: Vec<T>,
  pub total: usize,
  start: usize,
}

impl<T> ListResult<T> {
  pub fn new(start:usize, total:usize, items: Vec<T>) -> Self{
    Self{start, total, items}
  }
}

fn _convert(f_status: FIELDSTATUS) -> &'static str {
  match f_status {
      FIELDSTATUS::WARN => "deactive",
      FIELDSTATUS::ERROR => "errordata",
      _ => "info"
  }
}


#[derive(Default, Clone, Serialize)]
pub struct Field {
    #[serde(skip_serializing)]
    pub start: usize,
    #[serde(skip_serializing)]
    pub size: usize,
    pub summary: String,
    #[serde(skip_serializing)]
    pub data: Rc<Vec<u8>>,
    pub children: Vec<Field>,
}
impl Field {
    pub fn new(start: usize, size: usize, data: Rc<Vec<u8>>, summary: String) -> Field {
        Field {
            start,
            size,
            data,
            summary,
            children: Vec::new(),
        }
    }
    pub fn new2(summary: String, data: Rc<Vec<u8>>, vs: Vec<Field>) -> Field {
        Field {
            start: 0,
            size: 0,
            data,
            summary,
            children: vs,
        }
    }
    pub fn new3(summary: String) -> Field {
        Field {
            start: 0,
            size: 0,
            data: Rc::new(Vec::new()),
            summary,
            children: Vec::new(),
        }
    }
}

impl Field {
    pub fn summary(&self) -> String {
        self.summary.clone()
    }

    pub fn children(&self) -> &[Field] {
        &self.children
    }
}