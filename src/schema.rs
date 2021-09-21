table! {
    album (id) {
        id -> Int8,
        name -> Varchar,
        cover_image_url -> Varchar,
        source_id -> Varchar,
        source -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
    }
}

table! {
    favorites (id) {
        id -> Int8,
        song_id -> Nullable<Int8>,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Int8,
        source_id -> Varchar,
        like_status -> Int4,
        source -> Int4,
        playlist_id -> Int8,
        play_count -> Int4,
    }
}

table! {
    musicians (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        img1v1url -> Nullable<Varchar>,
    }
}

table! {
    playing_record (id) {
        id -> Int8,
        source_id -> Nullable<Varchar>,
        source -> Nullable<Int4>,
        created_time -> Nullable<Int8>,
        updated_time -> Nullable<Int8>,
        user_id -> Nullable<Int8>,
    }
}

table! {
    playlist (id) {
        id -> Int8,
        creator -> Int8,
        name -> Varchar,
        cover_url -> Varchar,
        description -> Nullable<Varchar>,
        subscribed -> Nullable<Int4>,
        subscribed_count -> Nullable<Int8>,
        comment_count -> Nullable<Int8>,
        share_count -> Nullable<Int4>,
        play_count -> Nullable<Int4>,
        source -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    songs (id) {
        id -> Int8,
        name -> Varchar,
        artists -> Varchar,
        album_id -> Int8,
        publishtime -> Int8,
        status -> Int4,
        duration -> Int4,
        source_id -> Varchar,
        source -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        album -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    album,
    favorites,
    musicians,
    playing_record,
    playlist,
    posts,
    songs,
);
