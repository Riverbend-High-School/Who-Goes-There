extern crate diesel;

use crate::auth::api_key;
use crate::{make_json_response, model::*, unwrap_or_return};
use regex::Regex;
use rocket::response::content::RawJson;
use rocket::serde::{self, Deserialize};
use rocket::tokio::io::AsyncReadExt;
use serde_json::json;

use crate::util::*;

use self::diesel::prelude::*;
use diesel::dsl;

use rocket::data::{Data, ToByteUnit};

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
pub async fn check_in(token: api_key, student: serde::json::Json<student_post>) -> RawJson<String> {
    let token = token.0;

    info!(
        "Token {} beginning to check in student {}",
        token, student.student_response
    );

    let student_response = &student.student_response;

    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return RawJson(
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
                // log_warn(format!(
                //     "Student with email '{}' does not exist! (error {})",
                //     query, e
                // ));
                warn!(
                    "Student with email '{}' does not exist! (error {})",
                    query, e
                );
                return RawJson(
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
                // log_warn(format!(
                //     "Student with seven digit id '{}' does not exist! (error {})",
                //     query, e
                // ));
                warn!(
                    "Student with seven digit id '{}' does not exist! (error {})",
                    query, e
                );
                return RawJson(
                    json!({
                        "status": 404,
                        "message": "7 digit ID not found in database!"
                    })
                    .to_string(),
                );
            }
        }
    } else {
        return RawJson(
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
            info!("Student {} had a previous check in! Setting their leave time to 2 hours from when they last checked in.", student.seven_id);
            match diesel::update(
                super::schema::visits::dsl::visits.filter(super::schema::visits::id.eq(v.id)),
            )
            .set(super::schema::visits::left_at.eq(v.checked_in + chrono::Duration::hours(2)))
            .execute(&connection)
            {
                Ok(_) => (),
                Err(e) => {
                    // log_warn(format!(
                    //     "Failed to update visit {} with new left_at with error {}",
                    //     v.id, e
                    // ));
                    warn!(
                        "Failed to update visit {} with new left_at with error {}",
                        v.id, e
                    );
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
            return RawJson(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            );
        }
    }

    RawJson(
        json!({
            "status": 204,
            "message": "Successfully checked in!"
        })
        .to_string(),
    )
}

#[post("/checkout", format = "application/json", data = "<student>")]
pub async fn check_out(
    token: api_key,
    student: serde::json::Json<student_post>,
) -> RawJson<String> {
    let token = token.0;

    info!(
        "Token {} beginning to check out student {}",
        token, student.student_response
    );

    let student_response = &student.student_response;

    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return RawJson(
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
                // log_warn(format!(
                //     "Student with email '{}' does not exist! (error {})",
                //     query, e
                // ));
                warn!(
                    "Student with email '{}' does not exist! (error {})",
                    query, e
                );
                return RawJson(
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
                // log_warn(format!(
                //     "Student with seven digit id '{}' does not exist! (error {})",
                //     query, e
                // ));
                warn!(
                    "Student with seven digit id '{}' does not exist! (error {})",
                    query, e
                );
                return RawJson(
                    json!({
                        "status": 404,
                        "message": "7 digit ID not found in database!"
                    })
                    .to_string(),
                );
            }
        }
    } else {
        return RawJson(
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
        }
        Err(e) if e == diesel::NotFound => {
            info!(
                "No active visit for student '{}' ({})",
                student.seven_id, student.email,
            );
            return RawJson(
                json!({
                    "status": 404,
                    "message": "Student has not checked in previously. Please remember to check in when coming into the library!"
                })
                .to_string(),
            );
        }
        Err(e) => {
            log_warn(format!(
                "Error occured while finding active visit for student with id '{}' (error {})",
                student.id, e,
            ));
            return RawJson(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            );
        }
    }

    RawJson(
        json!({
            "status": 204,
            "message": "Visit Checked Out"
        })
        .to_string(),
    )
}

#[get("/visits/all")]
pub async fn all_visits(_token: api_key) -> RawJson<String> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return RawJson(
                json!({
                    "status": 500,
                    "message": "Internal Server Error"
                })
                .to_string(),
            )
        }
    };

    let visits: Vec<visits_with_id> =
        match super::schema::visits::dsl::visits.get_results::<visits_with_id>(&connection) {
            Ok(v) => v,
            Err(e) => {
                log_warn(format!("Could not get active visits with error {}", e));
                return RawJson(
                    json!({
                        "status": 500,
                        "message": "Internal Server Error"
                    })
                    .to_string(),
                );
            }
        };

    RawJson(
        json!({
            "status": 200,
            "message": "Found",
            "data": visits
        })
        .to_string(),
    )
}

