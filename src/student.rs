extern crate diesel;

use crate::auth::api_key;
use crate::model::*;
use regex::Regex;
use rocket::response::content::Json;
use rocket::serde::{self, Deserialize};
use serde_json::json;

use crate::util::*;

use self::diesel::prelude::*;
use diesel::dsl;

lazy_static! {
    static ref EMAIL_PREFIX_REGEX: Regex =
        Regex::new(r"^(?P<username>[\w]+[-]{1}\d{2})(?P<email>@spotsylvania.k12.va.us)?$").unwrap();
    static ref SEVEN_ID_PREFIX_REGEX: Regex = Regex::new(r"^\d{7}$").unwrap();
}

#[derive(Deserialize)]
pub struct student_post {
    pub student_response: String,
}

#[post("/checkin", format = "application/json", data = "<student>")]
pub async fn check_in(token: api_key, student: serde::json::Json<student_post>) -> Json<String> {
    let token = token.0;

    info!(
        "Token {} beginning to check in student {}",
        token, student.student_response
    );

    let student_response = &student.student_response;

    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return Json(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            )
        }
    };

    // If student provided their email prefix/email
    let student = if let Some(cap) = EMAIL_PREFIX_REGEX.captures(student_response) {
        let has_email = cap.name("email").is_some();

        let mut query = student_response.clone();

        if !has_email {
            query += "@spotsylvania.k12.va.us";
        }

        match super::schema::students::dsl::students
            .filter(super::schema::students::dsl::email.eq(query.clone()))
            .first::<students_with_id>(&connection)
        {
            Ok(u) => u,
            Err(e) => {
                log_warn(format!(
                    "Student with email '{}' does not exist! (error {})",
                    query, e
                ));
                return Json(
                    json!({
                        "status": 404,
                        "message": "Email not found in database!"
                    })
                    .to_string(),
                );
            }
        }
    } else if let Some(_) = SEVEN_ID_PREFIX_REGEX.captures(student_response) {
        let query = student_response.clone();

        match super::schema::students::dsl::students
            .filter(super::schema::students::dsl::seven_id.eq(query.clone()))
            .first::<students_with_id>(&connection)
        {
            Ok(u) => u,
            Err(e) => {
                log_warn(format!(
                    "Student with seven digit id '{}' does not exist! (error {})",
                    query, e
                ));
                return Json(
                    json!({
                        "status": 404,
                        "message": "7 digit ID not found in database!"
                    })
                    .to_string(),
                );
            }
        }
    } else {
        return Json(
            json!({
                "status": 400,
                "message": "Invalid input. Please input your seven digit pin or your email!"
            })
            .to_string(),
        );
    };

    // TODO: Implement if student forgot to checkout previous visit,
    //       have them indicate duration of previous stay
    match super::schema::visits::dsl::visits
        .filter(super::schema::visits::student_id.eq(student.id))
        .filter(super::schema::visits::dsl::left_at.is_null())
        .first::<visits_with_id>(&connection)
    {
        Ok(v) => {
            log_info(format!("Student {} had a previous check in! Setting their leave time to 2 hours from when they last checked in.", student.seven_id));
            match diesel::update(
                super::schema::visits::dsl::visits.filter(super::schema::visits::id.eq(v.id)),
            )
            .set(super::schema::visits::left_at.eq(v.checked_in + chrono::Duration::hours(2)))
            .execute(&connection)
            {
                Ok(_) => (),
                Err(e) => {
                    log_warn(format!(
                        "Failed to update visit {} with new left_at with error {}",
                        v.id, e
                    ));
                }
            }
        }
        Err(_) => (),
    }

    match diesel::insert_into(super::schema::visits::dsl::visits)
        .values(super::schema::visits::dsl::student_id.eq(student.id))
        .execute(&connection)
    {
        Ok(_) => (),
        Err(e) => {
            log_warn(format!(
                "Failed to insert new visit into visits table for student {} ({}). (error {})",
                student.seven_id, student.email, e
            ));
            return Json(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            );
        }
    }

    Json(
        json!({
            "status": 204,
            "message": "Successfully checked in!"
        })
        .to_string(),
    )
}

