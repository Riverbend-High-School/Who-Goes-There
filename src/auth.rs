extern crate diesel;

use crate::model::*;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{self, FromRequest, Request};
use rocket::response::{Flash, Redirect};
use rocket::response::content::Json;

use self::diesel::prelude::*;

#[derive(Clone, PartialEq)]
struct api_key(String);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for api_key {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("x-api-key").collect();
        match keys.len() {
            0 => request::Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if validate_token(keys[0].to_owned()) => {
                request::Outcome::Success(api_key(keys[0].to_string()))
            }
            _ => request::Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

// http://0.0.0.0/login?token=someSecretKeyHere

pub async fn validate_token(token: String) -> bool {
    match get_user(token).await {
        Some(b) if b => true,
        _ => false,
    }
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

#[get("/me")]
pub async fn me(key: api_key) -> rocket::response::content::Json<users_with_id> {
    match get_user(key.0).await {
        Some(u) => Json(u),
        None => Json::default(),
    }
}
