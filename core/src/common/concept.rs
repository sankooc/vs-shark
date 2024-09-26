use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{files::Endpoint, specs::http::HTTP};

use super::Ref2;

#[derive(Default)]
pub struct HttpRequest {
  pub source: String,
  pub dest: String,
  pub srp: u16,
  pub dsp: u16,
  pub request: Option<Ref2<HTTP>>,
  pub response: Option<Ref2<HTTP>>,
}

impl HttpRequest {
  pub fn new(source: String, dest: String, srp: u16, dsp: u16 ) -> Self {
    Self{source, dest, srp, dsp, request: None,response:None }
  }
  pub fn set_request(&mut self, http: Ref2<HTTP>){
    self.request = Some(http);
  }
  pub fn set_response(&mut self, http: Ref2<HTTP>){
    self.response = Some(http);
  }
  
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
}

pub struct Case {
  pub label: String,
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
      list.push(Case{label: k.into(), value: *v})
    }
    list
  }
}