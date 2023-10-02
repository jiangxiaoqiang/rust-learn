use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rust_wheel::common::util::net::sse_stream::SseStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use std::time::{Duration, SystemTime};

#[get("/sse")]
pub async fn sse() -> impl Responder {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
    tokio::sync::mpsc::unbounded_channel();
    let mut cmd = Command::new().stdout(Stdio::piped());
    tokio::spawn(async move {
        loop {
            let message = format!("Current time: {:?}", SystemTime::now());

            // Broadcast the message to all clients
            tx.send(message).unwrap();

            // Wait for 1 second before sending the next message
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    let response = HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(SseStream { receiver: Some(rx) });
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(sse))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}