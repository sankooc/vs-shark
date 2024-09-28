use std::{cell::RefCell, collections::HashMap};

use serde::Serialize;

use crate::specs::http::{Request, Response, HTTP};

use super::Ref2;

#[derive(Default)]
pub struct HttpRequest {
  pub source: String,
  pub dest: String,
  pub srp: u16,
  pub dsp: u16,
  pub method: Option<String>,
  pub status: Option<String>,
  pub request: Option<Ref2<HTTP>>,
  pub response: Option<Ref2<HTTP>>,
}

impl HttpRequest {
  pub fn new(source: String, dest: String, srp: u16, dsp: u16 ) -> Self {
    Self{source, dest, srp, dsp, ..Default::default()}
  }
  pub fn set_request(&mut self, http: Ref2<HTTP>, req: &Request){
    self.request = Some(http);
    self.method = Some(req.method.clone());
  }
  pub fn set_response(&mut self, http: Ref2<HTTP>, res: &Response){
    self.response = Some(http);
    self.status = Some(res.code.clone());
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