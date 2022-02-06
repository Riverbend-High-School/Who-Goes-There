#[macro_export]
macro_rules! unwrap_or_return {
    ($r:expr, $s:expr) => {
        match $r {
            Ok(r) => r,
            Err(e) => {
                crate::util::log_warn(format!("Unwrapped on error {} (error {})", e, $s));
                return None;
            }
        }
    };
    ($o:expr, $s:expr) => {
        match $o {
            Some(r) => r,
            None => {
                crate::util::log_warn(format!("Unwrapped on None (error {})", $s));
                return None;
            }
        }
    };
}

pub fn log_info<T: std::fmt::Display>(m: T) {
    info!("{}", m);
    sentry::capture_message(m.to_string().as_str(), sentry::Level::Debug);
}

pub fn log_warn<T: std::fmt::Display>(m: T) {
    warn!("{}", m);
    sentry::capture_message(m.to_string().as_str(), sentry::Level::Warning);
}

/// A macro to write the standard output for our responses.
/// # Arguments
///
/// * status : i32 - A status code indicating the response type
/// * message : String - A string containing the message for the response
/// * data : <T> - An optional parameter, if the response is sending back data, it will be serialized by Serde here
///
/// # Example
///
/// ```
/// match Some(()) {
///     Some(_) => make_json_response!(200, "Found", {"Foo": "Bar"}),
///     None => make_json_response!(404, "Not Found"),
/// }
/// ```
#[macro_export]
macro_rules! make_json_response {
    ($status:expr, $message:expr, $data:expr) => {
        Json(json!({
            "status": $status as i32,
            "message": $message,
            "data": $data,
        }).to_string())
    };
    ($status:expr, $message:expr) => {
        Json(json!({
            "status": $status as i32,
            "message": $message,
        }).to_string())
    }
}

/*

{
    "status": 200,
    "message": "Found",
    "data": {foo: "Bar"}
}

*/

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
