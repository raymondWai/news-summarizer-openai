// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Int4,
        title -> Text,
        url -> Text,
        keywords -> Nullable<Array<Nullable<Text>>>,
        creator -> Nullable<Array<Nullable<Text>>>,
        video_source -> Nullable<Text>,
        description -> Text,
        content -> Text,
        date -> Timestamp,
        image_url -> Nullable<Text>,
        source_id -> Int4,
        language -> Nullable<Text>,
        country -> Nullable<Array<Nullable<Text>>>,
        category -> Nullable<Array<Nullable<Text>>>,
        sentiment -> Nullable<Text>,
        sentiment_stat -> Nullable<Jsonb>,
    }
}

diesel::table! {
    source (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        country -> Text,
        language -> Text,
    }
}

diesel::joinable!(article -> source (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    article,
    source,
);
