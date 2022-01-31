#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod model;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::{
    fs::NamedFile,
    futures::lock::Mutex,
    http::{ContentType, Cookie, CookieJar, Method, Status},
    response,
    serde::{Deserialize, Serialize},
};
use std::{
    env,
    ops::Deref,
    path::{Path, PathBuf},
};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    // Starting rocket
    match rocket::build()
        .mount("/", routes![index, files,])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("Server stopped unexpectedly... (error {})", e);
        }
    };
}

macro_rules! unwrap_or_return {
    ($r:expr, $s:expr) => {
        match $r {
            Ok(r) => r,
            Err(e) => {
                warn!("Unwrapped on error {} (error {})", e, $s);
                return None;
            }
        }
    };
    ($o:expr, $s:expr) => {
        match $o {
            Some(r) => r,
            None => {
                warn!("Unwrapped on None (error {})", $s);
                return None;
            }
        }
    };
}

// TODO: Exit gracefully
fn create_connection() -> Option<PgConnection> {
    let database_url = unwrap_or_return!(env::var("DATABASE_URL"), "Database URL not set.");
    Some(unwrap_or_return!(
        PgConnection::establish(&database_url),
        "Error connecting to database!"
    ))
}
