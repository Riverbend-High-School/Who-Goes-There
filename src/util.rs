
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

pub fn log_info<T: std::fmt::Display>(m : T) {
    info!("{}", m);
    sentry::capture_message(m.to_string().as_str(), sentry::Level::Debug);
}

pub fn log_warn<T: std::fmt::Display>(m : T) {
    warn!("{}", m);
    sentry::capture_message(m.to_string().as_str(), sentry::Level::Warning);
}