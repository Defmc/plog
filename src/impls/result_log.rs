use crate::{error, ok, self as plog};
use std::fmt::Debug;

pub trait ResultLog {
    fn log(&self);
}

pub trait ShowOk {
    fn show_ok(&self);
}

pub trait ShowErr {
    fn show_err(&self);
}

impl<T: Debug, U: Debug> ResultLog for Result<T, U> {
    fn log(&self) {
        match self {
            Ok(val) => ok!("obtained {val:?}"),
            Err(err) => error!("obtained {err:?}")
        }
    }
}

impl <T, U: Debug> ShowErr for Result<T, U> {
    fn show_err(&self) {
        if let Err(err) = self {
            error!("contains {err:?}");
        }
    }
}

impl <T: Debug, U> ShowOk for Result<T, U> {
    fn show_ok(&self) {
        if let Ok(val) = self {
            ok!("contains {val:?}");
        }
    }
}

