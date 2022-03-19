#[macro_use]
extern crate diesel;

mod schema;
mod models;

fn main() {
    use crate::diesel::prelude::*;
    use crate::schema::dict;
    let mut query = dict::table.into_boxed();
}