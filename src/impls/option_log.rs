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
    /// a "{name} is empty" message in `plog::warn!` when it's None
    fn log(self, _: T) -> Self;

    /// Just shows when it's `Option::is_none` return true
    fn show_none(self, _: T) -> Self;

    /// Just shows when it's `Option::is_some` returns true
    fn show_some(self, _: T) -> Self;

    /// Debug log variant.
    fn debug(self, _: T) -> Self;
    
    /// Debug `show_none` variant.
    fn show_none(self, _: T) -> Self;

    /// Debug `show_some` variant.
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
    /// opt_none.log("none"); // Logs "[WARN]: none is empty"
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
    /// use plog::impls::*;
    ///
    /// let opt_hello = Some("Hello world");
    /// opt_hello.log("hello"); // Logs "[OKAY]: hello has "Hello world""
    /// ```
    fn show_some(self, _name: U) -> Self {
        #[cfg(feature = "impls")]
        if let Some(ref x) = self {
            info!("{_name} has {x:?}");
        }
        self
    }

    /// Like `OptionLog::log`, but just shows on debug builds
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_none: Option<&str> = None;
    /// let opt_hello = Some("Hello world");
    /// opt_none.debug("none"); // Just logs "[WARN]: none is empty" on debug builds
    /// opt_hello.debug("hello"); // Just logs "[OKAY]: hello has "Hello world"" on debug builds
    /// ```
    fn debug(self, _name: U) -> Self {
        #[cfg(debug_assertions)]
        let self = self.debug(_name);
        self
    }

     /// Like `OptionLog::show_none`, but just shows on debug builds
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_none: Option<&str> = None;
    /// opt_none.log("none"); // Logs "[WARN]: none is empty"
    /// ```
    fn show_none(self, _name: U) -> Self {
        #[cfg(debug_assertions)]
        let self = self.show_none(_name);
        self
    }

    /// Like `OptionLog::show_some`, but just shows on debug builds
    /// ```rust
    /// use plog::impls::*;
    ///
    /// let opt_hello = Some("Hello world");
    /// opt_hello.log("hello"); // Logs "[OKAY]: hello has "Hello world""
    /// ```
    fn debug_some(self, _name: U) -> Self {
        #[cfg(debug_assertions)]
        let self = self.show_some(_name);
        self
    }
}
