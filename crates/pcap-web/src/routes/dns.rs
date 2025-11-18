use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::Criteria;
use serde::{Deserialize, Serialize};

use crate::web::WebApplication;
#[derive(Deserialize, Serialize)]
pub struct PCriteria {
    pub size: usize,
    pub start: usize,
    pub asc: bool,
}

impl From<&PCriteria> for Criteria {
    fn from(cri: &PCriteria) -> Self {
        Criteria {
            size: cri.size,
            start: cri.start,
        }
    }
}

impl PCriteria {
    fn asc(&self) -> bool {
        self.asc
    }
}

#[get("/list")]
async fn list(app: web::Data<Arc<WebApplication>>, query: web::Query<PCriteria>) -> HttpResponse {
    let cri = query.into_inner();
    let cri2 = &cri;
    let rs = app.engine().dns_records(cri2.into(), cri2.asc()).await;
    HttpResponse::Ok().json(rs)
}

#[get("/detail/{index}")]
async fn detail(app: web::Data<Arc<WebApplication>>, path: web::Path<String>, query: web::Query<Criteria>) -> HttpResponse {
    let cri = query.into_inner();
    let index = path.into_inner();
    match index.parse::<usize>() {
        Ok(num) => {
            let rs = app.engine().dns_record(num, cri).await;
            HttpResponse::Ok().json(rs)
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/dns").service(list).service(detail));
}
