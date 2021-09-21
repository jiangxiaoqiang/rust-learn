#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod api_response;

use models::QuerySongs;
use crate::diesel::prelude::*;
use diesel::pg::expression::dsl::any;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::models::QueryFavorites;
use crate::api_response::ApiResponse;

#[get("/")]
fn index() -> &'static str {
    use crate::schema::favorites::dsl::*;
    let connection = config::establish_connection();
    // https://stackoverflow.com/questions/69139883/how-to-do-a-in-query-using-diesel
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


