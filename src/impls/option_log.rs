use crate::{warn, info, ok};
use crate as plog;
use std::fmt::Debug;

pub trait OptionLog {
    fn log(&self);
    fn show_none(&self);
    fn show_some(&self);
}

impl<T: Debug> OptionLog for Option<T> {
    fn log(&self) {
        match self {
            Some(x) => ok!("obtained {x:?}"),
            None => warn!("nothing obtained")
        };
    }

    fn show_none(&self) {
        if let None = self {
            info!("contains nothing");
        }
    }

    fn show_some(&self) {
        if let Some(x) = self {
            info!("contains {x:?}");
        }
    }
}
