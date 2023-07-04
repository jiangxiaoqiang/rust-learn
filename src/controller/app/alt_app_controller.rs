use actix_web::{ HttpResponse, Responder, web};

pub async fn get_apps_by_tag() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/apps")
            .route("/{tag}", web::get().to(get_apps_by_tag))
    );
}
