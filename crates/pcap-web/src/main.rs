use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
    thread,
};

use actix_web::{web, App, HttpServer};
use clap::Parser;
use pcap_web::{control, routes::init_routes, web::WebApplication};
use tokio::runtime::Runtime;

async fn start(address: IpAddr, port: u16, target: Option<String>) -> std::io::Result<()> {
    let folder = include_dir::include_dir!("dist/socket");
    let (ui, mut engine) = util::core::build_engine();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            engine.run().await;
        });
    });
    let mut _app = WebApplication::new(folder, address, port, ui);
    _app.open(target).await;
    let app = Arc::new(_app);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app.clone()))
            .configure(init_routes)
            .route("/", web::get().to(control::index))
            // .route("/ws/", web::get().to(websocket))
            .route("/{path:.*}", web::get().to(control::get_static_file))
    })
    .bind((address, port))?
    .run()
    .await
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let fname = args.file;
    let address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 3000;
    let target: Option<String> = Some(fname);
    start(address, port, target).await
}
