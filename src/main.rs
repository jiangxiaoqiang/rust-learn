#[macro_use] extern crate rocket;


#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;

use models::QuerySongs;
use crate::diesel::prelude::*;
use diesel::pg::expression::dsl::any;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;


#[get("/")]
fn index() -> &'static str {
    use crate::schema::songs::dsl::*;
    let connection = config::establish_connection();
    // https://stackoverflow.com/questions/69139883/how-to-do-a-in-query-using-diesel
    let results = songs.filter(id.eq(1))
        .load::<QuerySongs>(&connection)
        .expect("Error loading posts");
    for query_song in &results {
        let artists_array:Vec<ArtistResponse> = serde_json::from_str(&query_song.artists.to_owned()).unwrap();
    }
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


