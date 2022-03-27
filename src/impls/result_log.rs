use crate::{self as plog, error, ok};
use std::fmt::Debug;

pub trait ResultLog {
    fn log(self) -> Self;
}

pub trait ShowOk {
    fn show_ok(self) -> Self;
}

pub trait ShowErr {
    fn show_err(self) -> Self;
}

impl<T: Debug, U: Debug> ResultLog for Result<T, U> {
    fn log(self) -> Self {
        match self {
            Ok(ref val) => ok!("obtained {val:?}"),
            Err(ref err) => error!("obtained {err:?}"),
        }
        self
    }
}

impl<T, U: Debug> ShowErr for Result<T, U> {
    fn show_err(self) -> Self {
        if let Err(ref err) = self {
            error!("contains {err:?}");
        }
        self
    }
}

impl<T: Debug, U> ShowOk for Result<T, U> {
    fn show_ok(self) -> Self {
        if let Ok(ref val) = self {
            ok!("contains {val:?}");
        }
        self
    }
}
