#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod api_response;
mod login_user_info;

use crate::login_user_info::LoginUserInfo;

#[get("/<id>")]
fn index(id:&str,loginUserInfo: LoginUserInfo) -> &'static str {
    return "ok";
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


