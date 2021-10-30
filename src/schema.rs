table! {
    dict (id) {
        word -> Varchar,
        phonetic -> Varchar,
        definition -> Varchar,
        translation -> Varchar,
        pos -> Varchar,
        collins -> Nullable<Int4>,
        oxford -> Nullable<Int4>,
        tag -> Varchar,
        bnc -> Int4,
        frq -> Int4,
        exchange -> Varchar,
        detail -> Varchar,
        audio -> Varchar,
        id -> Int8,
    }
}
