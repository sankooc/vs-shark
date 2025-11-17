use actix_web::web;

mod api;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(api::init);
}

