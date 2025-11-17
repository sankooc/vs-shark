use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::Criteria;

use crate::web::WebApplication;

#[get("/list")]
async fn list_users() -> HttpResponse {
    // let aa = app.get_info().await;
    HttpResponse::Ok().body("test")
}
// pub async fn get_info(app: web::Data<Arc<WebApplication>>) -> impl Responder {
//     app.get_info().await
// }

#[get("/frames")]
async fn frames(app: web::Data<Arc<WebApplication>>, query: web::Query<Criteria>) -> HttpResponse {
    let rs = app.engine().frames(query.into_inner()).await;
    HttpResponse::Ok().json(rs)
}

#[get("/ready")]
async fn ready(app: web::Data<Arc<WebApplication>>) -> HttpResponse {
    if let Some(filepath) = &app.target {
        let _ = app.engine().open_file(filepath.into()).await;
        HttpResponse::Ok().body("ok")
    } else {
        HttpResponse::Ok().body("error")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(frames).service(list_users).service(ready));
}
