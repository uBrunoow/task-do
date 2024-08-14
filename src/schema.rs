// @generated automatically by Diesel CLI.

diesel::table! {
    priorities (priority_id) {
        priority_id -> Int4,
        #[max_length = 255]
        priority_name -> Varchar,
    }
}

diesel::table! {
    tags (tags_id) {
        tags_id -> Int4,
        #[max_length = 255]
        tag_name -> Varchar,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        priority_id -> Nullable<Int4>,
        tags_id -> Nullable<Int4>,
        created_by -> Nullable<Int4>,
        assigned_to -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(tasks -> priorities (priority_id));
diesel::joinable!(tasks -> tags (tags_id));

diesel::allow_tables_to_appear_in_same_query!(
    priorities,
    tags,
    tasks,
    users,
);
