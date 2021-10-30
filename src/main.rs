#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod api_response;
mod login_user_info;
mod initial;

use crate::initial::getConfig;
use crate::login_user_info::LoginUserInfo;

#[get("/<id>")]
fn index(id:&str,loginUserInfo: LoginUserInfo) -> &'static str {
    use crate::schema::dict::dsl::*;
    return "ok";
}

//#[launch]
//fn rocket() -> _ {
//    getConfig("key");
//    rocket::build().mount("/", routes![index])
//}


fn main() {
    let config = getConfig("key").as_str();
    print!("{}",config)
}