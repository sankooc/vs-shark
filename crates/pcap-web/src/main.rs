

use std::{net::{IpAddr, Ipv4Addr}, sync::Arc};

use include_dir::{include_dir};
use pcap_web::web::WebApplication;

// static STATIC_DIR: Dir = include_dir!("/home/sankooc/repo/pcapview/webview/dist");

// async fn index() -> impl Responder {
//     let file = STATIC_DIR.get_file("index.html").unwrap();
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(file.contents())
// }
// async fn static_files(path: web::Path<String>) -> impl Responder {
//     let path: String = path.into_inner();

//     // 空路径 => index.html
//     let path = if path.is_empty() {
//         "index.html".to_string()
//     } else {
//         path
//     };

//     match STATIC_DIR.get_file(&path) {
//         Some(file) => {
//             let mime = mime_guess::from_path(&path).first_or_octet_stream();
//             HttpResponse::Ok()
//                 .append_header(("Content-Type", mime.as_ref()))
//                 .body(file.contents())
//         }
//         None => {
//             // SPA 模式：找不到的路径 fallback 至 index.html
//             if let Some(index) = STATIC_DIR.get_file("index.html") {
//                 return HttpResponse::Ok()
//                     .content_type("text/html")
//                     .body(index.contents());
//             }

//             HttpResponse::NotFound().body("404 Not Found")
//         }
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let folder = include_dir!("dist/socket");
    let address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 3000;
    let server = Arc::new(WebApplication::new(folder, address, port));

    // let logic_handle = std::thread::spawn(move || {
    //     engine.run().unwrap();
    // });

    let _ = server.listen().await;
    Ok(())
}

