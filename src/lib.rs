//! ## A modular pretty logger for Rust
//! Created with the philosophy to be minimum but extensible
//! everything (except core) is a feature, every configuration is defined
//! in compiler-level to minimal run-time overhead.
//! # Usage
//! Simplest as possible, works like a `std::println!` (more exatcly as a `std::eprintln!`):
//! ```rust
//! use plog::{info, ok};
//! use std::{thread, time::Duration};
//!
//! let threads: Vec<_> = (0..=10)
//!     .map(|id| {
//!         info!("Creating thread {id}");
//!         thread::spawn(move || {
//!             thread::sleep(Duration::from_millis(1000));
//!             ok!("Thread {id} terminated");
//!         })
//!     })
//!     .collect();
//!     
//! threads.into_iter().for_each(|thr| thr.join().unwrap());
//! ```
//! # Features
//! All the available features are listed [here](https://github.com/Defmc/plog/wiki/Features)

pub mod macros;
use std::io;

#[cfg(feature = "colored")]
use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};

#[cfg(feature = "persistent")]
pub mod persistent;

#[cfg(feature = "impls")]
pub mod impls {
    pub mod option_log;
    pub use option_log::*;
    pub mod result_log;
    pub use result_log::*;
}

#[cfg(test)]
pub mod test;

/// Log handler
/// Checks the enabled features and write to specified streams
/// `persistent` enable write to a file
/// `colored` enable colored output to terminal
pub fn log<T: AsRef<str>>(
    #[cfg(feature = "colored")] color: Color,
    prefix: &str,
    msg: T,
) -> io::Result<()> {
    print_log(
        #[cfg(feature = "colored")]
        color,
        prefix,
        &msg,
    )?;

    #[cfg(feature = "persistent")]
    if persistent::check_env() {
        persistent::write_log(prefix, &msg)?;
    }
    Ok(())
}

/// Prints log to STDERR
/// `colored` enable colored output to terminal
#[cfg(feature = "colored")]
fn print_log<T: AsRef<str>>(
    #[cfg(feature = "colored")] color: Color,
    prefix: &str,
    msg: &T,
) -> io::Result<()> {
    execute!(
        io::stderr().lock(),
        Print("["),
        SetAttribute(Attribute::Bold),
        SetForegroundColor(color),
        Print(prefix),
        ResetColor,
        Print("]: "),
        Print(msg.as_ref()),
        Print('\n')
    )
}

#[cfg(not(feature = "colored"))]
fn print_log<T: AsRef<str>>(prefix: &str, msg: T) -> io::Result<()> {
    use std::io::Write;
    writeln!(io::stderr().lock(), "[{prefix}]: {}", msg.as_ref()).map(|_| ())
}
