use std::time::{Duration, SystemTime};

use actix_web::{
    get,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use futures::stream::StreamExt;
use reqwest::Client;
use rust_wheel::{
    common::util::net::sse_stream::SseStream, config::app::app_conf_reader::get_app_config,
};
use serde_json::Value;
use tokio::{sync::mpsc::{UnboundedReceiver, UnboundedSender}, task};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TexCompileProjectReq {
    pub project_id: String,
    pub req_time: i64,
    pub file_name: String,
}

pub fn get_proj_compile_req(params: &TexCompileProjectReq, proj: &String) -> Value {
    let file_path = format!(
        "/opt/data/project/{}/{}",
        &params.project_id, params.file_name
    );
    let out_path = format!("/opt/data/project/{}", &params.project_id);
    let json_data = serde_json::json!({
        "file_path": file_path,
        "out_path": out_path,
        "req_time": params.req_time,
        "project_id": proj
    });
    return json_data;
}

#[get("/sse-generator")]
pub async fn sse() -> impl Responder {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        tokio::sync::mpsc::unbounded_channel();
    task::spawn(async move {
        loop {
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

#[get("/sse")]
async fn sse_handler(form: web::Query<TexCompileProjectReq>) -> HttpResponse {
    let (tx, rx): (UnboundedSender<String>, UnboundedReceiver<String>) =
        tokio::sync::mpsc::unbounded_channel();
    let client = Client::new();
    let url_path = format!("{}", "/render/compile/v1/project/sse");
    let url = format!("{}{}", get_app_config("texhub.render_api_url"), url_path);
    let json_data = get_proj_compile_req(&form.0, &form.project_id);
    let response = HttpResponse::Ok()
    .content_type("text/event-stream")
    .streaming(SseStream { receiver: Some(rx) });
    if let Ok(resp) = client.post(url).json(&json_data).send().await {
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
