use super::schema::*;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct students_with_id {
    pub id: i32,
    pub student_id: String,
    pub student_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_aide: bool,
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[table_name = "students"]
pub struct students_without_id {
    pub student_id: String,
    pub student_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_aide: bool,
}

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct user_login_with_id {
    pub id: i32,
    pub user_id: i32,
    pub login_time: NaiveDateTime,
    pub logout_time: NaiveDateTime,
    pub ip: String,
    pub session_id: String,
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[table_name = "user_login"]
pub struct user_login_without_id {
    pub user_id: i32,
    pub login_time: NaiveDateTime,
    pub logout_time: NaiveDateTime,
    pub ip: String,
    pub session_id: String,
}

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct users_with_id {
    pub id: i32,
    pub username: String,
    pub token: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[table_name = "users"]
pub struct users_without_id {
    pub username: String,
    pub token: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct visits_with_id {
    pub id: i32,
    pub student_id: i32,
    pub checked_in: NaiveDateTime,
    pub left_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[table_name = "visits"]
pub struct vists_without_id {
    pub student_id: i32,
    pub checked_in: NaiveDateTime,
    pub left_at: Option<NaiveDateTime>,
}