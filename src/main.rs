#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod api_response;
mod login_user_info;
mod jwt_util;

use models::QuerySongs;
use crate::diesel::prelude::*;
use diesel::pg::expression::dsl::any;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::models::QueryFavorites;
use crate::api_response::ApiResponse;
use crate::login_user_info::LoginUserInfo;

#[get("/<id>")]
fn index(id:&str,loginUserInfo: LoginUserInfo) -> &'static str {
    return "ok";
}

#[derive(Debug, PartialEq, Eq, Deserialize,Serialize)]
#[allow(non_snake_case)]
pub struct ArtistResponse {
    pub id: i64,
    pub name: String
}

#[derive(Debug, PartialEq, Eq, Deserialize,Serialize)]
#[allow(non_snake_case)]
pub struct MusicResponse {
    pub id: i64,
    pub title: String,
    pub artist: Vec<ArtistResponse>
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


