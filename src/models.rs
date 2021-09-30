use rocket::serde::Serialize;
use rocket::serde::Deserialize;

#[derive(Insertable, Serialize, Queryable, Deserialize,Default)]
#[table_name = "dict"]
pub struct QueryEdict {
    pub id: i64,
    pub word: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub pos: String,
    pub collins: String,
    pub oxford: String,
    pub tag: String,
    pub bnc: i32,
    pub frq: i32,
    pub exchange: String,
    pub detail: String,
    pub audio: String,
}
