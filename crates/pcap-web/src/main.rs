use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    path::Path,
    process::exit,
    sync::Arc,
    thread,
};

use actix_web::{web, App, HttpServer};
use clap::Parser;
use pcap_web::{control, routes::init_routes, web::WebApplication};
use tokio::runtime::Runtime;

async fn start(address: IpAddr, port: u16, target: String) -> std::io::Result<()> {
    let folder = include_dir::include_dir!("dist/socket");
    let (ui, mut engine) = util::core::build_engine();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            engine.run().await;
        });
    });
    let mut _app = WebApplication::new(folder, address, port, ui);
    if let Err(str) = _app.open(target.clone()).await {
        eprintln!("Error {str}: [{target}]");
        exit(1);
    }
    let app: Arc<WebApplication> = Arc::new(_app);
    banner(address.to_string(), port, &target);
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

fn is_port_in_use(port: u16) -> bool {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    TcpListener::bind(addr).is_err()
}

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = r#"
Pcap-web has started a lightweight web server for remote access.
"#
)]
struct Args {
    #[arg(short, long, help = "PCAP/PCAPng file to open")]
    file: String,
    #[arg(short, long, help = "Port to listen on")]
    port: Option<u16>,
    #[arg(short, long, default_value_t = false, help = "Listen on localhost only")]
    local: bool,
}

const DEFAULT_PORT: u16 = 6400;

fn get_port() -> Option<u16> {
    let start = DEFAULT_PORT;
    let end = start + 15;
    (start..end).find(|&port| !is_port_in_use(port))
}

fn get_host(args: &Args) -> IpAddr {
    if args.local {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    } else {
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
    }
}

fn file_exists(fname: &str) -> bool {
    let path = Path::new(fname);
    path.exists()
}
use colored::*;
fn banner(host: String, port: u16, file: &str) {
    println!("{}", "=============================================================".cyan());
    println!("{}", "                    Pcapviewer Web Service".bright_green().bold());
    println!("{}", "=============================================================".cyan());

    println!("Loaded file: {}", file.yellow());
    println!("Open in browser: {}://{}:{}",
        "http".green(),
        host.blue(),
        port.to_string().blue(),
    );

    println!("{}", "Press CTRL+C to stop the server.".dimmed());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let fname = args.file.clone();
    if !file_exists(&fname) {
        println!("File '{fname}' does not exist.");
        exit(1);
    }
    let address = get_host(&args);
    let port = if let Some(_port) = args.port {
        if is_port_in_use(_port) {
            println!("Port {_port} is already in use.");
            exit(1);
        }
        _port
    } else {
        get_port().unwrap()
    };
    start(address, port, fname.clone()).await
}
