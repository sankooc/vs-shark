

use std::{net::{IpAddr, Ipv4Addr}, sync::Arc, thread};

use include_dir::{include_dir};
use pcap_web::{core, web::WebApplication};
use tokio::runtime::Runtime;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let folder = include_dir!("dist/socket");
    let address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 3000;

    // let target = None; // 
    let (ui, mut engine) = core::build_engine();
    let logic_handle = thread::spawn(move ||  {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            engine.run().await;
        });
    });

    let server = Arc::new(WebApplication::new(folder, address, port, ui));
    let _ = server.listen().await;
    logic_handle.join().unwrap();
    Ok(())
}

