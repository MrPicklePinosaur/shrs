/// Logs using [log::error!] is variable was [Err] variant
#[macro_export]
macro_rules! log_if_err {
    ($result:expr) => {{
        if let Err(e) = $result {
            log::error!("{}", e);
        }
    }};
    ($result:expr, $fmt:expr) => {{
        if let Err(e) = $result {
            log::error!(concat!($fmt, ": {}"), e);
        }
    }};
    ($result:expr, $fmt:expr, $($arg:tt)*) => {{
        if let Err(e) = $result {
            log::error!(concat!($fmt, ": {}"), $($arg)*, e);
        }
    }};
}

/// Logs using [log::warn!] is variable was [Err] variant
#[macro_export]
macro_rules! warn_if_err {
    ($result:expr) => {{
        if let Err(e) = $result {
            log::warn!("{}", e);
        }
    }};
    ($result:expr, $fmt:expr) => {{
        if let Err(e) = $result {
            log::warn!(concat!($fmt, ": {}"), e);
        }
    }};
    ($result:expr, $fmt:expr, $($arg:tt)*) => {{
        if let Err(e) = $result {
            log::warn!(concat!($fmt, ": {}"), $($arg)*, e);
        }
    }};
}