#[get("/visits")]
pub async fn visits(_token: api_key) -> RawJson<String> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => return make_json_response!(500, "Internal Server Error"),
    };

    let visits: Vec<visits_with_student> = match super::schema::visits::dsl::visits
        .filter(super::schema::visits::dsl::left_at.is_null())
        .get_results::<visits_with_id>(&connection)
    {
        Ok(v) => v,
        Err(e) => {
            log_warn(format!("Could not get active visits with error {}", e));
            return make_json_response!(500, "Internal Server Error");
        }
    }
    .iter()
    .map(|v| visits_with_student::from(v))
    .collect();

    return make_json_response!(200, "Found", visits);
}

#[get("/visits/public")]
pub async fn public_visits() -> RawJson<String> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => return make_json_response!(500, "Internal Server Error"),
    };

    let visits: Vec<serde_json::Value> = match super::schema::visits::dsl::visits
        .order(super::schema::visits::dsl::left_at.desc())
        .limit(100)
        .get_results::<visits_with_id>(&connection)
    {
        Ok(v) => v,
        Err(e) => {
            log_warn(format!("Could not get visits with error {}", e));
            return make_json_response!(500, "Internal Server Error");
        }
    }
    .iter()
    .map(|v| {
        let visit = visits_with_student::from(v);
        // info!("Converting visit to json: {:?}", visit);
        let student = visit.student.unwrap_or_default();
        json!({
            "id": visit.id,
            "student_name": student.student_name,
            "checked_in": visit.checked_in,
            "left_at": visit.left_at,
            "is_aide": student.is_aide,
        })
    })
    .collect();

    make_json_response!(200, "Found", visits)
}

#[get("/visits/csv")]
pub async fn checkout_student_csv(_token: api_key) -> Option<String> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return None;
        }
    };

    let buffer = Vec::new();

    let visits: Vec<visits_with_id> =
        match super::schema::visits::dsl::visits.get_results::<visits_with_id>(&connection) {
            Ok(v) => v,
            Err(e) if e == diesel::NotFound => {
                // log_warn(format!(
                //     "Could not find student with id {} (error {})",
                //     id, e
                // ));
                warn!("Failed to get any student from the database! (error {})", e);
                return None;
            }
            Err(e) => {
                log_warn(format!("Unknown error '{}' while selecting all visits", e));
                return None;
            }
        };
    let mut writer = csv::Writer::from_writer(buffer);

    unwrap_or_return!(
        writer.write_record([
            "id",
            "student_name",
            "seven_id",
            "email",
            "checked_in",
            "left_at",
        ]),
        "Attempted to write header to buffer in checkout_student_csv but failed!"
    );
    visits
        .iter()
        .map(|v| visits_with_student::from(v))
        .for_each(|v| {
            let student = v.student.unwrap_or_default();
            let row = vec![
                v.id.to_string(),
                student.student_name,
                student.seven_id,
                student.email,
                v.checked_in.to_string(),
                match v.left_at {
                    Some(c) => c.to_string(),
                    None => String::default(),
                },
            ];
            writer.write_record(row).unwrap();
        });
    unwrap_or_return!(writer.flush(), "Attempted to flush writer but failed!");
    Some(unwrap_or_return!(
        String::from_utf8(unwrap_or_return!(
            writer.into_inner(),
            "Attempted to get writer inner but failed!"
        )),
        "Failed to open CSV after generating it!"
    ))
}

#[get("/student/<id>")]
pub async fn get_student(id: i32, _token: api_key) -> RawJson<String> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return make_json_response!(500, "Internal Server Error");
        }
    };

    let student: students_with_id = match super::schema::students::dsl::students
        .filter(super::schema::students::dsl::id.eq(id))
        .get_result::<students_with_id>(&connection)
    {
        Ok(s) => s,
        Err(e) if e == diesel::NotFound => {
            // log_warn(format!(
            //     "Could not find student with id {} (error {})",
            //     id, e
            // ));
            warn!("Could not find student with id {} (error {})", id, e);
            return make_json_response!(400, "Student not found!");
        }
        Err(e) => {
            log_warn(format!(
                "Unknown error '{}' while selecting student with id {}",
                e, id
            ));
            return make_json_response!(500, "Internal Server Error");
        }
    };

    make_json_response!(200, "Found", student)
}

