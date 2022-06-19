table! {
    account (account_no) {
        account_no -> Integer,
        account_name -> Text,
        password -> Text,
        lastupdate -> Nullable<Timestamp>,
    }
}

table! {
    thread (thread_id) {
        thread_id -> Integer,
        thread_name -> Text,
        lastupdate -> Nullable<Timestamp>,
    }
}

table! {
    thread_comment (thread_id, comment_no) {
        thread_id -> Integer,
        comment_no -> Integer,
        comment_name -> Text,
        comment -> Text,
        lastupdate -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    account,
    thread,
    thread_comment,
);
