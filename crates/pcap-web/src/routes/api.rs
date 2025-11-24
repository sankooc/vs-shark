use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::{Criteria, FrameIndex};

use crate::web::WebApplication;


#[get("/frames")]
async fn frames(app: web::Data<Arc<WebApplication>>, query: web::Query<Criteria>) -> HttpResponse {
    let rs = app.engine().frames(query.into_inner()).await;
    HttpResponse::Ok().json(rs)
}

//Frame
#[get("/frame/{index}")]
async fn frame(app: web::Data<Arc<WebApplication>>, path: web::Path<String>) -> HttpResponse {
    let index = path.into_inner();
    match index.parse::<usize>() {
        Ok(num) => {
            let rs = app.engine().frame(num as FrameIndex).await;
            HttpResponse::Ok().json(rs)
        },
        Err(_) => {
            HttpResponse::BadRequest().finish()
        },
    }
}


#[get("/ready")]
async fn ready(_app: web::Data<Arc<WebApplication>>) -> HttpResponse {
    HttpResponse::Ok().body("ok")
    // if let Some(filepath) = &app.target {
    //     let _ = app.engine().open_file(filepath.into()).await;
    //     HttpResponse::Ok().body("ok")
    // } else {
    //     HttpResponse::Ok().body("error")
    // }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(frame).service(frames).service(ready));
}
