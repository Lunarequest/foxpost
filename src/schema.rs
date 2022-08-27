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
