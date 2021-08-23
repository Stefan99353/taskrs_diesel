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
    categories (id) {
        id -> Int4,
        name -> Varchar,
        parent_category_id -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    permissions (id) {
        id -> Int4,
        name -> Varchar,
        group -> Varchar,
        description -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    user_permissions (user_id, permission_id) {
        user_id -> Int4,
        permission_id -> Int4,
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
joinable!(user_permissions -> permissions (permission_id));
joinable!(user_permissions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    auth_refresh_tokens,
    categories,
    permissions,
    user_permissions,
    users,
);
