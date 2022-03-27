use crate as plog;
use crate::{info, ok, warn};
use std::fmt::Debug;

pub trait OptionLog {
    fn log(self) -> Self;
    fn show_none(self) -> Self;
    fn show_some(self) -> Self;
}

impl<T: Debug> OptionLog for Option<T> {
    fn log(self) -> Self {
        match self {
            Some(ref x) => ok!("obtained {x:?}"),
            None => warn!("nothing obtained"),
        };
        self
    }

    fn show_none(self) -> Self {
        if let None = self {
            info!("contains nothing");
        }
        self
    }

    fn show_some(self) -> Self {
        if let Some(ref x) = self {
            info!("contains {x:?}");
        }
        self
    }
}
