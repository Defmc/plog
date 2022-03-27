pub const ERROR_LOG: &str = "Can't log to stderr";

#[allow(clippy::ptr_arg)]
// Impossible to use `&mut str`.
// Function `String::push_str` is needed when `datetime` feature is enabled
pub fn datetime(_input: &mut String) {
    #[cfg(any(feature = "date", feature = "time"))]
    {
        use chrono::Local;

        let formatter = match (cfg!(feature = "date"), cfg!(feature = "time")) {
            (true, true) => " on %Y-%m-%d %H:%M:%S",
            (true, false) => " on %Y-%m-%d",
            (false, true) => " on %H:%M:%S",
            (_, _) => "",
        };

        let datetime = Local::now().format(formatter);
        _input.push_str(&datetime.to_string());
    }
}

#[cfg(feature = "context")]
#[macro_export]
macro_rules! context {
    ($input:tt) => {
        $input.push_str(&format!(" at {}:{}", file!(), line!()))
    }
}

#[cfg(not(feature = "context"))]
#[macro_export]
macro_rules! context {
    ($input:tt) => {
    }
}

#[macro_export]
macro_rules! log {
    ($color:tt, $prefix:tt, $($args:tt)+) => {{
        let mut prefix = String::from($prefix);
        plog::macros::datetime(&mut prefix);
        plog::context!(prefix);
        plog::core_log!($color, { prefix.as_ref() }, $($args)+).expect(plog::macros::ERROR_LOG)
    }}
}

#[cfg(feature = "colored")]
#[macro_export]
macro_rules! core_log {
    ($color:tt, $prefix:expr, $($args:tt)+) => {
        plog::log(crossterm::style::Color::$color, $prefix, format!($($args)+))
    }
}

#[cfg(not(feature = "colored"))]
#[macro_export]
macro_rules! core_log {
    ($color:tt, $prefix:tt, $($args:tt)+) => {
        plog::log($prefix, format!($($args)+))
    }
}



#[macro_export]
macro_rules! debug {
    ($($args:tt)+) => {{
        plog::log!(Grey, "DEBG", $($args)+)
    }}
}

#[macro_export]
macro_rules! info {
    ($($args:tt)+) => {{
        plog::log!(White, "INFO", $($args)+)
    }}
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)+) => {{
        plog::log!(Yellow, "WARN", $($args)+)
    }}
}

#[macro_export]
macro_rules! error {
    ($($args:tt)+) => {{
        plog::log!(Red, "ERRO", $($args)+)
    }}
}

#[macro_export]
macro_rules! ok {
    ($($args:tt)+) => {
        plog::log!(Green, "OKAY", $($args)+)
    }
}
