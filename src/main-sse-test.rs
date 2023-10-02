use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::warn;
use rust_wheel::common::util::net::sse_stream::SseStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task;


#[get("/sse")]
pub async fn sse() -> impl Responder {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        tokio::sync::mpsc::unbounded_channel();
    warn!("start...");
    task::spawn_blocking(move ||  {
        warn!("compile result");
    });
    let response = HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(SseStream { receiver: Some(rx) });
    response
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    HttpServer::new(|| App::new().service(sse))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