#[get("/student/all")]
pub async fn get_all_student(_token: api_key) -> RawJson<String> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return make_json_response!(500, "Internal Server Error");
        }
    };

    let students: Vec<students_with_id> =
        match super::schema::students::dsl::students.get_results::<students_with_id>(&connection) {
            Ok(v) => v,
            Err(e) if e == diesel::NotFound => {
                // log_warn(format!(
                //     "Could not find student with id {} (error {})",
                //     id, e
                // ));
                warn!("Failed to get any student from the database! (error {})", e);
                return make_json_response!(400, "Students not found!");
            }
            Err(e) => {
                log_warn(format!(
                    "Unknown error '{}' while selecting all students",
                    e
                ));
                return make_json_response!(500, "Internal Server Error");
            }
        };

    make_json_response!(200, "Found", students)
}

#[post("/student/integrate", data = "<file>")]
pub async fn integrate_student_csv(_token: api_key, file: Data<'_>) -> RawJson<String> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => {
            return make_json_response!(500, "Internal Server Error");
        }
    };

    let mut buffer = String::new();
    match file
        .open(32u32.megabytes())
        .read_to_string(&mut buffer)
        .await
    {
        Ok(_) => (),
        Err(e) => {
            log_warn(format!(
                "Failed to read CSV info into a string with error {}",
                e
            ));
            return make_json_response!(500, "Internal Server Error");
        }
    };

    let mut rdr = csv::Reader::from_reader(buffer.as_bytes());

    let mut errors: Vec<(usize, String)> = vec![]; // Line number, seven digit ID

    let insertable_students: Vec<students_without_id> = rdr
        .records()
        .enumerate()
        .filter_map(|(i, result)| match result {
            Ok(result) => {
                let name = match result.get(0) {
                    Some(e) => e,
                    None => {
                        warn!("CSV is missing the name field for line {}.", i);
                        errors.push((i, String::from("Unknown")));
                        return None;
                    }
                };
                let seven_id = match result.get(1) {
                    Some(e) => e,
                    None => {
                        warn!("CSV is missing the seven_id field for line {}.", i);
                        errors.push((i, name.to_owned()));
                        return None;
                    }
                };
                let email = match result.get(2) {
                    Some(e) => e,
                    None => {
                        warn!("CSV is missing the email field for line {}.", i);
                        errors.push((i, seven_id.to_owned()));
                        return None;
                    }
                };
                let is_aide = match result.get(3) {
                    Some(s) => match s.to_lowercase().parse::<bool>() {
                        Ok(a) => a,
                        Err(e) => {
                            warn!(
                                "Failed to parse {} as boolean! (error {} for {})",
                                s, e, seven_id
                            );
                            errors.push((i, seven_id.to_owned()));
                            return None;
                        }
                    },
                    None => {
                        warn!("CSV is missing the is_aide field for line {}.", i);
                        errors.push((i, seven_id.to_owned()));
                        return None;
                    }
                };

                let now = chrono::offset::Local::now().naive_utc();

                Some(students_without_id {
                    seven_id: seven_id.to_owned(),
                    student_name: name.to_owned(),
                    email: email.to_owned(),
                    created_at: now,
                    updated_at: now,
                    is_aide: is_aide,
                })
            }
            Err(e) => {
                warn!(
                    "There was an error while getting csv information! (error {})",
                    e
                );
                errors.push((i, String::from("Unknown")));
                None
            }
        })
        .collect();

    let affected = match diesel::insert_into(super::schema::students::dsl::students)
        .values(&insertable_students)
        .on_conflict_do_nothing()
        .execute(&connection)
    {
        Ok(a) => a,
        Err(e) => {
            warn!("Failed to insert new students into database! (error {}", e);
            return make_json_response!(500, "Internal Server Error");
        }
    };

    if errors.len() > 0 || affected == 0 {
        if insertable_students.len() == 0 || affected == 0 {
            make_json_response!(400, "Bad Request", errors)
        } else {
            make_json_response!(202, "Partial Success", errors)
        }
    } else {
        make_json_response!(200, "Ok")
    }
}
