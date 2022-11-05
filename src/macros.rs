//! Core of the library.
//! Contains the front-end to `plog::log`, formating arguments
//! and applying features defined at compile-time

/// The default `expect` message for `plog::log!`
pub const ERROR_LOG: &str = "Can't log to stderr";

#[cfg(feature = "colored")]
pub mod colors {
    use crossterm::style::Color;
    pub const GREEN: Color = Color::Green;
    pub const WHITE: Color = Color::White;
    pub const RED: Color = Color::Red;
    pub const YELLOW: Color = Color::Yellow;
    pub const GREY: Color = Color::Grey;
}

/// `local_date` is formated by year, month and day
/// `local_time` is formated by hour, minute and second
#[allow(clippy::ptr_arg)]
// Impossible to use `&mut str`.
// Function `String::push_str` is needed when `datetime` feature is enabled
pub fn datetime(_input: &mut String) {
    #[cfg(any(feature = "local_date", feature = "local_time"))]
    {
        use time::OffsetDateTime;
        if let Ok(now) = OffsetDateTime::now_local() {
            if cfg!(feature = "local_date") {
                let date = format!(" on {}/{}/{}", now.year(), now.month(), now.day());
                _input.push_str(&date);
            }
            if cfg!(feature = "local_time") {
                let time = format!(" at {}:{}:{}", now.hour(), now.minute(), now.second());
                _input.push_str(&time);
            }
        }
    }
}

/// Context formatter, includes the line and file where the macro was called
/// Just enabled with `context` feature
/// ```rust
/// let mut log = String::from("hi");
/// plog::context!(log);
/// assert_eq!(log, "hi in src/macros.rs:5");
/// ```
#[cfg(feature = "context")]
#[macro_export]
macro_rules! context {
    ($input:tt) => {
        $input.push_str(&format!(" in {}:{}", file!(), line!()))
    };
}

#[cfg(not(feature = "context"))]
#[macro_export]
macro_rules! context {
    ($input:tt) => {};
}

/// Log formatter, apply every formating feature enabled:
/// ```rust
/// plog::log!(WHITE, "....", "I'm a four-dots complement");
/// ```
/// # Panic
/// When it's impossible to write on STDERR.
/// Write to a file also can panic, but it just can happen with `persistent` feature enabled
#[macro_export]
macro_rules! log {
    ($color:tt, $prefix:tt, $($args:tt)+) => {{
        let mut prefix = String::from($prefix);
        plog::macros::datetime(&mut prefix);
        plog::context!(prefix);
        plog::core_log!($color, { prefix.as_ref() }, $($args)+).expect(plog::macros::ERROR_LOG)
    }}
}

/// Log caller, calls the `log` function and format string using the `std::format` macro
/// Just enable colored terminal output with `colored` feature
/// ```rust
/// plog::core_log!(WHITE, ".... at src/main.rs:2", "I'm a four-dots complement").unwrap()
/// ```
#[cfg(feature = "colored")]
#[macro_export]
macro_rules! core_log {
    ($color:tt, $prefix:expr, $($args:tt)+) => {
        plog::log(plog::macros::colors::$color, $prefix, format!($($args)+))
    }
}

#[cfg(not(feature = "colored"))]
#[macro_export]
macro_rules! core_log {
    ($color:tt, $prefix:tt, $($args:tt)+) => {
        plog::log($prefix, format!($($args)+))
    }
}

/// A debugging logger, just will compile the log message when `debug_assert` is enabled
/// ```rust
/// let x = 5;
/// plog::debug!("x = {x}");
/// ```
#[macro_export]
macro_rules! debug {
    ($($args:tt)+) => {{
        if cfg!(debug_assertions) {
            plog::log!(GREY, "DEBG", $($args)+)
        }
    }}
}

/// An info logger, the lowest importancy level
/// ```rust
/// plog::info!("program initialized");
/// ```
#[macro_export]
macro_rules! info {
    ($($args:tt)+) => {{
        plog::log!(WHITE, "INFO", $($args)+)
    }}
}

/// A warn logger, for things that don't affect the program flow
/// ```rust
/// use std::env;
///
/// if env::var("LOG_FILEPATH").is_err() {
///     plog::warn!("LOG_FILEPATH is disabled")
/// }
/// ```
#[macro_export]
macro_rules! warn {
    ($($args:tt)+) => {{
        plog::log!(YELLOW, "WARN", $($args)+)
    }}
}

/// An error logger, the highest importancy level, for things that will affect the program use
/// ```rust
/// if 1 + 1 != 2 {
///     plog::error!("wait wait wait")
/// }
/// ```
#[macro_export]
macro_rules! error {
    ($($args:tt)+) => {{
        plog::log!(RED, "ERRO", $($args)+)
    }}
}

/// Success logger, for things that is working. The "release version" of `plog::debug!`
/// Also works on `debug_assert`
/// ```rust
/// let x = 5;
/// plog::ok!("x was initialized as {x}");
/// ```
#[macro_export]
macro_rules! ok {
    ($($args:tt)+) => {
        plog::log!(GREEN, "OKAY", $($args)+)
    }
}
