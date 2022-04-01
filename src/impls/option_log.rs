use crate as plog;
use crate::{info, ok, warn};
use std::fmt::{Debug, Display};

pub trait OptionLog<T> {
    fn log(self, _: T) -> Self;
    fn show_none(self, _: T) -> Self;
    fn show_some(self, _: T) -> Self;
}

impl<T: Debug, U: Display> OptionLog<U> for Option<T> {
    fn log(self, name: U) -> Self {
        match self {
            Some(ref x) => ok!("{name} has {x:?}"),
            None => warn!("{name} is empty"),
        };
        self
    }

    fn show_none(self, name: U) -> Self {
        if let None = self {
            info!("{name} is empty");
        }
        self
    }

    fn show_some(self, name: U) -> Self {
        if let Some(ref x) = self {
            info!("{name} has {x:?}");
        }
        self
    }
}
