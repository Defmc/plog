//! Plog implementation for Option
//! defines some methods to work with `plog` using `std::option::Option` wrapper
//! Due to adhension to method chains, every method consumes and return the ownership
//! like `Option::map`, `Option::and_then`, etc.

use crate as plog;
use crate::{info, ok, warn};
use std::fmt::{Debug, Display};

/// Methods to work with. Where `log = show_none + show_some`
pub trait OptionLog<T> {
    /// Show the content of a `Option` in a `plog::ok!` log or
    /// a "{name} is empty" message in `plog::warn!` when it's `Option::None`
    fn log(self, _: T) -> Self;

    /// Just shows when it's `Option::is_none` return true
    fn show_none(self, _: T) -> Self;

    /// Just shows when it's `Option::is_some` returns true
    fn show_some(self, _: T) -> Self;
}

impl<T: Debug, U: Display> OptionLog<U> for Option<T> {
    /// Default implementation. Shows "{name} has {item:?}" for `Option::Some`
    /// and "{name} is empty" for `Option::None`
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_none: Option<&str> = None;
    /// let opt_some = Some("Hello world");
    /// opt_none.log("opt_none"); // Logs "[WARN]: opt_none is empty"
    /// opt_some.log("opt_some"); // Logs "[OKAY]: opt_some has "Hello world""
    /// ```
    fn log(self, _name: U) -> Self {
        #[cfg(feature = "impls")]
        match self {
            Some(ref x) => ok!("{_name} has {x:?}"),
            None => warn!("{_name} is empty"),
        };
        self
    }

    /// Like `OptionLog::log`, but just shows for `Option::None`
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_none: Option<&str> = None;
    /// opt_none.log("opt_none"); // Logs "[WARN]: opt_none is empty"
    /// ```
    fn show_none(self, _name: U) -> Self {
        #[cfg(feature = "impls")]
        if let None = self {
            info!("{_name} is empty");
        }
        self
    }

    /// Like `OptionLog::log`, but just shows for `Option::Some`
    /// ```rust
    /// use plog::impls::OptionLog;
    ///
    /// let opt_some = Some("Hello world");
    /// opt_some.log("opt_some"); // Logs "[OKAY]: opt_some has "Hello world""
    /// ```
    fn show_some(self, _name: U) -> Self {
        #[cfg(feature = "impls")]
        if let Some(ref x) = self {
            info!("{_name} has {x:?}");
        }
        self
    }
}
