
#[macro_export]
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