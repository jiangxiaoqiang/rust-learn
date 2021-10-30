use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use crate::schema::dict;

#[derive(Insertable, Serialize, Queryable, Deserialize,Default)]
#[table_name = "dict"]
pub struct QueryEdict {
    pub id: i64,
    pub word: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub pos: String,
    pub collins: Option<i32>,
    pub oxford: Option<i32>,
    pub tag: String,
    pub bnc: i32,
    pub frq: i32,
    pub exchange: String,
    pub detail: String,
    pub audio: String,
}
