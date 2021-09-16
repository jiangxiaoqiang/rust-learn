#[macro_use] extern crate rocket;


#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;
mod api_response;

use models::QuerySongs;
use crate::diesel::prelude::*;
use crate::models::QueryFavorites;
use crate::api_response::ApiResponse;

#[get("/")]
fn index() -> &'static str {
    use crate::schema::favorites::dsl::*;
    let connection = config::establish_connection();
    let input_song_id = record.id;
    let post = diesel::update(favorites.find(id))
        .set(play_count.eq(200))
        .get_result::<QueryFavorites>(&connection)
        .expect(&format!("Unable to find post {}", input_song_id));
    let res = ApiResponse {
        result: post,
        ..Default::default()
    };
    let response_json = serde_json::to_string(&res).unwrap();
    return "ok";
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


