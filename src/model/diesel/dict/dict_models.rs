// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use std::io::Write;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Jsonb;
use rocket::serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dict::dict_schema::*;

#[derive(Queryable,Debug,Serialize,Deserialize,Default)]
pub struct Test {
    pub id: i64,
    pub tags: i64,
}
