use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::Criteria;

use crate::web::WebApplication;


#[get("/list")]
async fn list(app: web::Data<Arc<WebApplication>>, query: web::Query<Criteria>) -> HttpResponse {
    let cri = query.into_inner();
    let rs = app.engine().tls_list(cri).await;
    HttpResponse::Ok().json(rs)
}

#[get("/detail/{index}")]
async fn detail(app: web::Data<Arc<WebApplication>>, path: web::Path<String>, query: web::Query<Criteria>) -> HttpResponse {
    let cri = query.into_inner();
    let index = path.into_inner();
    match index.parse::<usize>() {
        Ok(num) => {
            let rs = app.engine().tls_detail(num, cri).await;
            HttpResponse::Ok().json(rs)
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/tls").service(list).service(detail));
}
