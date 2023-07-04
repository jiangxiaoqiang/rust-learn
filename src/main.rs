#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use controller::{
    app::alt_app_controller::{self},
    health::health_controller
};

pub mod controller;
pub mod model;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(alt_app_controller::config)
            .configure(health_controller::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}