#[macro_use] extern crate rocket;


#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod config;

use models::QuerySongs;
use crate::diesel::prelude::*;

#[get("/")]
fn index() -> &'static str {
    use crate::schema::songs::dsl::*;
    let connection = config::establish_connection();
    let results = songs.filter(crate::schema::songs::id.contains(vec![1, 2]))
        .load::<QuerySongs>(&connection)
        .expect("Error loading posts");
    return "ok";
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


