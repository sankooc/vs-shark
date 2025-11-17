use actix::ActorContext;
use actix::{Actor, StreamHandler};
use actix_web::http::StatusCode;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use include_dir::Dir;
use mime_guess;
use std::future::Future;
use std::net::IpAddr;
use std::sync::Arc;

use crate::control;
use crate::core::{UIEngine};

pub struct WebApplication {
    pub dir: Dir<'static>,
    pub ip: IpAddr,
    pub port: u16,
    target: Option<String>,
    engine: UIEngine,
}


impl WebApplication {
    pub fn new(dir: Dir<'static>, ip: IpAddr, port: u16, engine: UIEngine) -> Self {
        Self { dir, ip, port, engine, target: None }
    }
    pub async fn get_info(&self) -> impl Responder {
        let res = self.engine.get_list().await;
        res.customize().with_status(StatusCode::OK)
    }
    pub async fn ready(&self) -> impl Responder{
        let filepath = "//".to_string();
        self.engine.open_file(filepath).await
    }
    pub async fn index(&self) -> impl Responder {
        let file = self.dir.get_file("index.html").unwrap();
        HttpResponse::Ok().content_type("text/html").body(file.contents())
    }

    pub async fn static_file(&self, path: String) -> impl Responder {
        let path = if path.is_empty() { "index.html".into() } else { path };

        match self.dir.get_file(&path) {
            Some(file) => {
                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                HttpResponse::Ok().append_header(("Content-Type", mime.as_ref())).body(file.contents())
            }
            None => match self.dir.get_file("index.html") {
                Some(index) => HttpResponse::Ok().content_type("text/html").body(index.contents()),
                None => HttpResponse::NotFound().body("404 Not Found"),
            },
        }
    }

    pub async fn listen(self: Arc<Self>) -> std::io::Result<()> {
        let app = self.clone();
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(app.clone()))
                .route("/api/ready", web::get().to(control::ready))
                .route("/api/info", web::get().to(control::get_info))
                .route("/", web::get().to(control::index))
                .route("/ws/", web::get().to(websocket))
                .route("/{path:.*}", web::get().to(control::get_static_file))
        })
        .bind((self.ip, self.port))?
        .run()
        .await
    }
}

struct PWebSocket;

impl Actor for PWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("Received: {}", text);
                ctx.text(format!("Echo: {}", text));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket closed: {:?}", reason);
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(PWebSocket {}, &req, stream);
    println!("WebSocket connection attempt: {:?}", res);
    res
}
