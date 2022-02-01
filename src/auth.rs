extern crate diesel;

use crate::model::*;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content::Json;
use serde_json::json;

use self::diesel::prelude::*;

#[derive(Clone, PartialEq)]
pub struct api_key(pub String);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for api_key {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key = req.query_value::<&str>("token");

        match key {
            Some(r) => match r {
                Ok(s) if validate_token(s.to_owned()).await => {
                    request::Outcome::Success(api_key(s.to_owned()))
                },
                _ => request::Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
            },
            None => {
                let cookies = req.cookies();
                match cookies.get("token") {
                    Some(s) => if validate_token(s.to_string()).await {
                        request::Outcome::Success(api_key(s.to_string()))
                    } else {
                        request::Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                    },
                    _ => {
                        request::Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
                    }
                }
            },
        }
    }
}

pub async fn validate_token(token: String) -> bool {
    get_user(token).await.is_some()
}

pub async fn get_user(token: String) -> Option<users_with_id> {
    let connection = match crate::create_connection() {
        Some(c) => c,
        None => return None,
    };

    match super::schema::users::dsl::users
        .filter(super::schema::users::dsl::token.eq(token.clone()))
        .first::<users_with_id>(&connection)
    {
        Ok(u) => Some(u),
        Err(e) => {
            warn!(
                "User with token {} failed to validate! (error {})",
                token, e
            );
            None
        }
    }
}

// http://0.0.0.0/login?token=someSecretKeyHere
#[get("/login")]
pub async fn login(_key : api_key) -> Status {
    Status::Ok
}

#[get("/me")]
pub async fn me(key: api_key) -> rocket::response::content::Json<String> {
    match get_user(key.0).await {
        Some(u) => Json(
                json!({
                    "status": 200,
                    "message": "Found",
                    "data": u
                }).to_string()
            ),
        None => Json(
                json!({
                    "status": 500,
                    "message": "Internal Server Error",
                }).to_string()
            ),
    }   
}
