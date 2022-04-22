//! Plog implementation for Result
//! defines some methods to work with `plog` using `std::result::Result` wrapper
//! Due to adhension to method chains, every method consumes and return the ownership
//! like `Result::map`, `Result::inspect`, etc.

use crate::{self as plog, error, ok};
use std::fmt::{Debug, Display};

/// Methods to work with. Where `log = show_ok + show_err`
pub trait ResultLog<T> {
    /// Show the content of a `Result` in a `plog::ok!` log or
    /// a "{name} was failed with " message in `plog::error!` when it's `Result::Err`
    fn log(self, _: T) -> Self;

    /// Just shows when `Result::is_ok` returns true
    fn show_ok(self, _: T) -> Self;

    /// Just shows when `Result::is_err` returns true
    fn show_err(self, _: T) -> Self;
}

impl<T, U, N> ResultLog<N> for Result<T, U>
where
    T: Debug,
    U: Debug,
    N: Display,
{
    /// Default implementation. Shows "{name} returned {ok:?}" for `Result::Ok`
    /// and "{name} was failed with {err:?}" for `Result::Err`
    /// ```rust
    /// use plog::impls::ResultLog;
    /// type Res = Result<u8, ()>;
    ///
    ///
    /// let opt_err: Res = Err(());
    /// let opt_ok: Res = Ok(0);
    /// opt_ok.log("opt_error"); // Logs "[ERRO]: opt_error was failed with ()"
    /// opt_err.log("opt_ok"); // Logs "[OKAY]: opt_ok succeed 0"
    /// ```
    fn log(self, _name: N) -> Self {
        #[cfg(feature = "impls")]
        match self {
            Ok(ref val) => ok!("{name} succeed {val:?}"),
            Err(ref err) => error!("{name} was failed with {err:?}"),
        }
        self
    }

    /// Like `ResultLog::log`, but just shows for `Option::None`
    /// ```rust
    /// use plog::impls::ResultLog;
    /// type Res = Result<u8, ()>;
    ///
    /// let opt_err: Res = Err(());
    /// opt_err.log("opt_err"); // Logs "[ERRO]: opt_err was failed with ()"
    /// ```
    fn show_err(self, _name: N) -> Self {
        #[cfg(feature = "impls")]
        if let Err(ref err) = self {
            error!("{name} was failed with {err:?}");
        }
        self
    }

    /// Like `ResultLog::log`, but just shows for `Option::Some`
    /// ```rust
    /// use plog::impls::ResultLog;
    /// type Res = Result<u8, ()>;
    ///
    /// let opt_ok = OK(0);
    /// opt_ok.log("opt_ok"); // Logs "[OKAY]: opt_ok succeed with "Hello world""
    /// ```
    fn show_ok(self, _name: N) -> Self {
        #[cfg(feature = "impls")]
        if let Ok(ref val) = self {
            ok!("{name} succeed with {val:?}");
        }
        self
    }
}
