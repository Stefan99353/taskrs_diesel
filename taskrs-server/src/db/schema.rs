table! {
    auth_refresh_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        iat -> Int8,
        exp -> Int8,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        activated -> Bool,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

joinable!(auth_refresh_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    auth_refresh_tokens,
    users,
);
