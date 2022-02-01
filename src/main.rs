#![allow(non_camel_case_types)]


#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

pub mod auth;
pub mod model;
pub mod schema;
pub mod student;
pub mod util;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::fs::NamedFile;
use std::{
    env,
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
        .mount("/", routes![
            index, 
            files, 
        ])
        .mount("/api", routes![
            auth::login,
            auth::me,
            student::check_in,
            student::check_out,
            student::visits,
            student::all_visits,
            student::get_student,
        ])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("Server stopped unexpectedly... (error {})", e);
        }
    };
}

fn create_connection() -> Option<PgConnection> {
    let database_url = unwrap_or_return!(env::var("DATABASE_URL"), "Database URL not set.");
    Some(
        unwrap_or_return!(
            PgConnection::establish(&database_url),
            "Error connecting to database!"
        )
    )
}
