#[cfg(feature = "impls")]
use crate::impls::*;
use crate::{self as plog, error, info, ok, warn};
use std::{thread, time::Duration};
use tokio::time;

#[tokio::test]
async fn r#async() {
    info!("Application started");
    let jobs: Vec<_> = (0..=10)
        .map(|x| tokio::spawn(async move {
            ok!("{x}th thread initialized");
            time::sleep(Duration::from_millis(250)).await;
            ok!("{x}th thread finalized");
        })).collect();
    for job in jobs.into_iter() {
        job.await.unwrap();
    }
}

#[test]
fn multithread() {
    info!("Application started");
    let thrds: Vec<_> = (0..=10)
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

#[cfg(feature = "impls")]
#[test]
fn result() {
    #[allow(unused_must_use)]
    let exec = |name: &str, item: &Result<u8, ()>| {
        item.log(name);
        item.show_ok(name);
        item.show_err(name);
    };

    let n: Result<u8, ()> = Ok(2);
    let m: Result<u8, ()> = Err(());
    exec("n", &n);
    exec("m", &m);
}

#[cfg(feature = "impls")]
#[test]
fn option() {
    let exec = |name: &str, item: &Option<u8>| {
        item.log(name);
        item.show_some(name);
        item.show_none(name);
    };

    let n: Option<u8> = Some(2);
    let m: Option<u8> = None;
    exec("n", &n);
    exec("m", &m);
}
