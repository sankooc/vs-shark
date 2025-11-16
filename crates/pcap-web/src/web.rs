use actix::ActorContext;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use include_dir::Dir;
use mime_guess;
use std::future::Future;
use std::net::IpAddr;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;

use crate::core::UICommand;

pub struct WebApplication {
    pub dir: Dir<'static>,
    pub ip: IpAddr,
    pub port: u16,
    pub receiver: mpsc::Receiver<UICommand>,
    sender: mpsc::Sender<UICommand>,
}


impl WebApplication {
    fn _resp<F, T>(future: F) -> &'static str
    where
        F: Future<Output = T> + Send,
    {
        ""
    }
    pub fn new(dir: Dir<'static>, ip: IpAddr, port: u16) -> Self {
        todo!()
        // Self { dir, ip, port, sender: None }
    }
    async fn get_info(&self) -> &'static str {
        WebApplication::_resp(self.sender.send(UICommand::None))
    }
    async fn index(&self) -> impl Responder {
        let file = self.dir.get_file("index.html").unwrap();
        HttpResponse::Ok().content_type("text/html").body(file.contents())
    }

    async fn static_file(&self, path: String) -> impl Responder {
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
        async fn get_info(app: web::Data<Arc<WebApplication>>) -> impl Responder {
            app.get_info().await
        }
        async fn index(app: web::Data<Arc<WebApplication>>) -> impl Responder {
            app.index().await
        }
        async fn get_static_file(app: web::Data<Arc<WebApplication>>, path: web::Path<String>) -> impl Responder {
            app.static_file(path.into_inner()).await
        }
        HttpServer::new(move || {
            // let app = app.clone();
            App::new()
                .app_data(web::Data::new(app.clone()))
                .route("/api/info", web::get().to(get_info))
                .route("/", web::get().to(index))
                .route("/ws/", web::get().to(websocket))
                .route("/{path:.*}", web::get().to(get_static_file))
        })
        .bind((self.ip, self.port))?
        .run()
        .await
    }
}

// 定义 WebSocket Actor
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
