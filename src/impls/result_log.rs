use crate::{self as plog, error, ok};
use std::fmt::{Debug, Display};

pub trait ResultLog<T> {
    fn log(self, _: T) -> Self;
    fn show_ok(self, _: T) -> Self;
    fn show_err(self, _: T) -> Self;
}

impl<T, U, N> ResultLog<N> for Result<T, U> where
    T: Debug, U: Debug, N: Display {
    fn log(self, name: N) -> Self {
        match self {
            Ok(ref val) => ok!("{name} returned {val:?}"),
            Err(ref err) => error!("{name} was failed with {err:?}"),
        }
        self
    }

    fn show_err(self, name: N) -> Self {
        if let Err(ref err) = self {
            error!("{name} was failed with {err:?}");
        }
        self
    }

    fn show_ok(self, name: N) -> Self {
        if let Ok(ref val) = self {
            ok!("{name} was failed with {val:?}");
        }
        self
    }
}
