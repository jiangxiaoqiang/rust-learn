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

fn main() {
    let config = getConfig("key").as_str();
    print!("{}",config)
}