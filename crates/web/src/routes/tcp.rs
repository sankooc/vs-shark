use std::{sync::Arc};

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::{ConversationCriteria, Criteria};
use serde::{Deserialize, Serialize};

use crate::web::WebApplication;

#[derive(Deserialize, Serialize)]
pub struct TCPCriteria {
    pub size: usize,
    pub start: usize,
    pub ip: Option<String>,
}

impl From<&TCPCriteria> for Criteria {
    fn from(cri: &TCPCriteria) -> Self {
        Criteria {
            size: cri.size,
            start: cri.start,
        }
    }
}
impl From<&TCPCriteria> for ConversationCriteria {
    fn from(cri: &TCPCriteria) -> Self {
        ConversationCriteria {
            ip: cri.ip.clone(),
        }
    }
}


#[get("/list")]
async fn conversations(app: web::Data<Arc<WebApplication>>, query: web::Query<TCPCriteria>) -> HttpResponse {
    let cri: TCPCriteria = query.into_inner();
    let cri2 = &cri;
    let rs = app.engine().conversations(cri2.into(), cri2.into()).await;
    HttpResponse::Ok().json(rs)
}

#[get("/conv/{index}/list")]
async fn connections(app: web::Data<Arc<WebApplication>>, path: web::Path<String>, query: web::Query<Criteria>) -> HttpResponse {
    let index = path.into_inner();
    match index.parse::<usize>() {
        Ok(num) => {
            let cri = query.into_inner();
            let rs = app.engine().connections(num, cri).await;
            HttpResponse::Ok().json(rs)
        },
        Err(_) => {
            HttpResponse::BadRequest().finish()
        },
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/tcp").service(conversations).service(connections));
}
