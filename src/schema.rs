table! {
    dict (id) {
        word -> Nullable<Varchar>,
        phonetic -> Nullable<Varchar>,
        definition -> Nullable<Varchar>,
        translation -> Nullable<Varchar>,
        pos -> Nullable<Varchar>,
        collins -> Nullable<Int4>,
        oxford -> Nullable<Int4>,
        tag -> Nullable<Varchar>,
        bnc -> Nullable<Int4>,
        frq -> Nullable<Int4>,
        exchange -> Nullable<Varchar>,
        detail -> Nullable<Varchar>,
        audio -> Nullable<Varchar>,
        id -> Int4,
    }
}
