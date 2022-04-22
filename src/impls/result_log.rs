//! Plog implementation for Result
//! defines some methods to work with `plog` using `std::result::Result` wrapper
//! Due to adhension to method chains, every method consumes and return the ownership
//! like `Result::map`, `Result::inspect`, etc.

use crate::{self as plog, error, ok};
use std::fmt::{Debug, Display};

/// Methods to work with. Where `log = show_ok + show_err`
pub trait ResultLog<T> {
    /// show the content of a `Option` in a `plog::ok!` log or
    /// a "{name} is empty" message in `plog::warn!` when it's None
    fn log(self, _: T) -> Self;

    /// just show when `Option::is_none` return true
    fn show_ok(self, _: T) -> Self;

    /// just show when `Option::is_some` returns true
    fn show_err(self, _: T) -> Self;
}

impl<T, U, N> ResultLog<N> for Result<T, U>
where
    T: Debug,
    U: Debug,
    N: Display,
{
    /// Default implementation. Shows "{name} has {item:?}" for `Option::Some`
    /// and "{name} is empty" for `Option::None`
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_none: Option<&str> = None;
    /// let opt_hello = Some("Hello world");
    /// opt_none.log("none"); // Logs "[WARN]: none is empty"
    /// opt_hello.log("hello"); // Logs "[OKAY]: hello has "Hello world""
    /// ```
    #[cfg(feature = "impls")]
    fn log(self, name: N) -> Self {
        match self {
            Ok(ref val) => ok!("{name} returned {val:?}"),
            Err(ref err) => error!("{name} was failed with {err:?}"),
        }
        self
    }

    #[cfg(not(feature = "impls"))]
    fn log(self, _name: N) -> Self {
        self
    }

    /// Like `OptionLog::log`, but just works for `Option::None`
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_none: Option<&str> = None;
    /// opt_none.log("none"); // Logs "[WARN]: none is empty"
    /// ```
    #[cfg(feature = "impls")]
    fn show_err(self, name: N) -> Self {
        if let Err(ref err) = self {
            error!("{name} was failed with {err:?}");
        }
        self
    }

    #[cfg(not(feature = "impls"))]
    fn show_err(self, _name: N) -> Self {
        self
    }

    /// Like `OptionLog::log`, but just works for `Option::Some`
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_hello = Some("Hello world");
    /// opt_hello.log("hello"); // Logs "[OKAY]: hello has "Hello world""
    /// ```
    #[cfg(feature = "impls")]
    fn show_ok(self, name: N) -> Self {
        if let Ok(ref val) = self {
            ok!("{name} was failed with {val:?}");
        }
        self
    }

    #[cfg(not(feature = "impls"))]
    fn show_ok(self, _name: N) -> Self {
        self
    }
}
