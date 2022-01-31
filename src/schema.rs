table! {
    students (id) {
        id -> Int4,
        student_id -> Varchar,
        student_name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_aide -> Bool,
    }
}

table! {
    user_login (id) {
        id -> Int4,
        user_id -> Int4,
        login_time -> Timestamp,
        logout_time -> Timestamp,
        ip -> Varchar,
        session_id -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        token -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    visits (id) {
        id -> Int4,
        student_id -> Int4,
        checked_in -> Timestamp,
        left_at -> Nullable<Timestamp>,
    }
}

joinable!(visits -> students (student_id));

allow_tables_to_appear_in_same_query!(
    students,
    user_login,
    users,
    visits,
);
