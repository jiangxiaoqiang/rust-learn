use actix_web::{web, HttpResponse, Responder};
use rust_wheel::model::response::api_response::ApiResponse;

use crate::service::tag::tag_service::get_tags_list;

pub async fn get_tags() -> impl Responder {
    let tags_list = get_tags_list();
    let res = ApiResponse {
        result: tags_list,
        ..Default::default()
    };
    HttpResponse::Ok().json(res)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tag").route("/list", web::get().to(get_tags)));
}
