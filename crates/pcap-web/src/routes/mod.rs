use actix_web::web;

mod api;
mod stat;
mod tcp;
mod udp;
mod tls;
mod dns;
mod http;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(http::init);
    cfg.configure(dns::init);
    cfg.configure(udp::init);
    cfg.configure(tcp::init);
    cfg.configure(stat::init);
    cfg.configure(tls::init);
    cfg.configure(api::init);
}

