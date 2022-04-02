#[cfg(feature = "impls")]
fn main() {
    use plog::impls::*;
    use std::fmt::Debug;

    const LOREM: &str = "Lorem ipsum";
    const EMPTY_OPT: Option<&str> = None;
    const FILLD_OPT: Option<&str> = Some(LOREM);
    const EMPTY_RES: Result<&str, ()> = Ok(LOREM);
    const FILLD_RES: Result<&str, ()> = Err(());

    test(&EMPTY_OPT, &EMPTY_RES);
    test(&FILLD_OPT, &FILLD_RES);

    #[allow(unused_must_use)]
    #[cfg(feature = "impls")]
    fn test(opt: &Option<impl Debug>, res: &Result<impl Debug, impl Debug>) {
        opt.as_ref().log("opt");
        res.as_ref().log("res");
    }
}

#[cfg(not(feature = "impls"))]
fn main() {}
