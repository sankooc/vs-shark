use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::{Criteria, HttpCriteria, HttpMessageDetail};
use serde::{Deserialize, Serialize};

use crate::web::WebApplication;
#[derive(Deserialize, Serialize)]
pub struct PCriteria {
    pub size: usize,
    pub start: usize,
    pub asc: bool,
    pub host: Option<String>,
}

impl From<&PCriteria> for Criteria {
    fn from(cri: &PCriteria) -> Self {
        Criteria { size: cri.size, start: cri.start }
    }
}

impl PCriteria {
    fn filter(&self) -> Option<HttpCriteria> {
        self.host.as_ref().map(|f| HttpCriteria { hostname: Some(f.clone()) })
    }
    fn asc(&self) -> bool {
        self.asc
    }
}

#[derive(Serialize)]
struct HttpD {
    pub headers: Vec<String>,
    pub raw: Vec<u8>,
    pub plaintext: Option<String>,
    pub content_type: Option<String>,
}

impl From<&HttpMessageDetail> for HttpD {
    fn from(value: &HttpMessageDetail) -> Self {
        let headers = value.headers.clone();
        let raw = value.raw_content().to_vec();
        let plaintext = value.get_text_content();
        let content_type = value.content_type();
        Self {
            headers,
            raw,
            plaintext,
            content_type,
        }
    }
}

#[get("/list")]
async fn list(app: web::Data<Arc<WebApplication>>, query: web::Query<PCriteria>) -> HttpResponse {
    let cri = query.into_inner();
    let cri2 = &cri;
    let rs = app.engine().http_list(cri2.into(), cri2.filter(), cri2.asc()).await;
    HttpResponse::Ok().json(rs)
}

#[get("/detail/{index}")]
async fn detail(app: web::Data<Arc<WebApplication>>, path: web::Path<String>) -> HttpResponse {
    let index = path.into_inner();
    match index.parse::<usize>() {
        Ok(num) => {
            let rs = app.engine().http_detail(num).await;
            let rs2: Option<Vec<HttpD>> = rs.map(|f| f.iter().map(|e| e.into()).collect());
            HttpResponse::Ok().json(rs2)
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/http").service(list).service(detail));
}
