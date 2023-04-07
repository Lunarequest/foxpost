// @generated automatically by Diesel CLI.

diesel::table! {
    completed_requests (id) {
        id -> Int4,
        source -> Text,
        target -> Text,
        content -> Text,
        author -> Text,
        author_url -> Text,
        url -> Text,
    }
}

diesel::table! {
    pending_requests (id) {
        id -> Int4,
        source -> Text,
        target -> Text,
    }
}

diesel::table! {
    posts (slug) {
        slug -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        draft -> Bool,
        author -> Varchar,
        published -> Int8,
        tags -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    tags (tag) {
        tag -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        passwd -> Varchar,
        isadmin -> Bool,
        confirmed -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    completed_requests,
    pending_requests,
    posts,
    tags,
    users,
);
