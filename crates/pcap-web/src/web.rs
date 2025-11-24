use actix::ActorContext;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use include_dir::Dir;
use mime_guess;
use std::net::IpAddr;
use util::core::UIEngine;

pub struct WebApplication {
    pub dir: Dir<'static>,
    pub ip: IpAddr,
    pub port: u16,
    pub target: Option<String>,
    engine: UIEngine,
}

impl WebApplication {
    pub fn new(dir: Dir<'static>, ip: IpAddr, port: u16, engine: UIEngine) -> Self {
        Self {
            dir,
            ip,
            port,
            engine,
            target: None,
        }
    }
    pub async fn open(&mut self, target: String) -> Result<(), String>{
        self.target = Some(target.clone());
        self.engine.open_file(target).await
    }
    pub fn engine(&self) -> &UIEngine {
        &self.engine
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
                ctx.text(format!("Echo: {text}"));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(PWebSocket {}, &req, stream)
}
