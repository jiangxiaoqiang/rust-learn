use actix_web::{get, App, HttpResponse, HttpServer, Responder, post};
use futures::stream::StreamExt;
use reqwest::Client;
use rust_wheel::common::util::net::sse_stream::SseStream;
use std::time::{Duration, SystemTime};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task,
};

#[post("/sse-producer")]
pub async fn sse() -> impl Responder {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        tokio::sync::mpsc::unbounded_channel();
    task::spawn(async move {
        for _ in 0..15 {
            let message = format!("Current time: {:?}", SystemTime::now());
            tx.send(message).unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    let response = HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(SseStream { receiver: Some(rx) });
    response
}

#[get("/sse-consumer")]
async fn sse_handler() -> HttpResponse {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        tokio::sync::mpsc::unbounded_channel();
    let client = Client::new();
    let response = HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(SseStream { receiver: Some(rx) });
    if let Ok(resp) = client
        .post("http://localhost:8000/sse-producer")
        .send()
        .await
    {
        tokio::spawn(async move {
            let mut body = resp.bytes_stream();
            while let Some(item) = body.next().await {
                if let Ok(chunk) = item {
                    let str = std::str::from_utf8(&chunk);
                    let string_resp = str.unwrap().to_string();
                    println!("recieve producer message:{}", string_resp);
                    if tx.send(string_resp).is_err() {
                        break;
                    }
                } else {
                    break;
                }
            }
        });
    }
    response
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    HttpServer::new(|| App::new().service(sse_handler).service(sse))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
