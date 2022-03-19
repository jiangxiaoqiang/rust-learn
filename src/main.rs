#[macro_use]
extern crate diesel;

pub mod model;

fn main() {
    use crate::diesel::prelude::*;
    let mut query = crate::model::diesel::dict::dict_schema::dict::table.into_boxed();
}