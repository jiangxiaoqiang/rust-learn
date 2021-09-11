use super::schema::posts;
use super::schema::playing_record;
use super::schema::favorites;
use super::schema::songs;
use super::schema::playlist;
use super::schema::album;
use rocket::serde::Serialize;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Insertable)]
#[table_name="playing_record"]
pub struct NewPlayRecord<'a> {
    pub user_id: &'a i64,
    pub source_id: &'a str,
}

#[derive(Insertable,Serialize)]
#[table_name="songs"]
pub struct Songs {
    pub name: String,
    pub artists: String,
    pub album_id: i64,
    pub publishtime: i64,
    pub status: i32,
    pub duration: i32,
    pub source_id: String,
    pub source: i32,
    pub created_time: i64,
    pub updated_time: i64
}

#[derive(Insertable,Serialize,Queryable)]
#[table_name="songs"]
pub struct QuerySongs {
    pub id: i64,
    pub name: String,
    pub artists: String,
    pub album_id: i64,
    pub publishtime: i64,
    pub status: i32,
    pub duration: i32,
    pub source_id: String,
    pub source: i32,
    pub created_time: i64,
    pub updated_time: i64
}

#[derive(Insertable,Serialize,Queryable)]
#[table_name="playlist"]
pub struct QueryPlaylist {
    pub id: i64,
    pub creator: i64,
    pub name: String,
    pub cover_url: String,
    pub description: Option<String>,
    pub subscribed: Option<i32>,
    pub subscribed_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub share_count: Option<i32>,
    pub play_count: Option<i32>,
    pub source: i32
}

#[derive(Insertable,Serialize,Queryable)]
#[table_name="favorites"]
pub struct Favorites {
    pub user_id:  i64,
    pub source_id:  String,
    pub created_time:  i64,
    pub updated_time:  i64,
    pub like_status:  i32,
    pub source: i32
}

#[derive(Insertable,Serialize,Queryable)]
#[table_name="favorites"]
pub struct QueryFavorites {
    pub id: i64,
    pub song_id: Option<i64>,
    pub created_time:  i64,
    pub updated_time:  i64,
    pub user_id:  i64,
    pub source_id:  String,
    pub like_status:  i32,
    pub source: i32,
    pub playlist_id: i64
}

#[derive(Insertable)]
#[table_name="album"]
pub struct Album<'a> {
    pub name: &'a str,
    pub cover_image_url: &'a str,
    pub source_id: &'a str,
    pub source: &'a i32,
    pub created_time: &'a i64,
    pub updated_time: &'a i64
}

#[derive(Queryable,Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}