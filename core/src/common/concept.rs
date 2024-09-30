use std::{cell::RefCell, collections::{HashMap, HashSet}};

use serde::Serialize;

use crate::specs::http::{Request, Response, HTTP};

use super::Ref2;

pub struct HttpMessage {
  
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
  pub fn new(source: String, dest: String, srp: u16, dsp: u16 ) -> Self {
    Self{source, dest, srp, dsp, ..Default::default()}
  }
  pub fn set_request(&mut self, http: Ref2<HTTP>, req: &Request, ts: u64){
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
pub struct StatisticV {
  http_method: Vec<Case>,
  http_status: Vec<Case>,
  http_type: Vec<Case>,
}

#[derive(Default)]
pub struct Statistic {
  pub http_method: CaseGroup,
  pub http_status: CaseGroup,
  pub http_type: CaseGroup,
}

impl Statistic {
  pub fn new() -> Self{
    Self{..Default::default()}
  }
  pub fn to_json(&self) -> String {
    let enti = StatisticV{
      http_method: self.http_method.to_list(),
      http_status: self.http_status.to_list(),
      http_type: self.http_type.to_list(),
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
  _map: RefCell<HashMap<String, usize>>
}
impl CaseGroup {
  pub fn new() -> Self{
    Self{_map: RefCell::new(HashMap::new())}
  }
  pub fn inc(&self, name: &str) {
    let bind = self._map.borrow();
    let val = *(bind.get(name.into()).unwrap_or(&0));
    drop(bind);
    let mut bind2 = self._map.borrow_mut();
    bind2.insert(name.into(), val + 1);
    drop(bind2);
  }
  pub fn to_list(&self) -> Vec<Case> {
    let bind = self._map.borrow();
    let mut list = Vec::new();
    for (k, v) in bind.iter() {
      list.push(Case{name: k.into(), value: *v})
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
    Self{name, data}
  }
}

#[derive(Serialize)]
pub struct Lines {
  x: Vec<String>,
  y: HashSet<String>,
  data: Vec<LineData>,
}

impl Lines {
  pub fn new(x: Vec<String>,y: HashSet<String>, data: Vec<LineData>) -> Self {
    Self{x,y, data}
  }
  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

#[derive(Serialize,Default)]
pub struct PCAPInfo{
  pub file_type: String,
  pub start_time: u64,
  pub end_time: u64,
  pub frame_count: usize,
  pub http_count: usize,
  pub dns_count: usize,
  pub tcp_count: usize,
}

impl PCAPInfo {
  pub fn new() -> Self {
    Self{..Default::default()}
  }
  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

#[derive(Default)]
pub struct TCPWrap{
  pub source_ip: String,
  pub source_port: u16,
  pub source_host: String,
  pub target_ip: String,
  pub target_port: u16,
  pub target_host: String,
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