#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod api_response;
mod login_user_info;
mod initial;

use ring::{digest, test};
use std::str;
use data_encoding::HEXLOWER;

fn main() {
    let expect="d2c8b081f8ed48d7953e30f613a1dc7f20c76ca6a7954c0e63793a1d25b5f5d1dbaaa99cbf3bd62c35ff467868afaeac0bd8888b3db32f4fecff22db87b04cd7";
    let password = "$mycruise123";
    let salt = "PIg6raF1xzsoaAMQ";
    let calc = password.to_string() + salt;
    let actual = digest::digest(&digest::SHA512, calc.as_ref());
    let calc_digest_str = HEXLOWER.encode(actual.as_ref());
    println!("SHA-512 digest is {}", HEXLOWER.encode(actual.as_ref()));
    //let result = format!("{:x}", actual);
}