table! {
    posts (id) {
        id -> Int4,
        uuid -> Int4,
        title -> Varchar,
        synopsis -> Nullable<Text>,
        body -> Text,
        created -> Timestamp,
    }
}

table! {
    post_topics (post_id, topic_id) {
        post_id -> Int4,
        topic_id -> Int4,
    }
}

table! {
    topics (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    user_posts (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Int4,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

table! {
    user_topics (user_id, topic_id) {
        user_id -> Int4,
        topic_id -> Int4,
    }
}

joinable!(post_topics -> posts (post_id));
joinable!(post_topics -> topics (topic_id));
joinable!(user_posts -> posts (post_id));
joinable!(user_posts -> users (user_id));
joinable!(user_topics -> topics (topic_id));
joinable!(user_topics -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    post_topics,
    topics,
    user_posts,
    users,
    user_topics,
);
