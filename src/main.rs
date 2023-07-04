#[macro_use]
extern crate diesel;

use diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::{model::diesel::alt::custom_alt_models::AltTag, common::database::get_connection};

pub mod model;
pub mod common;

fn main() {
    use crate::model::diesel::alt::alt_schema::alt_tag as cv_work_table;
    let query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    let _cvs = query
        .load::<AltTag>(&mut get_connection())
        .expect("error get tags list");
}