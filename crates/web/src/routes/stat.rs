use std::{sync::Arc};

use actix_web::{get, web, HttpResponse};

use crate::web::WebApplication;

#[get("/{tp}")]
async fn stat(app: web::Data<Arc<WebApplication>>, path: web::Path<String>) -> HttpResponse {
    let tp = path.into_inner();
    let content = app.engine().stat(tp).await;
    HttpResponse::Ok().content_type("application/json").body(content)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/stat").service(stat));
}
