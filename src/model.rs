use crate::util::log_warn;

use super::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct students_with_id {
    pub id: i32,
    pub seven_id: String,
    pub student_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_aide: bool,
}
impl Default for students_with_id {
    fn default() -> Self {
        Self {
            id: Default::default(),
            seven_id: Default::default(),
            student_name: Default::default(),
            email: Default::default(),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
            is_aide: Default::default(),
        }
    }
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
#[table_name = "students"]
pub struct students_without_id {
    pub seven_id: String,
    pub student_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_aide: bool,
}

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct user_login_with_id {
    pub id: i32,
    pub user_id: i32,
    pub login_time: NaiveDateTime,
    pub logout_time: NaiveDateTime,
    pub ip: String,
    pub session_id: String,
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
#[table_name = "user_login"]
pub struct user_login_without_id {
    pub user_id: i32,
    pub login_time: NaiveDateTime,
    pub logout_time: NaiveDateTime,
    pub ip: String,
    pub session_id: String,
}

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct users_with_id {
    pub id: i32,
    pub username: String,
    pub token: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct users_without_id {
    pub username: String,
    pub token: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct visits_with_id {
    pub id: i32,
    pub student_id: i32,
    pub checked_in: NaiveDateTime,
    pub left_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
#[table_name = "visits"]
pub struct visits_without_id {
    pub student_id: i32,
    pub checked_in: NaiveDateTime,
    pub left_at: Option<NaiveDateTime>,
}

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct visits_with_student {
    pub id: i32,
    pub student: Option<students_with_id>,
    pub student_id: i32,
    pub checked_in: NaiveDateTime,
    pub left_at: Option<NaiveDateTime>,
}
impl From<&visits_with_id> for visits_with_student {
    fn from(v: &visits_with_id) -> Self {
        let mut default = visits_with_student {
            id: v.id,
            student: None,
            student_id: v.student_id,
            checked_in: v.checked_in,
            left_at: v.left_at,
        };

        let connection = match crate::create_connection() {
            Some(c) => c,
            None => return default,
        };

        let student: students_with_id = match super::schema::students::dsl::students
            .filter(super::schema::students::dsl::id.eq(v.student_id))
            .first::<students_with_id>(&connection)
        {
            Ok(s) => s,
            Err(e) if e == diesel::NotFound => {
                // log_warn(format!(
                //     "Failed to find student with id {} while adding student to visit struct!",
                //     v.student_id
                // ));
                warn!(
                    "Failed to find student with id {} while adding student to visit struct!",
                    v.student_id
                );
                return default;
            }
            Err(e) => {
                log_warn(format!("Unknown error occurred while adding student with id {} to a visit struct (error {})", v.student_id, e));
                return default;
            }
        };

        default.student = Some(student);
        default
    }
}
