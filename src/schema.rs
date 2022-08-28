table! {
    posts (slug) {
        slug -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        draft -> Bool,
        author -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        passwd -> Varchar,
        isadmin -> Bool,
        salt -> Text,
    }
}

allow_tables_to_appear_in_same_query!(posts, users,);
