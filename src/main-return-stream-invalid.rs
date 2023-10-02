use actix_web::{get, App, HttpResponse, HttpServer};
use rust_wheel::common::util::net::sse_stream::SseStream;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task,
};

#[get("/sse")]
pub async fn sse() -> HttpResponse {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        tokio::sync::mpsc::unbounded_channel();
        task::spawn(async move {
            render_texhub_project_sse( tx).await;
        });
    let response = HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(SseStream { receiver: Some(rx) });
    response
}

async fn render_texhub_project_sse(tx: UnboundedSender<String>) {
    //let mut cmd = Command::new("ls").stdout(Stdio::piped()).spawn();
    let mut cmd = Command::new("xelatex")
        .arg("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src/dolphin-book-2023.tex")
        .stdout(Stdio::piped())
        .current_dir("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src")
        .spawn();
    let stdout = cmd.unwrap().stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    task::spawn_blocking({
        let tx: UnboundedSender<String> = tx.clone();
        move || {
            let shared_tx = Arc::new(Mutex::new(tx));
            for line in reader.lines() {
                if let Ok(line) = line {
                    let message = format!("Current time: {:?}", line);
                    println!("{}", message);
                    shared_tx.lock().unwrap().send(message).unwrap();
                }
            }
            shared_tx.lock().unwrap().send("eof".to_string()).unwrap();
        }
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(sse))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
