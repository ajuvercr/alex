table! {
    posts (id) {
        id -> Int4,
        uuid -> Int4,
        title -> Varchar,
        body -> Text,
        created -> Timestamp,
    }
}

table! {
    topics (post_id, user_id) {
        post_id -> Int4,
        user_id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Int4,
        email -> Varchar,
        name -> Varchar,
        password_hash -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    topics,
    users,
);
