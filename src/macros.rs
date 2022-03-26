pub const ERROR_LOG: &str = "Can't log to stderr";

#[macro_export]
macro_rules! core_log {
    ($color:tt, $prefix:tt, $($args:tt)+) => {
        plog::log(crossterm::style::Color::$color, $prefix, format!($($args)+))
            .expect(plog::macros::ERROR_LOG)
    }
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)+) => {
        plog::core_log!(Grey, "DEBG", $($args)+)
    }
}

#[macro_export]
macro_rules! info {
    ($($args:tt)+) => {
        plog::core_log!(White, "INFO", $($args)+)
    }
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)+) => {
        plog::core_log!(Yellow, "WARN", $($args)+)
    }
}

#[macro_export]
macro_rules! error {
    ($($args:tt)+) => {
        plog::core_log!(Red, "ERRO", $($args)+)
    }
}

#[macro_export]
macro_rules! ok {
    ($($args:tt)+) => {
        plog::core_log!(Green, "OKAY", $($args)+)
    }
}
