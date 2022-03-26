use crate::{self as plog, error, info, ok, warn};
use std::thread;

#[test]
fn multithread() {
    info!("Application started");
    let thrds: Vec<_> = (0..=1000)
        .map(|x| {
            thread::spawn(move || {
                ok!("{x}th thread initialized");
                ok!("{x}th thread finalized");
            })
        })
        .collect();

    thrds
        .into_iter()
        .for_each(|thrd| assert!(thrd.join().is_ok()));
}

#[test]
fn pretty_out() {
    let n = 5;
    ok!("it's working {n}");
    info!("it's started to work {n}");
    warn!("wait, is it really working {n}?");
    error!("OHHHH NOOOO");
}
