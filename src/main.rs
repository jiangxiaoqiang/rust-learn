#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;

mod schema;
mod models;

#[get("/<id>")]
fn index(id:&str) -> &'static str {
    return "ok";
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


