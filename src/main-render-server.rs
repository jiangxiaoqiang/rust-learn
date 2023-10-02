use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::warn;
use rust_wheel::common::util::net::sse_stream::SseStream;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task;

fn run_xelatex(tx: UnboundedSender<String>) -> io::Result<String> {
    tx.send("inner".to_owned()).unwrap();
    let mut cmd = Command::new("xelatex")
        .arg("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src/dolphin-book-2023.tex")
        .stdout(Stdio::piped())
        .current_dir("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src")
        .spawn()?;

    let stdout = cmd.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    tx.send("a".to_owned()).unwrap();
    tokio::spawn(async move {
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
                let send_result = tx.send(line);
                thread::sleep(Duration::from_secs(1));
                match send_result {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("send xelatex compile log error: {}", e);
                    }
                }
            }
        }
    });

    let status = cmd.wait()?;

    if status.success() {
        Ok("Compilation successful".to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Compilation failed"))
    }
}

#[derive(serde::Deserialize)]
pub struct CompileAppParams {
    pub file_path: String,
    pub out_path: String,
    pub project_id: String,
    pub req_time: i64,
}

fn run_xelatex_demo(tx: UnboundedSender<String>) -> io::Result<String> {
    tx.send("inner".to_owned()).unwrap();
    let mut cmd = Command::new("xelatex")
        .arg("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src/dolphin-book-2023.tex")
        .stdout(Stdio::piped())
        .current_dir("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src")
        .spawn()?;

    let stdout = cmd.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    tx.send("a".to_owned()).unwrap();

    let sse_send_handle = task::spawn_blocking(move ||{
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
                let send_result = tx.send(line);
                thread::sleep(Duration::from_secs(1));
                match send_result {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("send xelatex compile log error: {}", e);
                    }
                }
            }
        }
    });
    sse_send_handle.await.unwrap();

    return Ok("ok".to_owned());
}

#[get("/sse")]
pub async fn sse() -> impl Responder {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        tokio::sync::mpsc::unbounded_channel();
    task::spawn(async move {
        let output = run_xelatex_demo(tx);
        warn!("compile result: {}", output.unwrap());

        //let message = format!("Current time: {:?}", SystemTime::now());
        // Broadcast the message to all clients
        //tx.send(message).unwrap();
        // Wait for 1 second before sending the next message
        //tokio::time::sleep(Duration::from_secs(1)).await;
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
