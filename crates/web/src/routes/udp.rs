use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::Criteria;
use serde::{Deserialize, Serialize};

use crate::web::WebApplication;

#[derive(Deserialize, Serialize)]
struct TCPCriteria {
    pub size: usize,
    pub start: usize,
    pub ip: Option<String>,
    pub asc: bool,
}

impl From<&TCPCriteria> for Criteria {
    fn from(cri: &TCPCriteria) -> Self {
        Criteria { size: cri.size, start: cri.start }
    }
}
impl TCPCriteria {
    fn fiter(&self) -> Option<String> {
        self.ip.clone()
    }
    fn asc(&self) -> bool {
        self.asc
    }
}

#[get("/list")]
async fn list(app: web::Data<Arc<WebApplication>>, query: web::Query<TCPCriteria>) -> HttpResponse {
    let cri: TCPCriteria = query.into_inner();
    let cri2 = &cri;
    let rs = app.engine().udp_list(cri2.into(), cri2.fiter(), cri2.asc()).await;
    HttpResponse::Ok().json(rs)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/udp").service(list));
}
