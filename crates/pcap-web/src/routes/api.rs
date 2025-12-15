use std::{fs, path::Path, sync::Arc};

use actix_web::{get, web, HttpResponse};
use pcap::common::concept::{Criteria, FrameIndex};
use serde::Serialize;

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

#[derive(Serialize)]
struct PFile{
    pub name: String,
    pub size: u64,
}

impl PFile {
    pub fn new(filepath: &str) -> Option<Self> {
        let path = Path::new(filepath);
        if let Ok(meta) = fs::metadata(path) {
            let size = meta.len();
            Some(Self{size, name: filepath.to_string()})
        } else {
            None
        }
    }
}

#[get("/ready")]
async fn ready(app: web::Data<Arc<WebApplication>>) -> HttpResponse {
    if let Some(pf) = app.target.clone().and_then(|f| PFile::new(f.as_str())) {
        HttpResponse::Ok().json(&pf)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[get("/metadata")]
async fn metadata(app: web::Data<Arc<WebApplication>>) -> HttpResponse {
    let rs = app.engine().metadata().await;
    if let Some(metadata) = rs {
        HttpResponse::Ok().json(&metadata)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(frame).service(frames).service(ready).service(metadata));
}
