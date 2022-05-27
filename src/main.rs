#![allow(non_camel_case_types)]

#[macro_use]
extern crate rocket;

extern crate openssl;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel_migrations;

embed_migrations!("migrations");

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
    openssl_probe::init_ssl_cert_env_vars();

    let connection = create_connection().expect("Failed to connect to database");

    embedded_migrations::run(&connection).expect("Failed to run embedded migrations");

    std::mem::drop(connection);

    let _guard = match env::var("SENTRY_DSN") {
        Ok(s) => Some(sentry::init((
            s,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ))),
        Err(e) => {
            println!(
                "Failed to get SENTRY_DSN with error {}. Continuing without Sentry logging.",
                e
            );
            None
        }
    };

    // Starting rocket
    match rocket::build()
        .mount("/", routes![index, files,])
        .mount(
            "/api",
            routes![
                auth::login,
                auth::me,
                student::check_in,
                student::check_out,
                student::visits,
                student::all_visits,
                student::get_student,
                student::get_all_student,
                student::public_visits,
                student::integrate_student_csv,
                student::checkout_student_csv,
                student::get_student_stats
            ],
        )
        .attach(crate::util::CORS)
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
    Some(unwrap_or_return!(
        PgConnection::establish(&database_url),
        "Error connecting to database!"
    ))
}
