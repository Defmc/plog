use plog::impls::*;
use std::fmt::Debug;

fn main() {
    const LOREM: &str = "Lorem ipsum";
    const EMPTY_OPT: Option<&str> = None;
    const FILLD_OPT: Option<&str> = Some(LOREM);
    const EMPTY_RES: Result<&str, ()> = Ok(LOREM);
    const FILLD_RES: Result<&str, ()> = Err(());

    test(&EMPTY_OPT, &EMPTY_RES);
    test(&FILLD_OPT, &FILLD_RES);
}

#[allow(unused_must_use)]
fn test(opt: &Option<impl Debug>, res: &Result<impl Debug, impl Debug>) {
    opt.as_ref().log();
    res.as_ref().log();
}
