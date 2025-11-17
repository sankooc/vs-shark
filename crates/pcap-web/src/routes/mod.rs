use actix_web::web;

mod api;
mod stat;
mod tcp;
mod udp;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(udp::init);
    cfg.configure(tcp::init);
    cfg.configure(stat::init);
    cfg.configure(api::init);
}

