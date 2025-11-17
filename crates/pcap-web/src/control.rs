use std::sync::Arc;

use actix_web::{web, Responder};

use crate::web::{WebApplication};


pub async fn ready(app: web::Data<Arc<WebApplication>>) -> impl Responder {
    app.ready().await
}
pub async fn get_info(app: web::Data<Arc<WebApplication>>) -> impl Responder {
    app.get_info().await
}
pub async fn index(app: web::Data<Arc<WebApplication>>) -> impl Responder {
    app.index().await
}
pub async fn get_static_file(app: web::Data<Arc<WebApplication>>, path: web::Path<String>) -> impl Responder {
    app.static_file(path.into_inner()).await
}