#[post("/checkout", format = "application/json", data = "<student>")]
pub async fn check_out(token: api_key, student: serde::json::Json<student_post>) -> Json<String> {
    let token = token.0;

    info!(
        "Token {} beginning to check out student {}",
        token, student.student_response
    );

    let student_response = &student.student_response;

    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return Json(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            )
        }
    };

    // If student provided their email prefix/email
    let student = if let Some(cap) = EMAIL_PREFIX_REGEX.captures(student_response) {
        let has_email = cap.name("email").is_some();

        let mut query = student_response.clone();

        if !has_email {
            query += "@spotsylvania.k12.va.us";
        }

        match super::schema::students::dsl::students
            .filter(super::schema::students::dsl::email.eq(query.clone()))
            .first::<students_with_id>(&connection)
        {
            Ok(u) => u,
            Err(e) => {
                log_warn(format!(
                    "Student with email '{}' does not exist! (error {})",
                    query, e
                ));
                return Json(
                    json!({
                        "status": 404,
                        "message": "Email not found in database!"
                    })
                    .to_string(),
                );
            }
        }
    } else if let Some(_) = SEVEN_ID_PREFIX_REGEX.captures(student_response) {
        let query = student_response.clone();

        match super::schema::students::dsl::students
            .filter(super::schema::students::dsl::seven_id.eq(query.clone()))
            .first::<students_with_id>(&connection)
        {
            Ok(u) => u,
            Err(e) => {
                log_warn(format!(
                    "Student with seven digit id '{}' does not exist! (error {})",
                    query, e
                ));
                return Json(
                    json!({
                        "status": 404,
                        "message": "7 digit ID not found in database!"
                    })
                    .to_string(),
                );
            }
        }
    } else {
        return Json(
            json!({
                "status": 400,
                "message": "Invalid input"
            })
            .to_string(),
        );
    };

    match super::schema::visits::dsl::visits
        .filter(super::schema::visits::student_id.eq(student.id))
        .filter(super::schema::visits::dsl::left_at.is_null())
        .first::<visits_with_id>(&connection)
    {
        Ok(v) => {
            match diesel::update(
                super::schema::visits::dsl::visits.filter(super::schema::visits::id.eq(v.id)),
            )
            .set(super::schema::visits::left_at.eq(dsl::now))
            .execute(&connection)
            {
                Ok(_) => (),
                Err(e) => {
                    log_warn(format!(
                        "Failed to update visit {} with new left_at with error {}",
                        v.id, e
                    ));
                }
            }
        },
        Err(e) if e == diesel::NotFound => {
            log_info(format!(
                "No active visit for student '{}' ({})", student.seven_id, student.email, 
            ));
            return Json(
                json!({
                    "status": 404,
                    "message": "Student has not checked in previously. Please remember to check in when coming into the library!"
                })
                .to_string(),
            )
        }
        Err(e) => {
            log_warn(format!(
                "Error occured while finding active visit for student with id '{}' (error {})", student.id, e, 
            ));
            return Json(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            )
        },
    }
    
    Json(
        json!({
            "status": 204,
            "message": "Visit Checked Out"
        })
        .to_string(),
    )
}

#[get("/visits/all")]
pub async fn all_visits(_token: api_key) -> Json<String> {
   
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return Json(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            )
        }
    };

    let visits : Vec<visits_with_id> = match super::schema::visits::dsl::visits
        .get_results::<visits_with_id>(&connection) {
            Ok(v) => {
                v
            },
            Err(e) => {
                log_warn(format!(
                    "Could not get active visits with error {}", e
                ));
                return Json(
                    json!({
                        "status": 500,
                        "message": "Internal Server Error"
                    })
                    .to_string(),
                )
            }
        };
   
    Json(
        json!({
            "status": 200,
            "message": "Found",
            "data": visits
        })
        .to_string(),
    )
}

#[get("/visits")]
pub async fn visits(_token: api_key) -> Json<String> {
   
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return Json(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            )
        }
    };

    let visits : Vec<visits_with_id> = match super::schema::visits::dsl::visits
        .filter(super::schema::visits::dsl::left_at.is_null())
        .get_results::<visits_with_id>(&connection) {
            Ok(v) => {
                v
            },
            Err(e) => {
                log_warn(format!(
                    "Could not get active visits with error {}", e
                ));
                return Json(
                    json!({
                        "status": 500,
                        "message": "Internal Server Error"
                    })
                    .to_string(),
                )
            }
        };
   
    Json(
        json!({
            "status": 200,
            "message": "Found",
            "data": visits
        })
        .to_string(),
    )
}

#[get("/student/<id>")]
pub async fn get_student(id : i32, _token : api_key) -> Json<String> {


    Json(
        json!({
            "status": 200,
            "message": "Found",
            "data": students_with_id::default()
        }).to_string()
    )
}
