pub const ERROR_LOG: &str = "Can't log to stderr";

#[allow(clippy::ptr_arg)]
// Impossible to use `&mut str`.
// Function `String::push_str` is needed when `datetime` feature is enabled
pub fn datetime(_input: &mut String) {
    #[cfg(feature = "datetime")]
    {
        use chrono::Local;
        let datetime = Local::now().format(" %Y-%m-%d %H:%M:%S");
        _input.push_str(&datetime.to_string());
    }
}

#[macro_export]
macro_rules! context {
    ($input:tt) => {
        $input.push_str(&format!(" at {}:{}", file!(), line!()))
    };
}

#[macro_export]
macro_rules! core_log {
    ($color:tt, $prefix:tt, $($args:tt)+) => {
        let mut prefix = $prefix.into();
        plog::macros::datetime(&mut prefix);
        plog::context!(prefix);
        plog::log(crossterm::style::Color::$color, &prefix, format!($($args)+))
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
