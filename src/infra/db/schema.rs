// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        mail -> Text,
        hashed_password -> Text,
    }
}

diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
