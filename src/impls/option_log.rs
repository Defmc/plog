//! Plog implementation for Option
//! defines some methods to work with `plog` using `std::option::Option` wrapper
//! Due to adhension to method chains, every method consumes and return the ownership
//! like `Option::map`, `Option::and_then`, etc.

use crate as plog;
use crate::{info, ok, warn};
use std::fmt::{Debug, Display};

/// Methods to work with. Where `log = show_none + show_some`
pub trait OptionLog<T> {
    /// show the content of a `Option` in a `plog::ok!` log or
    /// a "{name} is empty" message in `plog::warn!` when it's None
    fn log(self, _: T) -> Self;

    /// just show when `Option::is_none` return true
    fn show_none(self, _: T) -> Self;

    /// just show when `Option::is_some` returns true
    fn show_some(self, _: T) -> Self;
}

impl<T: Debug, U: Display> OptionLog<U> for Option<T> {
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
    fn log(self, name: U) -> Self {
        match self {
            Some(ref x) => ok!("{name} has {x:?}"),
            None => warn!("{name} is empty"),
        };
        self
    }

    #[cfg(not(feature = "impls"))]
    fn log(self, _name: U) -> Self {
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
    fn show_none(self, name: U) -> Self {
        if let None = self {
            info!("{name} is empty");
        }
        self
    }
    #[cfg(not(feature = "impls"))]
    fn show_none(self, _name: U) -> Self {
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
    fn show_some(self, name: U) -> Self {
        if let Some(ref x) = self {
            info!("{name} has {x:?}");
        }
        self
    }

    #[cfg(not(feature = "impls"))]
    fn show_some(self, _name: U) -> Self {
        self
    }
}
